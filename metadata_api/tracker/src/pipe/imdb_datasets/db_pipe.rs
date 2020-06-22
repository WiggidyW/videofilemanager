use crate::pipe::Pipe;
use crate::token::ImdbId;
use super::{Row, Rows, DataError, TitleInfo, TitleInfoEpisode, TitleInfoTitle, TitleInfoPerson};
use derive_more::{Display, Error, From};
use tokio::sync::Mutex as TokioMutex;
use std::collections::{HashSet, HashMap};
use async_trait::async_trait;
use futures::stream::StreamExt;
use futures::stream::futures_unordered::FuturesUnordered;
use std::sync::Arc;

pub struct SqlitePipe<P> {
    pipe: P,
    pool: sqlx::SqlitePool,
    client: reqwest::Client,
}

#[derive(Debug, From, Display, Error)]
pub enum SqliteError<E> {
    DataError(DataError),
    DatabaseError(sqlx::Error),
    #[from(ignore)]
    PipeError(E),
    ImdbIdError(crate::token::ImdbIdError),
    #[display(fmt = "The provided ImdbId does not exist")]
    InvalidImdbId,
    JsonError(serde_json::Error),
}

#[async_trait]
impl<P: Pipe<(), Rows>> Pipe<ImdbId, TitleInfo> for SqlitePipe<P> {
    type Error = SqliteError<<P as Pipe<(), Rows>>::Error>;
    type Stream = futures::stream::Iter<std::iter::Once<Result<TitleInfo, Self::Error>>>;
    async fn get(&self, token: ImdbId) -> Result<Self::Stream, Self::Error> {
        let mut title_info = self.get_title_info(token).await?;
        if {
            &title_info.title_type == &None
        } {
            self.refresh().await?;
            title_info = self.get_title_info(token).await?;
        }
        Ok(futures::stream::iter(std::iter::once(Ok(title_info))))
    }
}

