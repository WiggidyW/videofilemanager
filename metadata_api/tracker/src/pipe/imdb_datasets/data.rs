use derive_more::{Error, Display};

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

#[derive(Debug)]
pub struct Rows {
    pub inner: bytes::Bytes,
    kind: DatasetKind,
}

#[derive(Debug)]
pub enum Row<'a> {
    TitlePrincipals { // tconst, ordering, nconst, category, job, characters
        imdb_id: u32, // n dupes
        ordering: u32,
        name_id: u32,
        category: &'a str,
        job: Option<&'a str>,
        characters: Option<&'a str>,
    },
    NameBasics { // nconst, primaryName, birthYear, deathYear, primaryProfession, knownForTitles
        name_id: u32,
        name: &'a str,
        birth_year: Option<u32>,
        death_year: Option<u32>,
        primary_profession: Option<Vec<&'a str>>,
        imdb_ids: Option<Vec<u32>>,
    },
    TitleAkas { // titleId, ordering, title, region, language, types, attributes, isOriginalTitle
        imdb_id: u32, // ~n/2 dupes
        ordering: u32,
        title: Option<&'a str>,
        region: Option<&'a str>,
        language: Option<&'a str>,
        types: Option<&'a str>,
        attributes: Option<&'a str>,
        is_original_title: Option<bool>,
    },
    TitleBasics { // tconst, titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres
        imdb_id: u32, // 0 dupes
        title_type: &'a str,
        primary_title: Option<&'a str>,
        original_title: Option<&'a str>,
        is_adult: bool,
        start_year: Option<u32>,
        end_year: Option<u32>,
        runtime_minutes: Option<u32>,
        genres: Option<Vec<&'a str>>,
    },
    TitleCrew { // tconst, directors, writers
        imdb_id: u32, // 0 dupes
        directors: Option<Vec<u32>>,
        writers: Option<Vec<u32>>,
    },
    TitleEpisode { // tconst, parentTconst, seasonNumber, episodeNumber
        imdb_id: u32, // 0 dupes
        series_id: u32,
        season_number: Option<u32>,
        episode_number: Option<u32>,
    },
    TitleRatings { // tconst, averageRating, numVotes
        imdb_id: u32, // 0 dupes
        average_rating: f32,
        num_votes: u32,
    },
}

pub struct TitleInfo {
    imdb_id: u32,
    title_type: Option<String>,
    primary_title: Option<String>,
    original_title: Option<String>,
    is_adult: bool,
    start_year: Option<u32>,
    end_year: Option<u32>,
    runtime_minutes: Option<u32>,
    genres: Option<Vec<String>>,
    average_rating: Option<f32>,
    num_votes: Option<u32>,
    series_id: Option<u32>,
    season_number: Option<u32>,
    episode_number: Option<u32>,
    episodes: Option<Vec<TitleInfoEpisode>>,
    titles: Option<Vec<TitleInfoTitle>>,
    people: Option<Vec<TitleInfoPerson>>,
}
pub struct TitleInfoEpisode {
    imdb_id: u32,
    season_number: Option<u32>,
    episode_number: Option<u32>,
}
pub struct TitleInfoTitle {
    title: String,
    region: Option<String>,
    language: Option<String>,
    types: Option<String>,
    attributes: Option<String>,
    is_original_title: Option<bool>,
}
pub struct TitleInfoPerson {
    name_id: u32,
    name: Option<String>,
    category: Option<String>,
    job: Option<String>,
    characters: Option<String>,
    writer: bool,
    director: bool,
}

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "The following bytes were found to be invalid UTF-8:\n'{:?}'", chunk)]
    Utf8Error {
        source: std::str::Utf8Error,
        chunk: bytes::Bytes,
    },
    #[display(fmt = "The following value was an invalid integer: '{}'\nRow: '{:?}'", value, row)]
    StrToIntError {
        source: <u32 as std::str::FromStr>::Err,
        value: String,
        row: Vec<String>,
    },
    #[display(fmt = "The following value was an invalid float: '{}'\nRow: '{:?}'", value, row)]
    StrToFloatError {
        source: <f32 as std::str::FromStr>::Err,
        value: String,
        row: Vec<String>,
    },
    #[display(fmt = "Error when parsing\nExpected: '{}'\nFound: '{}'\nRow: '{:?}'", expected, value, row)]
    InvalidField {
        value: String,
        row: Vec<String>,
        expected: &'static str,
    },
    #[display(fmt =
        "The row of kind '{:?}' was an invalid length.\nExpected: '{}'\nFound: '{}'\nRow: '{:?}'",
        kind, expected, found, row,
    )]
    RowLengthError {
        kind: DatasetKind,
        row: Vec<String>,
        expected: usize,
        found: usize,
    },
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
}

impl Rows {
    pub(super) fn new(bytes: bytes::Bytes, kind: DatasetKind) -> Self {
        Self {
            inner: bytes,
            kind: kind,
        }
    }
    fn try_iter<'a>(
        &'a self
    ) -> Result<impl Iterator<Item = impl Iterator<Item = &'a str>>, std::str::Utf8Error> {
        Ok(
            std::str::from_utf8(&self.inner)?
                .split('\n')
                .filter(|&str| str != "") // filter out empty rows (is expected!)
                .map(|row| row.split('\t'))
        )
    }
}

