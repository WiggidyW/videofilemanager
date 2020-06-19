use super::{Client, ShouldRefresh, Timestamp};
use serde::{Serialize, Deserialize};
use sqlx::sqlite;

pub struct ImdbDatasetsClient<C> {
    conn: sqlite::SqliteConnection,
    client: C,
}

impl<'a, C> ImdbDatasetsClient<C>
where
    C: Client<'a, Dataset, Item = Row<'a>>,
{

}

#[derive(Clone, Copy)]
pub enum Dataset {
    TitlePrincipals,
    NameBasics,
    TitleAkas,
    TitleBasics,
    TitleCrew,
    TitleEpisode,
    TitleRatings,
}

#[derive(Serialize, Deserialize)]
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