impl<P: Pipe<(), Rows>> SqlitePipe<P> {
    pub fn new(pipe: P, pool: sqlx::SqlitePool, client: reqwest::Client) -> Self {
        Self {
            pipe: pipe,
            pool: pool,
            client: client,
        }
    }
    pub async fn tables_up(&self) -> Result<(), SqliteError<<P as Pipe<(), Rows>>::Error>> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query_file!("resources/imdb_datasets_UP.sql")
            .execute(&mut conn)
            .await?;
        Ok(())
    }
    pub async fn tables_down(&self) -> Result<(), SqliteError<<P as Pipe<(), Rows>>::Error>> {
        let mut conn = self.pool.acquire().await?;
        sqlx::query_file!("resources/imdb_datasets_DOWN.sql")
            .execute(&mut conn)
            .await?;
        Ok(())
    }
    async fn refresh(&self) -> Result<(), SqliteError<<P as Pipe<(), Rows>>::Error>> {
        let insert_row = |row: Row| {
            match row {
                Row::TitlePrincipals {
                    imdb_id, ordering, name_id, category, job, characters,
                } => sqlx::query!(
                    "INSERT INTO title_principals VALUES ( $1, $2, $3, $4, $5, $6, $7 )",
                    Option::<i32>::None, imdb_id, ordering, name_id, category, job, characters,
                ),
                Row::NameBasics {
                    name_id, name, birth_year, death_year, primary_profession, imdb_ids,
                } => sqlx::query!(
                    "INSERT INTO name_basics VALUES ( $1, $2, $3, $4, $5, $6, $7 )",
                    Option::<i32>::None, name_id, name, birth_year, death_year,
                    primary_profession.map(|s| serde_json::Value::from(s).to_string()),
                    imdb_ids.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleAkas {
                    imdb_id, ordering, title, region, language, types, attributes,
                    is_original_title,
                } => sqlx::query!(
                    "INSERT INTO title_akas VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9 )",
                    Option::<i32>::None, imdb_id, ordering, title, region, language, types,
                    attributes, is_original_title,
                ),
                Row::TitleBasics {
                    imdb_id, title_type, primary_title, original_title, is_adult, start_year,
                    end_year, runtime_minutes, genres,
                } => sqlx::query!(
                    "INSERT INTO title_basics VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10 )",
                    Option::<i32>::None, imdb_id, title_type, primary_title, original_title,
                    is_adult, start_year, end_year, runtime_minutes,
                    genres.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleCrew {
                    imdb_id, directors, writers,
                } => sqlx::query!(
                    "INSERT INTO title_crew VALUES ( $1, $2, $3, $4 )",
                    Option::<i32>::None, imdb_id,
                    directors.map(|s| serde_json::Value::from(s).to_string()),
                    writers.map(|s| serde_json::Value::from(s).to_string()),
                ),
                Row::TitleEpisode {
                    imdb_id, series_id, season_number, episode_number,
                } => sqlx::query!(
                    "INSERT INTO title_episode VALUES ( $1, $2, $3, $4, $5 )",
                    Option::<i32>::None, imdb_id, series_id, season_number, episode_number,
                ),
                Row::TitleRatings {
                    imdb_id, average_rating, num_votes,
                } => sqlx::query!(
                    "INSERT INTO title_ratings VALUES ( $1, $2, $3, $4 )",
                    Option::<i32>::None, imdb_id, average_rating, num_votes,
                ),
            }
        };
        let transaction = Arc::new(TokioMutex::new( // needed for looping, rust shenanigans
            self.pool.begin().await?
        ));
        sqlx::query_file!("resources/imdb_datasets_DOWN.sql") // drops tables (in transaction)
            .execute(&mut *transaction.lock().await)
            .await?;
        sqlx::query_file!("resources/imdb_datasets_UP.sql") // creates tables (in transaction)
            .execute(&mut *transaction.lock().await)
            .await?;
        let futures_pool = FuturesUnordered::new(); // will store the futures that parse + insert
        let mut stream = self.pipe.get(()).await.map_err(|e| SqliteError::PipeError(e))?;
        while let Some(rows) = stream.next().await {
            let rows = rows.map_err(|e| SqliteError::PipeError(e))?;
            let tx = transaction.clone();
            futures_pool.push(tokio::spawn(async move {
                for row in rows.try_iter()? // this iterator makes up a lot of our cpu time
                {
                    let row = row?;
                    insert_row(row) // query
                        .execute(&mut *tx.lock().await)
                        .await?;
                }
                Result::<
                    (),
                    SqliteError<<P as Pipe<(), Rows>>::Error>,
                >::Ok(())
            }));
        }
        futures_pool.then(|join_handle| async move { join_handle.unwrap() })
            .collect::<Vec<Result<(), SqliteError<<P as Pipe<(), Rows>>::Error>>>>()
            .await
            .into_iter()
            .collect::<Result<_, SqliteError<<P as Pipe<(), Rows>>::Error>>>()?;
        match Arc::try_unwrap(transaction) {
            Ok(transaction) => transaction.into_inner().commit().await?,
            Err(_) => unreachable!(),
        };
        Ok(())
    }
    async fn get_title_info(
        &self,
        id: ImdbId,
    ) -> Result<TitleInfo, SqliteError<<P as Pipe<(), Rows>>::Error>> {
        // inner closure that will short circuit if the imdb_id is invalid
        let validate_imdb_id = || async {
            let mut connection = self.pool.acquire().await?;
            match id.is_valid_cached() {
                None => Ok({
                    if let Some(_) = sqlx::query!(
                        r#"
SELECT imdb_id
FROM title_info
WHERE
    imdb_id = $1
                        "#,
                        *id.as_ref() as i32,
                    )
                        .fetch_optional(&mut connection)
                        .await?
                    {
                        id.force_valid_cached();
                    }
                    else {
                        if !id.is_valid(&self.client).await?
                        {
                            return Result::<(), SqliteError<<P as Pipe<(), Rows>>::Error>>::Err(
                                SqliteError::InvalidImdbId
                            );
                        }
                    }
                }),
                Some(false) => Err(SqliteError::InvalidImdbId),
                Some(true) => Ok(()),
            }
        };

        // inner closure that will mutate given data with info and then return it
        let query_title_info_info = |id: i32, pool: sqlx::SqlitePool| async move {
            let mut connection = pool.acquire().await?;
            let mut data = TitleInfo::default();
            data.imdb_id = id;
            if let Some(d) = sqlx::query!(
                r#"
SELECT title_type, primary_title, original_title, is_adult, start_year, end_year, runtime_minutes,
    genres, average_rating, num_votes, series_id, season_number, episode_number
FROM title_info
WHERE
    imdb_id = $1
                "#,
                id,
            )
                .fetch_optional(&mut connection)
                .await?
            {
                data.title_type = d.title_type;
                data.primary_title = d.primary_title;
                data.original_title = d.original_title;
                data.is_adult = d.is_adult;
                data.start_year = d.start_year;
                data.end_year = d.end_year;
                data.runtime_minutes = d.runtime_minutes;
                data.genres = d.genres.map(|g| serde_json::from_str(&g)).transpose()?;
                data.average_rating = d.average_rating;
                data.num_votes = d.num_votes;
                data.series_id = d.series_id;
                data.season_number = d.season_number;
                data.episode_number = d.episode_number;
            }
            Result::<
                TitleInfo,
                SqliteError<<P as Pipe<(), Rows>>::Error>,
            >::Ok(data)
        };

        // inner closure that will mutate given data with episodes and then return it
        let query_title_info_episodes = |id: i32, pool: sqlx::SqlitePool| async move {
            let mut connection = pool.acquire().await?;
            let data = match sqlx::query_as!(
                TitleInfoEpisode,
                r#"
SELECT imdb_id, season_number, episode_number
FROM title_episode
WHERE
    imdb_id = $1
                "#,
                id,
            )
                .fetch_all(&mut connection)
                .await?
            {
                vec if vec.len() > 0 => Some(vec),
                _ => None,
            };
            Result::<
                Option<Vec<TitleInfoEpisode>>,
                SqliteError<<P as Pipe<(), Rows>>::Error>,
            >::Ok(data)
        };

        // inner closure that will mutate given data with titles and then return it
        let query_title_info_titles = |id: i32, pool: sqlx::SqlitePool| async move {
            let mut connection = pool.acquire().await?;
            let data = match sqlx::query_as!(
                TitleInfoTitle,
                r#"
SELECT title, ordering, region, language, types, attributes, is_original_title
FROM title_akas
WHERE
    title IS NOT NULL AND
    imdb_id = $1
                "#,
                id,
            )
                .fetch_all(&mut connection)
                .await?
            {
                vec if vec.len() > 0 => Some(vec),
                _ => None,
            };
            Result::<
                Option<Vec<TitleInfoTitle>>,
                SqliteError<<P as Pipe<(), Rows>>::Error>,
            >::Ok(data)
        };

        // inner closure that will mutate given data with persons and then return it
        let query_title_info_people = |id: i32, pool: sqlx::SqlitePool| async move {
            let mut connection = pool.acquire().await?;
            let (mut writers, mut directors): (HashSet<i32>, HashSet<i32>) = match sqlx::query!(
                r#"
SELECT writers, directors
FROM title_info
WHERE
    imdb_id = $1
                "#,
                id,
            )
                .fetch_optional(&mut connection)
                .await?
            {
                None => (HashSet::new(), HashSet::new()),
                Some(s) => match (s.writers, s.directors) {
                    (None, None) => (HashSet::new(), HashSet::new()),
                    (None, Some(d)) => (
                        HashSet::new(),
                        serde_json::from_str(&d)?
                    ),
                    (Some(w), None) => (
                        serde_json::from_str(&w)?,
                        HashSet::new()
                    ),
                    (Some(w), Some(d)) => (
                        serde_json::from_str(&w)?,
                        serde_json::from_str(&d)?
                    ),
                },
            };
            let mut data: HashMap<i32, TitleInfoPerson> = HashMap::new();
            let mut cursor = sqlx::query!(
                r#"
SELECT name_id, category, job, characters
FROM title_principals
WHERE
    imdb_id = $1
                "#,
                id,
            )
                .fetch(&mut connection);
            while let Some(row) = cursor.next().await {
                let row = row?;
                match data.get_mut(&row.name_id) {
                    Some(person) => {
                        person.categories.as_mut().unwrap().push(row.category); // will never be None!
                        if let Some(job) = row.job {
                            match person.jobs.as_mut() {
                                Some(j) => j.push(job),
                                None => person.jobs = Some(vec![job]),
                            };
                        }
                        if let Some(characters) = row.characters {
                            let mut characters: Vec<String> = serde_json::from_str(&characters)?;
                            match person.characters.as_mut() {
                                Some(c) => c.append(&mut characters),
                                None => person.characters = Some(characters),
                            };
                        }
                    },
                    None => {
                        data.insert(row.name_id, {
                            let mut person = TitleInfoPerson::default();
                            person.name_id = row.name_id;
                            person.categories = Some(vec![row.category]);
                            person.jobs = row.job.map(|j| vec![j]);
                            person.characters = row.characters
                                .map(|c| serde_json::from_str(&c))
                                .transpose()?;
                            person.writer = writers.remove(&row.name_id);
                            person.director = directors.remove(&row.name_id);
                            person
                        });
                    },
                };
            }
            writers.into_iter().for_each(|name_id| {
                data.insert(name_id, {
                    let mut person = TitleInfoPerson::default();
                    person.name_id = name_id;
                    person.writer = true;
                    person.director = directors.remove(&name_id);
                    person
                });
            });
            directors.into_iter().for_each(|name_id| {
                let _ = data.insert(name_id, {
                    let mut person = TitleInfoPerson::default();
                    person.name_id = name_id;
                    person.director = true;
                    person
                });
            });
            let connection = TokioMutex::new( // it was destroyed when cursor was dropped
                pool.acquire().await?
            );
            for person in data.values_mut() {
                match sqlx::query!(
                    r#"
SELECT name, birth_year, death_year
FROM name_basics
WHERE
    name_id = $1
                    "#,
                    person.name_id,
                )
                    .fetch_optional(&mut *connection.lock().await)
                    .await?
                {
                    None => (),
                    Some(p) => {
                        person.name = Some(p.name);
                        person.birth_year = p.birth_year;
                        person.death_year = p.death_year;
                    },
                }
            }
            let data = match data.len() {
                0 => None,
                _ => Some(data.into_iter().map(|(_, p)| p).collect()),
            };
            Result::<
                Option<Vec<TitleInfoPerson>>,
                SqliteError<<P as Pipe<(), Rows>>::Error>,
            >::Ok(data)
        };

        validate_imdb_id().await?;
        let id = *id.as_ref() as i32;
        let people_task = tokio::spawn(query_title_info_people(id, self.pool.clone()));
        let title_info_task = tokio::spawn(query_title_info_info(id, self.pool.clone()));
        let titles_task = tokio::spawn(query_title_info_titles(id, self.pool.clone()));
        let episodes_task = tokio::spawn(query_title_info_episodes(id, self.pool.clone()));
        let mut title_info = title_info_task.await.unwrap()?;
        title_info.titles = titles_task.await.unwrap()?;
        title_info.episodes = episodes_task.await.unwrap()?;
        title_info.people = people_task.await.unwrap()?;
        Ok(title_info)
    }
}