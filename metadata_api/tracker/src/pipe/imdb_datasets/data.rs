use derive_more::{Error, Display, From};
use bytes::{Bytes, BytesMut};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DatasetKind {
    TitlePrincipals,
    NameBasics,
    TitleAkas,
    TitleBasics,
    TitleCrew,
    TitleEpisode,
    TitleRatings,
}

#[derive(Debug, From)]
pub struct Chunk {
    pub bytes: Bytes,
    pub kind: DatasetKind,
}

#[derive(Debug, Default, Clone)]
pub struct ChunkExtra {
    pub columns: Vec<Bytes>,
    pub part_column: Option<Bytes>,
    pub expected_len: usize,
}

#[derive(Debug)]
// This will always have the correct number of rows!
pub struct ChunkRow {
    pub bytes: Vec<Bytes>,
    pub kind: DatasetKind,
}

#[derive(Debug)]
pub enum Row<'a> {
    TitlePrincipals { // tconst, ordering, nconst, category, job, characters
        imdb_id: i32, // n dupes
        ordering: i32,
        name_id: i32,
        category: &'a str,
        job: Option<&'a str>,
        characters: Option<&'a str>,
    },
    NameBasics { // nconst, primaryName, birthYear, deathYear, primaryProfession, knownForTitles
        name_id: i32,
        name: &'a str,
        birth_year: Option<i32>,
        death_year: Option<i32>,
        primary_profession: Option<Vec<&'a str>>,
        imdb_ids: Option<Vec<i32>>,
    },
    TitleAkas { // titleId, ordering, title, region, language, types, attributes, isOriginalTitle
        imdb_id: i32, // ~n/2 dupes
        ordering: i32,
        title: Option<&'a str>,
        region: Option<&'a str>,
        language: Option<&'a str>,
        types: Option<&'a str>,
        attributes: Option<&'a str>,
        is_original_title: Option<bool>,
    },
    TitleBasics { // tconst, titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres
        imdb_id: i32, // 0 dupes
        title_type: &'a str,
        primary_title: Option<&'a str>,
        original_title: Option<&'a str>,
        is_adult: bool,
        start_year: Option<i32>,
        end_year: Option<i32>,
        runtime_minutes: Option<i32>,
        genres: Option<Vec<&'a str>>,
    },
    TitleCrew { // tconst, directors, writers
        imdb_id: i32, // 0 dupes
        directors: Option<Vec<i32>>,
        writers: Option<Vec<i32>>,
    },
    TitleEpisode { // tconst, parentTconst, seasonNumber, episodeNumber
        imdb_id: i32, // 0 dupes
        series_id: i32,
        season_number: Option<i32>,
        episode_number: Option<i32>,
    },
    TitleRatings { // tconst, averageRating, numVotes
        imdb_id: i32, // 0 dupes
        average_rating: f32,
        num_votes: i32,
    },
}

#[derive(Debug, Display, Error)]
pub enum ToRowError {
    #[display(fmt = "Encountered field that was invalid UTF-8\nErr: {}\nValue: '{:?}'\nRow: '{:?}'", source, value, row)]
    Utf8Error {
        source: std::str::Utf8Error,
        value: Bytes,
        row: Vec<Bytes>,
    },
    #[display(fmt = "Encountered invalid field, expected: '{}', found: '{}'\nRow: '{:?}'", expected, value, row)]
    InvalidField {
        expected: &'static str,
        value: String,
        row: Vec<Bytes>,
    }
}

impl DatasetKind {
    pub fn iter() -> impl Iterator<Item = Self> {
        vec![
            DatasetKind::TitlePrincipals,
            DatasetKind::NameBasics,
            DatasetKind::TitleAkas,
            DatasetKind::TitleBasics,
            DatasetKind::TitleCrew,
            DatasetKind::TitleEpisode,
            DatasetKind::TitleRatings,
        ].into_iter()
    }
    pub fn count(&self) -> usize {
        match self {
            DatasetKind::TitlePrincipals => 6,
            DatasetKind::NameBasics => 6,
            DatasetKind::TitleAkas => 8,
            DatasetKind::TitleBasics => 9,
            DatasetKind::TitleCrew => 3,
            DatasetKind::TitleEpisode => 4,
            DatasetKind::TitleRatings => 3,
        }
    }
}

