use crate::pipe::Pipe;
use super::{Row, Rows, DataError};
use derive_more::{Display, Error, From};
use futures::stream::StreamExt;
use std::cell::RefCell;

pub struct SqlitePipe<P> {
    pipe: P,
    pool: sqlx::SqlitePool,
}

#[derive(Debug, From, Display, Error)]
pub enum SqliteError<E> {
    DataError(DataError),
    DatabaseError(sqlx::Error),
    #[from(ignore)]
    PipeError(E),
}

impl<P: Pipe<(), Rows>> SqlitePipe<P> {
    pub fn new(pipe: P, pool: sqlx::SqlitePool) -> Self {
        Self {
            pipe: pipe,
            pool: pool,
        }
    }
    async fn refresh(&self) -> Result<(), SqliteError<<P as Pipe<(), Rows>>::Error>> {
        let insert_row = |row: Row| {
            match row {
                Row::TitlePrincipals {
                    imdb_id, ordering, name_id, category, job, characters,
                } => sqlx::query!(
                    r#"INSERT INTO title_principals VALUES ( $1, $2, $3, $4, $5, $6, $7 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    ordering,
                    name_id,
                    category,
                    job,
                    characters,
                ),
                Row::NameBasics {
                    name_id, name, birth_year, death_year, primary_profession, imdb_ids,
                } => sqlx::query!(
                    r#"INSERT INTO name_basics VALUES ( $1, $2, $3, $4, $5, $6, $7 )"#,
                    Option::<i32>::None,
                    name_id,
                    name,
                    birth_year,
                    death_year,
                    primary_profession.map(|s| serde_json::Value::from(s).to_string()),
                    imdb_ids.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleAkas {
                    imdb_id, ordering, title, region, language, types, attributes, is_original_title,
                } => sqlx::query!(
                    r#"INSERT INTO title_akas VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    ordering,
                    title,
                    region,
                    language,
                    types,
                    attributes,
                    is_original_title,
                ),
                Row::TitleBasics {
                    imdb_id, title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes, genres,
                } => sqlx::query!(
                    r#"INSERT INTO title_basics VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    title_type,
                    primary_title,
                    original_title,
                    is_adult,
                    start_year,
                    end_year,
                    runtime_minutes,
                    genres.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleCrew {
                    imdb_id, directors, writers,
                } => sqlx::query!(
                    r#"INSERT INTO title_crew VALUES ( $1, $2, $3, $4 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    directors.map(|s| serde_json::Value::from(s).to_string()),
                    writers.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleEpisode {
                    imdb_id, series_id, season_number, episode_number,
                } => sqlx::query!(
                    r#"INSERT INTO title_episode VALUES ( $1, $2, $3, $4, $5 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    series_id,
                    season_number,
                    episode_number,
                ),
                Row::TitleRatings {
                    imdb_id, average_rating, num_votes,
                } => sqlx::query!(
                    r#"INSERT INTO title_ratings VALUES ( $1, $2, $3, $4 )"#,
                    Option::<i32>::None,
                    imdb_id,
                    average_rating,
                    num_votes,
                ),
            }
        };
        let transaction = RefCell::new( // using a refcell here because of... rust shenanigans
            self.pool.begin().await?
        );
        sqlx::query_file!("resources/imdb_datasets.sql")
            .execute(&mut *transaction.borrow_mut())
            .await?;
        let mut stream = self.pipe.get(()).await.map_err(|e| SqliteError::PipeError(e))?;
        while let Some(rows) = stream.next().await {
            let rows = rows.map_err(|e| SqliteError::PipeError(e))?;
            for row in rows.try_iter()? {
                let row = row?;
                insert_row(row)
                    .execute(&mut *transaction.borrow_mut())
                    .await?;
            }
        }
        Ok(())
    }
}