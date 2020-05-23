use serde::Deserialize;
use crate::deser;

// use std::io::Read;

// pub fn read() -> impl Iterator<Item = Result<TitleRatings, csv::Error>> {
//     csv::ReaderBuilder::new()
//         .delimiter(b'\t')
//         .quoting(false)
//         .from_path("/home/user/Programming/videofilemanager/metadata_api/resources/test/title.ratings.tsv")
//         .unwrap()
//         .into_deserialize()
// }

#[derive(Debug, Deserialize)]
pub struct TitleRatings {
    #[serde(alias = "tconst", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "averageRating")]
    average_rating: f32,
    #[serde(alias = "numVotes")]
    num_votes: u32,
}

#[derive(Debug, Deserialize)]
pub struct TitleEpisode {
    #[serde(alias = "tconst", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "parentTconst", deserialize_with = "deser::imdb_id")]
    series_id: u32,
    #[serde(alias = "seasonNumber", deserialize_with = "csv::invalid_option")]
    season_number: Option<u32>,
    #[serde(alias = "episodeNumber", deserialize_with = "csv::invalid_option")]
    episode_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct TitleCrew {
    #[serde(alias = "tconst", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "directors", deserialize_with = "deser::option_name_id_vec")]
    directors: Option<Vec<u32>>,
    #[serde(alias = "writers", deserialize_with = "deser::option_name_id_vec")]
    writers: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct TitleBasics {
    #[serde(alias = "tconst", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "titleType")]
    title_type: String,
    #[serde(alias = "primaryTitle")]
    primary_title: String,
    #[serde(alias = "originalTitle")]
    original_title: String,
    #[serde(alias = "isAdult", deserialize_with = "deser::boolean")]
    is_adult: bool,
    #[serde(alias = "startYear", deserialize_with = "csv::invalid_option")]
    start_year: Option<u32>,
    #[serde(alias = "endYear", deserialize_with = "csv::invalid_option")]
    end_year: Option<u32>,
    #[serde(alias = "runtimeMinutes", deserialize_with = "csv::invalid_option")]
    runtime_minutes: Option<u32>,
    #[serde(alias = "genres", deserialize_with = "deser::option_string_vec")]
    genres: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct TitleAkas {
    #[serde(alias = "titleId", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "ordering")]
    ordering: u32,
    #[serde(alias = "title")]
    title: String,
    #[serde(alias = "region", deserialize_with = "deser::option_string")]
    region: Option<String>,
    #[serde(alias = "language", deserialize_with = "deser::option_string")]
    language: Option<String>,
    #[serde(alias = "types", deserialize_with = "deser::option_string")]
    types: Option<String>,
    #[serde(alias = "attributes", deserialize_with = "deser::option_string")]
    attributes: Option<String>,
    #[serde(alias = "isOriginalTitle", deserialize_with = "deser::option_boolean")]
    is_original_title: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct NameBasics {
    #[serde(alias = "nconst", deserialize_with = "deser::name_id")]
    person_id: u32,
    #[serde(alias = "primaryName")]
    primary_name: String,
    #[serde(alias = "birthYear", deserialize_with = "csv::invalid_option")]
    birth_year: Option<u32>,
    #[serde(alias = "deathYear", deserialize_with = "csv::invalid_option")]
    death_year: Option<u32>,
    #[serde(alias = "primaryProfession", deserialize_with = "deser::option_string_vec")]
    primary_profession: Option<Vec<String>>,
    #[serde(alias = "knownForTitles", deserialize_with = "deser::option_imdb_id_vec")]
    known_for_titles: Option<Vec<u32>>,
}

#[derive(Debug, Deserialize)]
pub struct TitlePrincipals {
    #[serde(alias = "tconst", deserialize_with = "deser::imdb_id")]
    imdb_id: u32,
    #[serde(alias = "ordering")]
    ordering: u32,
    #[serde(alias = "nconst", deserialize_with = "deser::name_id")]
    name_id: u32,
    #[serde(alias = "category")]
    category: String,
    #[serde(alias = "job", deserialize_with = "deser::option_string")]
    job: Option<String>,
    #[serde(alias = "characters", deserialize_with = "deser::option_string")]
    characters: Option<String>,
}