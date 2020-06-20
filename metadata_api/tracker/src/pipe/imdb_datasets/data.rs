use derive_more::{Error, Display, From};

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
        imdb_id: u32,
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
        imdb_id: u32,
        ordering: u32,
        title: Option<&'a str>,
        region: Option<&'a str>,
        language: Option<&'a str>,
        types: Option<&'a str>,
        attributes: Option<&'a str>,
        is_original_title: Option<bool>,
    },
    TitleBasics { // tconst, titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres
        imdb_id: u32,
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
        imdb_id: u32,
        directors: Option<Vec<u32>>,
        writers: Option<Vec<u32>>,
    },
    TitleEpisode { // tconst, parentTconst, seasonNumber, episodeNumber
        imdb_id: u32,
        series_id: u32,
        season_number: Option<u32>,
        episode_number: Option<u32>,
    },
    TitleRatings { // tconst, averageRating, numVotes
        imdb_id: u32,
        average_rating: f32,
        num_votes: u32,
    },
}

#[derive(Debug, Display, Error)]
#[display(fmt="There was an error parsing this data.\nData: {:?}\nProblem: {}", data, kind)]
pub struct Error {
    data: bytes::Bytes,
    kind: ErrorKind,
}

#[derive(Debug, Display, Error, From)]
enum ErrorKind {
    #[display(fmt="imdb dataset was invalid utf-8")]
    Utf8Error(std::str::Utf8Error),
    #[display(fmt="imdb dataset field was not a number")]
    #[from(ignore)]
    FromStrError(<u32 as std::str::FromStr>::Err),
    #[display(fmt="imdb dataset had incorrect row length")]
    RowLengthError(
        #[error(not(source))]
        usize, usize
    ),
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

pub enum Error {
    Utf8Error {
        source: std::str::Utf8Error,
        chunk: bytes::Bytes,
    },
    FromStrError {
        source: <u32 as std::str::FromStr>::Err,
        value: String,
        row: Vec<String>,
    },
    RowLengthError {
        row: Vec<String>,
        
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
                .map(|row| row.split('\t'))
        )
    }
}

impl<'a> std::convert::TryFrom<&'a Rows> for Vec<Row<'a>> {
    type Error = Error;
    fn try_from(value: &'a Rows) -> Result<Self, Self::Error> {
        value.try_iter()
            .map_err(|e| Error {
                data: value.inner.clone(),
                kind: ErrorKind::from(e),
            })?
            .map(|row| Row::try_from_iter(row, value.kind))
            .collect::<Result<Vec<Row>, ErrorKind>>()
            .map_err(|e| Error {
                data: value.inner.clone(),
                kind: e,
            })
    }
}

impl<'a> Row<'a> {
    fn try_from_iter(
        iter: impl Iterator<Item = &'a str>,
        kind: DatasetKind,
    ) -> Result<Self, ErrorKind> {
        match kind {
            DatasetKind::TitlePrincipals => {
                match iter.collect::<Vec<&str>>().len() {
                    6 => (),
                    i => return Err(ErrorKind::RowLengthError(6, i)),
                };
                Ok(Self::TitlePrincipals {
                    imdb_id: 0,
                    ordering: 0,
                    name_id: 0,
                    category: "",
                    job: None,
                    characters: None,
                })
            },
            DatasetKind::NameBasics => {
                match iter.collect::<Vec<&str>>().len() {
                    6 => (),
                    i => return Err(ErrorKind::RowLengthError(6, i)),
                };
                Ok(Self::NameBasics {
                    name_id: 0,
                    name: "",
                    birth_year: None,
                    death_year: None,
                    primary_profession: None,
                    imdb_ids: None,
                })
            },
            DatasetKind::TitleAkas => {
                match iter.collect::<Vec<&str>>().len() {
                    8 => (),
                    i => return Err(ErrorKind::RowLengthError(8, i)),
                };
                Ok(Self::TitleAkas {
                    imdb_id: 0,
                    ordering: 0,
                    title: None,
                    region: None,
                    language: None,
                    types: None,
                    attributes: None,
                    is_original_title: None,
                })
            },
            DatasetKind::TitleBasics => {
                match iter.collect::<Vec<&str>>().len() {
                    9 => (),
                    i => return Err(ErrorKind::RowLengthError(9, i)),
                };
                Ok(Self::TitleBasics {
                    imdb_id: 0,
                    title_type: "",
                    primary_title: None,
                    original_title: None,
                    is_adult: false,
                    start_year: None,
                    end_year: None,
                    runtime_minutes: None,
                    genres: None,
                })
            },
            DatasetKind::TitleCrew => {
                match iter.collect::<Vec<&str>>().len() {
                    3 => (),
                    i => return Err(ErrorKind::RowLengthError(3, i)),
                };
                Ok(Self::TitleCrew {
                    imdb_id: 0,
                    directors: None,
                    writers: None,
                })
            },
            DatasetKind::TitleEpisode => {
                match iter.collect::<Vec<&str>>().len() {
                    4 => (),
                    i => return Err(ErrorKind::RowLengthError(4, i)),
                };
                Ok(Self::TitleEpisode {
                    imdb_id: 0,
                    series_id: 0,
                    season_number: None,
                    episode_number: None,
                })
            },
            DatasetKind::TitleRatings => {
                match iter.collect::<Vec<&str>>().len() {
                    3 => (),
                    i => return Err(ErrorKind::RowLengthError(3, i)),
                };
                Ok(Self::TitleRatings {
                    imdb_id: 0,
                    average_rating: 0.0,
                    num_votes: 0,
                })
            },
        }
    }
}