impl Chunk {
    pub fn into_chunk_rows(self, extra: &mut ChunkExtra) -> Vec<ChunkRow> {
        if self.bytes.is_empty() {
            return Vec::new(); // short circuit if self contains nothing
        }
        let mut iter = self.bytes
            .iter()
            .enumerate()
            .filter_map(|(i, b)| match (i, b) {
                (i, b'\t') | (i, b'\n') => Some((i, b)),
                (i, b) if i + 1 == self.bytes.len() => Some((i, b)),
                _ => None,
            });
        let mut index = match iter.next() {
            Some((i, &b)) => {
                let idx = match i + 1 == self.bytes.len() && b != b'\t' && b != b'\n' {
                    true => i + 1,
                    false => i,
                };
                extra.part_column = match &extra.part_column {
                    None => Some(self.bytes.slice(0..idx)),
                    Some(old_b) if old_b.is_empty() => Some(self.bytes.slice(0..idx)),
                    Some(old_b) => {
                        let mut new_b = BytesMut::with_capacity(old_b.len() + i);
                        new_b.extend_from_slice(old_b);
                        new_b.extend_from_slice(&self.bytes.slice(0..idx));
                        Some(new_b.freeze())
                    },
                };
                if idx == self.bytes.len() && b != b'\t' && b != b'\n' {
                    return Vec::new();
                }
                i
            },
            None => unreachable!(),
        };
        let new_columns;
        let mut new_part_column = None;
        let mut chunk_rows: Vec<ChunkRow> = Vec::with_capacity(extra.expected_len);
        let mut iter = extra.columns.drain(..)
            .chain(std::iter::once( {
                let part_column = extra.part_column.as_mut().unwrap();
                let len = part_column.len();
                part_column.split_to(len)
            }))
            .chain(iter
                .filter_map(|(i, b)| match (i, b, index) {
                    (i, b'\t', idx) | (i, b'\n', idx) => {
                        index = i;
                        Some(self.bytes.slice(idx + 1..i))
                    },
                    (i, _, idx) if i + 1 == self.bytes.len() => {
                        new_part_column = Some(self.bytes.slice(idx + 1..i + 1));
                        None
                    },
                    _ => unreachable!(),
                })
            );
        loop {
            let row: Vec<Bytes> = iter
                .by_ref()
                .take(self.kind.count())
                .collect();
            match row.len() == self.kind.count() {
                true => chunk_rows.push(ChunkRow {
                    bytes: row,
                    kind: self.kind,
                }),
                false => {
                    new_columns = row;
                    break;
                },
            }
        }
        std::mem::drop(iter); // makes extra assignable! specifically, columns, since part_column is cloned.
        extra.columns = new_columns;
        extra.part_column = new_part_column;
        extra.expected_len = std::cmp::max(extra.expected_len, chunk_rows.len());
        chunk_rows
    }
}

