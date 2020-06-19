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
    pub kind: DatasetKind,
}

#[derive(Debug)]
pub enum Row<'a> {
    TitlePrincipals {
        imdb_id: u32,
        ordering: u32,
        name_id: u32,
        category: &'a str,
        job: Option<&'a str>,
        characters: Option<&'a str>,
    },
    NameBasics {
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
    TitleBasics { // titleType, primaryTitle, originalTitle, isAdult, startYear, endYear, runtimeMinutes, genres
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

struct Person {
    name_id: u32,
    name: Option<String>,
    category: Option<String>,
    job: Option<String>,
    characters: Option<String>,
    writer: Option<bool>,
    director: Option<bool>,
    birth_year: Option<u32>,
    death_year: Option<u32>,
    primary_profession: Option<Vec<String>>,
    other_work: Option<Vec<u32>>,
}

struct Title {
    title_type: String,
    primary_title: Option<String>,
    original_title: Option<String>,
    is_adult: bool,
    start_year: Option<u32>,
    end_year: Option<u32>,
    runtime_minutes: Option<u32>,
    genres: Option<Vec<String>>,
}

pub struct ImdbIdDatasetInfo {
    imdb_id: u32,
    series_id: Option<u32>,
    season_number: Option<u32>,
    episode_number: Option<u32>,
    average_rating: Option<f32>,
    num_votes: Option<u32>,
    titles: Vec<Title>,
    people: Vec<Person>,
}

#[derive(Debug, Display, Error, From)]
pub enum DataError {
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

impl Rows {
    pub fn into_iter<'a>(
        &'a self
    ) -> Result<impl Iterator<Item = Result<Row<'a>, DataError>>, DataError>
    {
        Ok(std::str::from_utf8(&self.inner)?
            .split('\n')
            .map(|row| row.split('\t'))
            .map(|row| Row::try_from_iter(row))
        )
    }
}

impl<'a> Row<'a> {
    fn try_from_iter(
        iter: impl Iterator<Item = &'a str>,
    ) -> Result<Self, DataError>
    {
        unimplemented!()
    }
}