pub mod error;
mod ost;
mod omdb;

pub use ost::Params as OpenSubtitlesParams;
pub use omdb::Params as OmdbParams;

use error::Error;
use serde_json::Value as Json;
use bytes::Bytes;
use async_trait::async_trait;
use std::time::{SystemTime, SystemTimeError};
use serde::{Serialize, Deserialize};

pub struct Client(reqwest::Client);

#[derive(Debug, Serialize, Deserialize)]
pub struct Data {
    pub kind: &'static str,
    pub timestamp: u64,
    pub data: Json,
}

#[async_trait]
pub trait Database {
    type Error: std::error::Error + Send + 'static;
    async fn insert(&self, data: Data) -> Result<(), Self::Error>;
}

pub trait Params {
    const KIND: &'static str;
    fn parse(&self) -> Result<reqwest::Request, error::ParamsError>;
    fn validate(&self, data: Bytes) -> Result<(), error::ResponseError>;
}

impl Client {
    pub fn new(client: reqwest::Client) -> Self {
        Self(client)
    }
    pub async fn get<P: Params, Db: Database>(
        &self,
        params: &P,
        database: &Db
    ) -> Result<(), Error<<Db as Database>::Error>>
    {
        let request = params.parse()?;
        let bytes = self.0.execute(request)
            .await?
            .error_for_status()?
            .bytes()
            .await?;
        params.validate(bytes.clone())?;
        let json: Json = serde_json::from_slice(&bytes)
            .map_err(|e| error::ResponseError::new(
                "the response by the server was not valid Json",
                error::ResponseErrorKind::InvalidJson,
                Some(e),
            ))?;
        database
            .insert(
                Data::new(P::KIND, json)?
            )
            .await
            .map_err(|e| Error::DatabaseError(e))
    }
}

impl Data {
    fn new(kind: &'static str, data: Json) -> Result<Self, SystemTimeError> {
        Ok(Self {
            kind: kind,
            data: data,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_secs()
        })
    }
}