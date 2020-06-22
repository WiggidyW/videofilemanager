use lazy_static::lazy_static;
use std::sync::RwLock;
use std::collections::HashMap;
use derive_more::{Error, Display, From};

#[derive(Clone, Copy)]
pub struct ImdbId(u32);

lazy_static! {
    static ref IMDBIDS: RwLock<HashMap<u32, bool>> = RwLock::new(HashMap::new());
}

#[derive(Debug, Display, Error, From)]
pub enum ValidationError {
    RequestError(reqwest::Error),
    StatusCodeError(
        #[from(forward)]
        StatusCodeError
    ),
}

#[derive(Debug, Display, Error)]
pub enum StatusCodeError {
    ServerError(reqwest::Error),
    ClientError(reqwest::Error),
    #[display(fmt = "{:?}", "_0")]
    Other(
        #[error(not(source))]
        reqwest::Response
    ),
}

impl ImdbId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }
    pub async fn is_valid(&self, client: &reqwest::Client) -> Result<bool, ValidationError> {
        if self.0 >= 100_000_000 {
            return Ok(false);
        }
        if let Some(&boolean) = IMDBIDS.read().unwrap().get(&self.0) {
            return Ok(boolean);
        }
        match client
            .head(&format!("https://www.imdb.com/title/tt{:0>7}/", self.0))
            .send()
            .await?
        {
            r if r.status().is_success() => {
                IMDBIDS.write().unwrap().insert(self.0, true);
                Ok(true)
            },
            r if r.status().as_u16() == 404 => {
                IMDBIDS.write().unwrap().insert(self.0, false);
                Ok(false)
            },
            r => Err(ValidationError::from(r)),
        }
    }
    pub fn is_valid_cached(&self) -> Option<bool> {
        if self.0 >= 100_000_000 {
            return Some(false);
        }
        IMDBIDS.read()
            .unwrap()
            .get(&self.0)
            .map(|boolean| *boolean)
    }
    pub fn force_valid_cached(&self) {
        IMDBIDS.write()
            .unwrap()
            .insert(self.0, true);
    }
    pub fn to_raw_string(&self) -> String {
        self.0.to_string()
    }
}

impl AsRef<u32> for ImdbId {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl ToString for ImdbId {
    fn to_string(&self) -> String {
        format!("tt{0:7}", self.0)
    }
}

impl From<reqwest::Response> for StatusCodeError {
    fn from(value: reqwest::Response) -> Self {
        match value.status() {
            s if s.is_server_error() => Self::ServerError(value.error_for_status().unwrap_err()),
            s if s.is_client_error() => Self::ClientError(value.error_for_status().unwrap_err()),
            _ => Self::Other(value),
        }
    }
}