use derive_more::{Display, Error, From};
use std::sync::Arc;
use super::{DatasetKind, ChunkRow, Row};

pub struct PgPipe<P> {
    pool: sqlx::PgPool,
    chunk_row_pipe: Arc<P>,
}

#[derive(Debug, From, Display, Error)]
pub enum PgPipeError<E> {
    #[from(ignore)]
    ChunkRowPipeError(E),
    DatabaseError(sqlx::Error),
    ToRowsError(super::ToRowError),
}

mod postgres_pipe {
    use super::{PgPipe, PgPipeError, DatasetKind, ChunkRow, Row};
    use crate::Pipe;
    use crate::tokens::Refresh;
    use futures::stream::{Stream, StreamExt};
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::mpsc as tokio_mpsc;

    pub struct PgInsertStream<S> {
        pool: sqlx::PgPool,
        stream: S,
    }

    pub async fn insert_row<C>(conn: C, row: Row<'_>) -> Result<(), sqlx::Error>
    where
        C: sqlx::Executor<Database = sqlx::Postgres>,
    {
        let query = match row {
            Row::TitlePrincipals { imdb_id, ordering, name_id, category, job, characters } => sqlx::query(
                    r#"
SELECT title_principals($1, $2, $3, $4, $5, $6);
                    "#
                ).bind(imdb_id).bind(ordering).bind(name_id).bind(category).bind(job).bind(characters),
            Row::NameBasics { name_id, name, birth_year, death_year, primary_profession, imdb_ids } => sqlx::query(
                    r#"
INSERT INTO name_basics ( name_id, name, birth_year, death_year, primary_profession, imdb_ids )
VALUES ( $1, $2, $3, $4, $5, $6 )
                    "#
                ).bind(name_id).bind(name).bind(birth_year).bind(death_year).bind(primary_profession).bind(imdb_ids),
            Row::TitleAkas { imdb_id, ordering, title, region, language, types, attributes, is_original_title } => sqlx::query(
                    r#"
INSERT INTO title_akas ( imdb_id, ordering, title, region, language, types, attributes, is_original_title )
VALUES ( $1, $2, $3, $4, $5, $6, $7, $8 )
                    "#
            ).bind(imdb_id).bind(ordering).bind(title).bind(region).bind(language).bind(types).bind(attributes).bind(is_original_title),
            Row::TitleBasics { imdb_id, title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes, genres } => sqlx::query(
                    r#"
INSERT INTO title_info ( imdb_id, title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes, genres )
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
                    "#
                ).bind(imdb_id).bind(title_type).bind(primary_title).bind(original_title).bind(is_adult).bind(start_year).bind(end_year).bind(runtime_minutes).bind(genres),
            Row::TitleCrew { imdb_id, directors, writers } => sqlx::query(
                    r#"
SELECT title_crew($1, $2, $3)
                    "#
                ).bind(imdb_id).bind(directors).bind(writers),
            Row::TitleEpisode { imdb_id, series_id, season_number, episode_number } => sqlx::query(
                    r#"
INSERT INTO title_info ( imdb_id, series_id, season_number, episode_number )
VALUES ( $1, $2, $3, $4 )
ON CONFLICT ( imdb_id )
DO UPDATE SET
series_id = $2,
season_number = $3,
episode_number = $4
                    "#
                ).bind(imdb_id).bind(series_id).bind(season_number).bind(episode_number),
            Row::TitleRatings { imdb_id, average_rating, num_votes } => sqlx::query(
                    r#"
INSERT INTO title_info ( imdb_id, average_rating, num_votes )
VALUES ( $1, $2, $3 )
ON CONFLICT ( imdb_id )
DO UPDATE SET
average_rating = $2,
num_votes = $3
                    "#
                ).bind(imdb_id).bind(average_rating).bind(num_votes),
        };
        query.execute(conn).await?;
        Ok(())
    }

    #[async_trait]
    impl<P: Pipe<DatasetKind, ChunkRow>> Pipe<DatasetKind, ()> for PgPipe<P> {
        type Error = PgPipeError<P::Error>;
        type Stream = impl Stream<Item = Result<(), Self::Error>> + Send + Unpin;
        async fn pull(self: Arc<Self>, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
            let stream = self.chunk_row_pipe.clone()
                .pull(token)
                .await
                .map_err(|e| PgPipeError::ChunkRowPipeError(e))?
                .zip(futures::stream::repeat(self.pool.clone()))
                .then(|(result, pool)| Box::pin(async move { // have to box pin for reasons known only by the rust async lifetime wizards
                    let chunk_row = result.map_err(|e| PgPipeError::ChunkRowPipeError(e))?;
                    let row = chunk_row.to_row()?;
                    insert_row(&pool, row).await?;
                    Ok(())
                }));
            Ok(stream)
        }
    }

    #[async_trait]
    impl<P: Pipe<DatasetKind, ChunkRow>> Pipe<Refresh, ()> for PgPipe<P> {
        type Error = PgPipeError<P::Error>;
        type Stream = impl Stream<Item = Result<(), Self::Error>> + Send + Unpin;
        async fn pull(self: Arc<Self>, _: Refresh) -> Result<Self::Stream, Self::Error> {
            let (tx, rx) = tokio_mpsc::unbounded_channel();
            for kind in DatasetKind::iter() {
                let tx = tx.clone();
                let mut stream = <Self as Pipe<DatasetKind, ()>>::pull(self.clone(), kind).await?;
                let _ = tokio::spawn(async move {
                    while let Some(result) = stream.next().await {
                        tx.send(result).unwrap();
                    }
                });
            }
            Ok(rx)
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    }
}