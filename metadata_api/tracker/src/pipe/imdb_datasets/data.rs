use derive_more::{Error, Display, From};
use bytes::Bytes;

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

#[derive(Debug, From)]
pub struct ByteRow {
    bytes: Vec<Bytes>,
    kind: DatasetKind,
}

#[derive(Debug, Display, Error)]
#[display(fmt =
    "Error converting Bytes to ByteRow.\n\tBytes: {:?}\n\tKind: {:?}\n\tExpected Length: {}",
    row, kind, expected_len,
)]
pub struct ByteRowError {
    row: Vec<Bytes>,
    kind: DatasetKind,
    expected_len: usize,
}

const SPLIT: u8 = b'\t';

impl ByteRow {
    pub fn new(bytes: Bytes, kind: DatasetKind) -> Result<Self, ByteRowError> {
        let row: Vec<Bytes> = Vec::with_capacity(Self::expected_len(kind));
        let indexes = bytes.iter()
            .enumerate()
            .filter_map(|(i, b)| match b == &SPLIT {
                true => Some(i),
                false => None,
            })
            .peekable();
        if let Some(i) = indexes.peek() {
            row.push(bytes.slice(0..*i));
        }
        for index in indexes {
            match indexes.peek() {
                Some(i) => row.push(bytes.slice(index + 1..*i)),
                None => row.push(bytes.slice(index + 1..bytes.len())),
            };
        }
        match row.len() == Self::expected_len(kind) {
            true => Ok(Self {
                bytes: row,
                kind: kind,
            }),
            false => Err(ByteRowError {
                row: row,
                kind: kind,
                expected_len: Self::expected_len(kind),
            })
        }
    }
    fn expected_len(kind: DatasetKind) -> usize {
        match kind {
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

// #[derive(Debug)]
// pub enum Row<'a> {
//     TitlePrincipals { // tconst, ordering, nconst, category, job, characters
//         imdb_id: i32, // n dupes
//         ordering: i32,
//         name_id: i32,
//         category: &'a str,
//         job: Option<&'a str>,
//         characters: Option<&'a str>,
//     },
//     NameBasics { // nconst, primaryName, birthYear, deathYear, primaryProfession, knownForTitles
//         name_id: i32,
//         name: &'a str,
//         birth_year: Option<i32>,
//         death_year: Option<i32>,
//         primary_profession: Option<Vec<&'a str>>,
//         imdb_ids: Option<Vec<i32>>,
//     },
//     TitleAkas { // titleId, ordering, title, region, language, types, attributes, isOriginalTitle
//         imdb_id: i32, // ~n/2 dupes
//         ordering: i32,
//         title: Option<&'a str>,
//         region: Option<&'a str>,
//         language: Option<&'a str>,
//         types: Option<&'a str>,
//         attributes: Option<&'a str>,
//         is_original_title: Option<bool>,
//     },
//     TitleBasics { // tconst, titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres
//         imdb_id: i32, // 0 dupes
//         title_type: &'a str,
//         primary_title: Option<&'a str>,
//         original_title: Option<&'a str>,
//         is_adult: bool,
//         start_year: Option<i32>,
//         end_year: Option<i32>,
//         runtime_minutes: Option<i32>,
//         genres: Option<Vec<&'a str>>,
//     },
//     TitleCrew { // tconst, directors, writers
//         imdb_id: i32, // 0 dupes
//         directors: Option<Vec<i32>>,
//         writers: Option<Vec<i32>>,
//     },
//     TitleEpisode { // tconst, parentTconst, seasonNumber, episodeNumber
//         imdb_id: i32, // 0 dupes
//         series_id: i32,
//         season_number: Option<i32>,
//         episode_number: Option<i32>,
//     },
//     TitleRatings { // tconst, averageRating, numVotes
//         imdb_id: i32, // 0 dupes
//         average_rating: f32,
//         num_votes: i32,
//     },
// }

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

#[derive(Debug, Display, Error)]
pub enum Error {
    #[display(fmt = "The following bytes were found to be invalid UTF-8:\n'{:?}'", chunk)]
    Utf8Error {
        source: std::str::Utf8Error,
        chunk: bytes::Bytes,
    },
    #[display(fmt = "The following value was an invalid integer: '{}'\nRow: '{:?}'", value, row)]
    StrToIntError {
        source: <i32 as std::str::FromStr>::Err,
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

// impl From<(Bytes, DatasetKind)> for Chunk {
//     fn from(v: (Bytes, DatasetKind)) -> Self {
//         match v.1 {
//             DatasetKind::TitlePrincipals => Self::TitlePrincipals(v.0),
//             DatasetKind::NameBasics => Self::NameBasics(v.0),
//             DatasetKind::TitleAkas => Self::TitleAkas(v.0),
//             DatasetKind::TitleBasics => Self::TitleBasics(v.0),
//             DatasetKind::TitleCrew => Self::TitleCrew(v.0),
//             DatasetKind::TitleEpisode => Self::TitleEpisode(v.0),
//             DatasetKind::TitleRatings => Self::TitleRatings(v.0),
//         }
//     }
// }

// impl Rows {
//     pub(super) fn new(bytes: bytes::Bytes, kind: DatasetKind) -> Self {
//         Self {
//             inner: bytes,
//             kind: kind,
//         }
//     }
//     pub fn try_iter<'a>(&'a self) -> Result<impl Iterator<Item = Result<Row<'a>, Error>>, Error> {
//         Ok(RowsIter {
//             inner: std::str::from_utf8(&self.inner)
//                 .map_err(|e| Error::Utf8Error {
//                     source: e,
//                     chunk: self.inner.clone(),
//                 })?
//                 .split('\n')
//                 .filter(|&str| str != "")
//                 .map(|row| row.split('\t')),
//             kind: self.kind,
//         })
//     }
// }

// impl<'a, T: Iterator<Item = impl Iterator<Item = &'a str>>> Iterator for RowsIter<T> {
//     type Item = Result<Row<'a>, Error>;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.inner.next() {
//             Some(iter) => Some(Row::try_from_iter(iter, self.kind)),
//             None => None,
//         }
//     }
// }

// impl<'a> Row<'a> {
//     fn try_from_iter(
//         iter: impl Iterator<Item = &'a str>,
//         kind: DatasetKind,
//     ) -> Result<Self, Error>
//     {
//         let row: Vec<&'a str> = iter.collect();
        
//         let assert_len = |i: usize| -> Result<(), Error> {
//             match row.len() {
//                 len if len == i => Ok(()),
//                 _ => Err(Error::RowLengthError {
//                     kind: kind,
//                     found: row.len(),
//                     row: row.iter().map(|s| s.to_string()).collect(),
//                     expected: i,
//                 }),
//             }
//         };
//         let map_none = |s: &'a str| -> Option<&'a str> {
//             match s {
//                 "\\N" => None,
//                 "" => None,
//                 s if s.chars().all(|c| c == ' ') => None,
//                 s => Some(s),
//             }
//         };
//         let map_int = |s: &'a str| -> Result<i32, Error> {
//             s.parse().map_err(|e| Error::StrToIntError {
//                 source: e,
//                 value: s.to_string(),
//                 row: row.iter().map(|s| s.to_string()).collect(),
//             })
//         };
//         let map_id = |s: &'a str| -> Result<i32, Error> {
//             match s.get(2..) {
//                 Some(s) => map_int(s),
//                 None => Err(Error::InvalidField {
//                     value: s.to_string(),
//                     row: row.iter().map(|s| s.to_string()).collect(),
//                     expected: "a Valid ID",
//                 })
//             }
//         };
//         let map_float = |s: &'a str| -> Result<f32, Error> {
//             s.parse().map_err(|e| Error::StrToFloatError {
//                 source: e,
//                 value: s.to_string(),
//                 row: row.iter().map(|s| s.to_string()).collect(),
//             })
//         };
//         let map_bool = |s: &'a str| -> Result<bool, Error> {
//             match s {
//                 "0" => Ok(false),
//                 "1" => Ok(true),
//                 _ => Err(Error::InvalidField {
//                     value: s.to_string(),
//                     row: row.iter().map(|s| s.to_string()).collect(),
//                     expected: "a bool: either 1 or 0",
//                 })
//             }
//         };

//         match kind {
//             DatasetKind::TitlePrincipals => {
//                 assert_len(6)?;
//                 Ok(Self::TitlePrincipals {
//                     imdb_id: map_id(row[0])?,
//                     ordering: map_int(row[1])?,
//                     name_id: map_id(row[2])?,
//                     category: row[3],
//                     job: map_none(row[4]),
//                     characters: map_none(row[5]),
//                 })
//             },
//             DatasetKind::NameBasics => {
//                 assert_len(6)?;
//                 Ok(Self::NameBasics {
//                     name_id: map_id(row[0])?,
//                     name: row[1],
//                     birth_year: map_none(row[2]).map(|s| map_int(s)).transpose()?,
//                     death_year: map_none(row[3]).map(|s| map_int(s)).transpose()?,
//                     primary_profession: map_none(row[4]).map(|s| s.split(',').collect()),
//                     imdb_ids: map_none(row[5]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
//                 })
//             },
//             DatasetKind::TitleAkas => {
//                 assert_len(8)?;
//                 Ok(Self::TitleAkas {
//                     imdb_id: map_id(row[0])?,
//                     ordering: map_int(row[1])?,
//                     title: map_none(row[2]),
//                     region: map_none(row[3]),
//                     language: map_none(row[4]),
//                     types: map_none(row[5]),
//                     attributes: map_none(row[6]),
//                     is_original_title: map_none(row[7]).map(|s| map_bool(s)).transpose()?,
//                 })
//             },
//             DatasetKind::TitleBasics => {
//                 assert_len(9)?;
//                 Ok(Self::TitleBasics {
//                     imdb_id: map_id(row[0])?,
//                     title_type: row[1],
//                     primary_title: map_none(row[2]),
//                     original_title: map_none(row[3]),
//                     is_adult: map_bool(row[4])?,
//                     start_year: map_none(row[5]).map(|s| map_int(s)).transpose()?,
//                     end_year: map_none(row[6]).map(|s| map_int(s)).transpose()?,
//                     runtime_minutes: map_none(row[7]).map(|s| map_int(s)).transpose()?,
//                     genres: map_none(row[8]).map(|s| s.split(',').collect()),
//                 })
//             },
//             DatasetKind::TitleCrew => {
//                 assert_len(3)?;
//                 Ok(Self::TitleCrew {
//                     imdb_id: map_id(row[0])?,
//                     directors: map_none(row[1]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
//                     writers: map_none(row[2]).map(|s| s.split(',').map(|id| map_id(id)).collect()).transpose()?,
//                 })
//             },
//             DatasetKind::TitleEpisode => {
//                 assert_len(4)?;
//                 Ok(Self::TitleEpisode {
//                     imdb_id: map_id(row[0])?,
//                     series_id: map_id(row[1])?,
//                     season_number: map_none(row[2]).map(|s| map_int(s)).transpose()?,
//                     episode_number: map_none(row[3]).map(|s| map_int(s)).transpose()?,
//                 })
//             },
//             DatasetKind::TitleRatings => {
//                 assert_len(3)?;
//                 Ok(Self::TitleRatings {
//                     imdb_id: map_id(row[0])?,
//                     average_rating: map_float(row[1])?,
//                     num_votes: map_int(row[2])?,
//                 })
//             },
//         }
//     }
// }