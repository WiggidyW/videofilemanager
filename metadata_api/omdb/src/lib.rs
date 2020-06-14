use std::error::Error as StdError;
use async_trait::async_trait;
use serde::Serialize;
use reqwest::Url;
mod error;

pub use crate::error::Error;

#[async_trait]
pub trait DbWriter {
    type Error: StdError + Send + 'static;
    async fn insert<D: Serialize>(
        &self,
        namespace: &str,
        data: D,
    ) -> Result<(), Self::Error>;
}

pub async fn get<W: DbWriter>(
    writer: &W,
    imdbid: u32,
    apikey: &str,
) -> Result<(), Error>
{
    let params = Params {
        apikey: apikey,
        imdbid: ImdbId::new(imdbid)
            .ok_or(Error::InvalidImdbid(imdbid))?
    };
    let json: serde_json::Value = reqwest::get(
        params.to_url()
            .map_err(|e| Error::params(e))?
        )
        .await?
        .error_for_status()?
        .bytes()
        .await
        .map(|b| serde_json::from_slice(&b))??;
    writer.insert("omdb", json)
        .await
        .map_err(|e| Error::writer(e))
}

struct ImdbId(u32);

struct Params<'a> {
    imdbid: ImdbId,
    apikey: &'a str,
}

impl ImdbId {
    fn new(id: u32) -> Option<Self> {
        if id < 100_000_000 {
            Some(Self(id))
        }
        else {
            None
        }
    }
}

impl ToString for ImdbId {
    fn to_string(&self) -> String {
        format!("tt{:07}", self.0)
    }
}

impl<'a> Params<'a> {
    fn to_url(&self) -> Result<Url, impl StdError + Send + 'static> {
        Url::parse_with_params("https://www.omdbapi.com/", &[
            ("plot", "full"),
            ("r", "json"),
            ("i", &self.imdbid.to_string()),
            ("apikey", self.apikey),
        ])
    }
}