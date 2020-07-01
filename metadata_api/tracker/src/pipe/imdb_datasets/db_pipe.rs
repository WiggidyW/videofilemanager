use derive_more::{Display, Error, From};

pub struct PostgresPipe<P> {
    conn_pool: sqlx::PgPool,
    crow_pipe: P,
}

#[derive(From)]
pub enum PostgresPipeError<E> {
    #[from(ignore)]
    ChunkRowPipeError(E),
    DatabaseError(sqlx::Error),
    ToRowsError(super::ToRowError),
}

mod postgres_pipe {
    use super::{PostgresPipe, PostgresPipeError};
    use crate::pipe::Pipe;
    use crate::pipe::imdb_datasets::{DatasetKind, ChunkRow, Row};

    impl<P: Pipe<DatasetKind, ChunkRow>> PostgresPipe<P> {
        pub async fn insert_row(&self, row: Row) -> Result<(), PostgresPipeError<P::Error>> {
            let mut conn = self.conn_pool.acquire().await?;
            let query = match row {
                Row::TitlePrincipals {
                    imdb_id, ordering, name_id, category, job, characters,
                } => sqlx::query!(
                    r#"
INSERT INTO title_person ( imdb_id, name_id, principals )
VALUES ( $1, $2, $3 )
ON CONFLICT ( imdb_id, name_id )
DO UPDATE SET
    principals = array_append(EXCLUDED.principals, $3)
                    "#,
                    imdb_id, name_id, (ordering, category, job, characters),
                ),
                Row::NameBasics {
                    name_id, name, birth_year, death_year, primary_profession, imdb_ids,
                } => sqlx::query!(
                    r#"
INSERT INTO name_basics ( name_id, name, birth_year, death_year, primary_profession, imdb_ids )
VALUES ( $1, $2, $3, $4, $5, $6 )
                    "#,
                    name_id, name, birth_year, death_year, primary_profession, imdb_ids,
                ),
                Row::TitleAkas {
                    imdb_id, ordering, title, region, language, types, attributes,
                    is_original_title,
                } => sqlx::query!(
                    r#"
INSERT INTO title_akas ( imdb_id, ordering, title, region, language, types, attributes,
    is_original_title )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
                    "#,
                    imdb_id, ordering, title, region, language, types, attributes,
                    is_original_title,
                ),
                Row::TitleBasics {
                    imdb_id, title_type, primary_title, original_title, is_adult, start_year,
                    end_year, runtime_minutes, genres,
                } => sqlx::query!(
                    r#"
INSERT INTO title_info ( imdb_id, title_type, primary_title, original_title, is_adult, start_year,
    end_year, runtime_minutes, genres )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )
ON CONFLICT ( imdb_id )
DO UPDATE SET
    title_type = $2,
    primary_title = $3,
    original_title = $4,
    is_adult = $5,
    start_year = $6,
    end_year = $7,
    runtime_minutes = $8,
    genres = $9
                    "#,
                    imdb_id, title_type, primary_title, original_title, is_adult, start_year,
                    end_year, runtime_minutes, genres,
                ),
                Row::TitleCrew {
                    imdb_id, directors, writers,
                } => sqlx::query!(
                    r#"
SELECT title_crew($1, $2, $3)
                    "#,
                    imdb_id, directors, writers,
                ),
                Row::TitleEpisode {
                    imdb_id, series_id, season_number, episode_number,
                } => sqlx::query!(
                    r#"
INSERT INTO title_info ( imdb_id, series_id, season_number, episode_number )
VALUES ( $1, $2, $3, $4 )
ON CONFLICT ( imdb_id )
DO UPDATE SET
    series_id = $2,
    season_number = $3,
    episode_number = $4
                    "#,
                    imdb_id, series_id, season_number, episode_number,
                ),
                Row::TitleRatings {
                    imdb_id, average_rating, num_votes,
                } => sqlx::query!(
                    r#"
INSERT INTO title_info ( imdb_id, average_rating, num_votes )
VALUES ( $1, $2, $3 )
ON CONFLICT ( imdb_id )
DO UPDATE SET
    average_rating = $2,
    num_votes = $3;
                    "#,
                    imdb_id, average_rating, num_votes,
                ),
            };
        }
    }
}