impl<'a> std::convert::TryFrom<&'a Rows> for Vec<Row<'a>> {
    type Error = Error;
    fn try_from(value: &'a Rows) -> Result<Self, Self::Error> {
        value.try_iter()
            .map_err(|e| Error::Utf8Error {
                source: e,
                chunk: value.inner.clone(),
            })?
            .map(|row| Row::try_from_iter(row, value.kind))
            .collect()
    }
}

impl<'a> Row<'a> {
    fn try_from_iter(
        iter: impl Iterator<Item = &'a str>,
        kind: DatasetKind,
    ) -> Result<Self, Error> {
        let row: Vec<&'a str> = iter.collect();
        let assert_len = |i: usize| -> Result<(), Error> {
            match row.len() {
                len if len == i => Ok(()),
                _ => Err(Error::RowLengthError {
                    kind: kind,
                    found: row.len(),
                    row: row.iter().map(|s| s.to_string()).collect(),
                    expected: i,
                }),
            }
        };
        let map_none = |s: &'a str| -> Option<&'a str> {
            match s {
                "\\N" => None,
                "" => None,
                s if s.chars().all(|c| c == ' ') => None,
                s => Some(s),
            }
        };
        let map_int = |s: &'a str| -> Result<u32, Error> {
            s.parse().map_err(|e| Error::StrToIntError {
                source: e,
                value: s.to_string(),
                row: row.iter().map(|s| s.to_string()).collect(),
            })
        };
        let map_id = |s: &'a str| -> Result<u32, Error> {
            match s.get(2..) {
                Some(s) => map_int(s),
                None => Err(Error::InvalidField {
                    value: s.to_string(),
                    row: row.iter().map(|s| s.to_string()).collect(),
                    expected: "a Valid ID",
                })
            }
        };
        let map_float = |s: &'a str| -> Result<f32, Error> {
            s.parse().map_err(|e| Error::StrToFloatError {
                source: e,
                value: s.to_string(),
                row: row.iter().map(|s| s.to_string()).collect(),
            })
        };
        let map_bool = |s: &'a str| -> Result<bool, Error> {
            match s {
                "0" => Ok(false),
                "1" => Ok(true),
                _ => Err(Error::InvalidField {
                    value: s.to_string(),
                    row: row.iter().map(|s| s.to_string()).collect(),
                    expected: "a bool: either 1 or 0",
                })
            }
        };
        match kind {
            DatasetKind::TitlePrincipals => {
                assert_len(6)?;
                Ok(Self::TitlePrincipals {
                    imdb_id: map_id(row[0])?,
                    ordering: map_int(row[1])?,
                    name_id: map_id(row[2])?,
                    category: row[3],
                    job: map_none(row[4]),
                    characters: map_none(row[5]),
                })
            },
            DatasetKind::NameBasics => {
                assert_len(6)?;
                Ok(Self::NameBasics {
                    name_id: map_id(row[0])?,
                    name: row[1],
                    birth_year: map_none(row[2]).map(|s| map_int(s)).transpose()?,
                    death_year: map_none(row[3]).map(|s| map_int(s)).transpose()?,
                    primary_profession: map_none(row[4]).map(|s| s.split(',').collect()),
                    imdb_ids: map_none(row[5]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
                })
            },
            DatasetKind::TitleAkas => {
                assert_len(8)?;
                Ok(Self::TitleAkas {
                    imdb_id: map_id(row[0])?,
                    ordering: map_int(row[1])?,
                    title: map_none(row[2]),
                    region: map_none(row[3]),
                    language: map_none(row[4]),
                    types: map_none(row[5]),
                    attributes: map_none(row[6]),
                    is_original_title: map_none(row[7]).map(|s| map_bool(s)).transpose()?,
                })
            },
            DatasetKind::TitleBasics => {
                assert_len(9)?;
                Ok(Self::TitleBasics {
                    imdb_id: map_id(row[0])?,
                    title_type: row[1],
                    primary_title: map_none(row[2]),
                    original_title: map_none(row[3]),
                    is_adult: map_bool(row[4])?,
                    start_year: map_none(row[5]).map(|s| map_int(s)).transpose()?,
                    end_year: map_none(row[6]).map(|s| map_int(s)).transpose()?,
                    runtime_minutes: map_none(row[7]).map(|s| map_int(s)).transpose()?,
                    genres: map_none(row[8]).map(|s| s.split(',').collect()),
                })
            },
            DatasetKind::TitleCrew => {
                assert_len(3)?;
                Ok(Self::TitleCrew {
                    imdb_id: map_id(row[0])?,
                    directors: map_none(row[1]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
                    writers: map_none(row[2]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
                })
            },
            DatasetKind::TitleEpisode => {
                assert_len(4)?;
                Ok(Self::TitleEpisode {
                    imdb_id: map_id(row[0])?,
                    series_id: map_id(row[1])?,
                    season_number: map_none(row[2]).map(|s| map_int(s)).transpose()?,
                    episode_number: map_none(row[3]).map(|s| map_int(s)).transpose()?,
                })
            },
            DatasetKind::TitleRatings => {
                assert_len(3)?;
                Ok(Self::TitleRatings {
                    imdb_id: map_id(row[0])?,
                    average_rating: map_float(row[1])?,
                    num_votes: map_int(row[2])?,
                })
            },
        }
    }
}