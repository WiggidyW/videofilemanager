use std::error::Error as StdError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    RequestError(reqwest::Error),
    WriterError(WriterError),
    ParamsError(ParamsError),
    InvalidImdbid(
        #[error(not(source))]
        u32
    ),
    JsonError(serde_json::Error),
}

impl Error {
    pub(crate) fn writer(e: impl StdError + Send + 'static) -> Self {
        Self::WriterError(WriterError(Box::new(e)))
    }
    pub(crate) fn params(e: impl StdError + Send + 'static) -> Self {
        Self::ParamsError(ParamsError(Box::new(e)))
    }
}

#[derive(Debug, Display)]
pub struct ParamsError(Box<dyn StdError + Send + 'static>);

#[derive(Debug, Display)]
pub struct WriterError(Box<dyn StdError + Send + 'static>);

impl StdError for ParamsError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.0)
    }
}

impl StdError for WriterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.0)
    }
}