impl ChunkRow {
    pub fn to_row<'a>(&'a self) -> Result<Row<'a>, ToRowError> {
        let map_none = |s: &'a str| -> Option<&'a str> {
            match s {
                "\\N" => None,
                "" => None,
                s if s.chars().all(|c| c == ' ') => None,
                s => Some(s),
            }
        };
        let map_i32 = |s: &'a str| -> Result<i32, ToRowError> {
            s.parse().map_err(|_| ToRowError::InvalidField {
                expected: "A valid integer",
                value: s.to_string(),
                row: self.bytes.clone(),
            })
        };
        let map_f32 = |s: &'a str| -> Result<f32, ToRowError> {
            s.parse().map_err(|_| ToRowError::InvalidField {
                expected: "A valid float",
                value: s.to_string(),
                row: self.bytes.clone(),
            })
        };
        let map_id = |s: &'a str| -> Result<i32, ToRowError> {
            if let Some(s) = s.get(2..) {
                if let Ok(u) = map_i32(s) {
                    return Ok(u);
                }
            }
            Err(ToRowError::InvalidField {
                expected: "A valid id: 2 characters followed by 7 or more digits",
                value: s.to_string(),
                row: self.bytes.clone(),
            })
        };
        let map_bool = |s: &'a str| -> Result<bool, ToRowError> {
            match s {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(ToRowError::InvalidField {
                    expected: "A bool: either 1 or 0",
                    value: s.to_string(),
                    row: self.bytes.clone(),
                })
            }
        };
        let map_principal_characters = |s: &'a str| -> Result<&'a str, ToRowError> {
            if let Some(s) = s.get(2..s.len() - 1) {
                return Ok(s);
            }
            Err(ToRowError::InvalidField {
                expected: "A slice starting with '[\"' and ending with '\"]",
                value: s.to_string(),
                row: self.bytes.clone(),
            })
        };
        let mut iter = self.bytes
            .iter()
            .map(|b| std::str::from_utf8(b)
                .map_err(|e| ToRowError::Utf8Error {
                    source: e,
                    value: b.clone(),
                    row: self.bytes.clone(),
                })
            );
        match self.kind {
            DatasetKind::TitlePrincipals => Ok(Row::TitlePrincipals {
                imdb_id: map_id(iter.next().unwrap()?)?,
                ordering: map_i32(iter.next().unwrap()?)?,
                name_id: map_id(iter.next().unwrap()?)?,
                category: iter.next().unwrap()?,
                job: map_none(iter.next().unwrap()?),
                characters: map_none(iter.next().unwrap()?)
                    .map(|s| map_principal_characters(s))
                    .transpose()?,
            }),
            DatasetKind::NameBasics => Ok(Row::NameBasics {
                name_id: map_id(iter.next().unwrap()?)?,
                name: iter.next().unwrap()?,
                birth_year: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                death_year: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                primary_profession: map_none(iter.next().unwrap()?)
                    .map(|s| s.split(',').collect()),
                imdb_ids: map_none(iter.next().unwrap()?)
                    .map(|s| s.split(',').map(|id| map_id(id)).collect())
                    .transpose()?,
            }),
            DatasetKind::TitleAkas => Ok(Row::TitleAkas {
                imdb_id: map_id(iter.next().unwrap()?)?,
                ordering: map_i32(iter.next().unwrap()?)?,
                title: map_none(iter.next().unwrap()?),
                region: map_none(iter.next().unwrap()?),
                language: map_none(iter.next().unwrap()?),
                types: map_none(iter.next().unwrap()?),
                attributes: map_none(iter.next().unwrap()?),
                is_original_title: map_none(iter.next().unwrap()?)
                    .map(|s| map_bool(s))
                    .transpose()?,
            }),
            DatasetKind::TitleBasics => Ok(Row::TitleBasics {
                imdb_id: map_id(iter.next().unwrap()?)?,
                title_type: iter.next().unwrap()?,
                primary_title: map_none(iter.next().unwrap()?),
                original_title: map_none(iter.next().unwrap()?),
                is_adult: map_bool(iter.next().unwrap()?)?,
                start_year: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                end_year: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                runtime_minutes: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                genres: map_none(iter.next().unwrap()?)
                    .map(|s| s.split(',').collect()),
            }),
            DatasetKind::TitleCrew => Ok(Row::TitleCrew {
                imdb_id: map_id(iter.next().unwrap()?)?,
                directors: map_none(iter.next().unwrap()?)
                    .map(|s| s.split(',').map(|id| map_id(id)).collect())
                    .transpose()?,
                writers: map_none(iter.next().unwrap()?)
                    .map(|s| s.split(',').map(|id| map_id(id)).collect())
                    .transpose()?,
            }),
            DatasetKind::TitleEpisode => Ok(Row::TitleEpisode {
                imdb_id: map_id(iter.next().unwrap()?)?,
                series_id: map_id(iter.next().unwrap()?)?,
                season_number: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
                episode_number: map_none(iter.next().unwrap()?)
                    .map(|s| map_i32(s))
                    .transpose()?,
            }),
            DatasetKind::TitleRatings => Ok(Row::TitleRatings {
                imdb_id: map_id(iter.next().unwrap()?)?,
                average_rating: map_f32(iter.next().unwrap()?)?,
                num_votes: map_i32(iter.next().unwrap()?)?,
            }),
        }
    }
}

// #[derive(Debug, Default, Clone)]
// pub struct TitleInfo {
//     pub imdb_id: i32,
//     pub title_type: Option<String>,
//     pub primary_title: Option<String>,
//     pub original_title: Option<String>,
//     pub is_adult: Option<bool>,
//     pub start_year: Option<i32>,
//     pub end_year: Option<i32>,
//     pub runtime_minutes: Option<i32>,
//     pub genres: Option<Vec<String>>,
//     pub average_rating: Option<f32>,
//     pub num_votes: Option<i32>,
//     pub series_id: Option<i32>,
//     pub season_number: Option<i32>,
//     pub episode_number: Option<i32>,
//     pub episodes: Option<Vec<TitleInfoEpisode>>,
//     pub titles: Option<Vec<TitleInfoTitle>>,
//     pub people: Option<Vec<TitleInfoPerson>>,
// }
// #[derive(Debug, Default, Clone)]
// pub struct TitleInfoEpisode {
//     pub imdb_id: i32,
//     pub season_number: Option<i32>,
//     pub episode_number: Option<i32>,
// }
// #[derive(Debug, Default, Clone)]
// pub struct TitleInfoTitle {
//     pub title: String,
//     pub ordering: i32,
//     pub region: Option<String>,
//     pub language: Option<String>,
//     pub types: Option<String>,
//     pub attributes: Option<String>,
//     pub is_original_title: Option<bool>,
// }
// #[derive(Debug, Default, Clone)]
// pub struct TitleInfoPerson {
//     pub name_id: i32,
//     pub name: Option<String>,
//     pub birth_year: Option<i32>,
//     pub death_year: Option<i32>,
//     pub categories: Option<Vec<String>>,
//     pub jobs: Option<Vec<String>>,
//     pub characters: Option<Vec<String>>,
//     pub writer: bool,
//     pub director: bool,
// }