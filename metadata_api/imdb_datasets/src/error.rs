use std::error::Error as StdError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    RequestError(reqwest::Error),
    StreamError(std::io::Error),
    WriterError(WriterError),
    AsyncJoinError(tokio::task::JoinError),
    Utf8Error(std::str::Utf8Error),
}

impl Error {
    pub(crate) fn writer(e: impl StdError + Send + 'static) -> Self {
        Self::WriterError(WriterError(Box::new(e)))
    }
}

#[derive(Debug, Display)]
pub struct WriterError(Box<dyn StdError + Send + 'static>);

impl StdError for WriterError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&*self.0)
    }
}