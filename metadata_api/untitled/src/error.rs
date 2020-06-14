use derive_more::{Display, Error, From};
use std::error::Error as StdError;

#[derive(Debug, Display, Error, From)]
pub enum Error<E> {
    RequestError(reqwest::Error),
    #[from(ignore)]
    ResponseCodeError(reqwest::Error),
    ParamsError(ParamsError),
    ResponseError(ResponseError),
    #[from(ignore)]
    DatabaseError(E),
    SystemTImeError(std::time::SystemTimeError),
}

#[derive(Debug, Display)]
#[display(fmt = "parameter '{}' had invalid value '{}'", "key", "val")]
pub struct ParamsError {
    source: Option<Box<dyn StdError + Send + 'static>>,
    key: String,
    val: String,
}

#[derive(Debug, Display)]
#[display(fmt = "server response error of kind '{:?}': '{}'", "kind", "text")]
pub struct ResponseError {
    source: Option<Box<dyn StdError + Send + 'static>>,
    kind: ResponseErrorKind,
    text: String,
}

#[derive(Debug)]
pub enum ResponseErrorKind {
    Authentication,
    InvalidJson,
    Other,
}

impl ParamsError {
    pub(crate) fn new(
        key: impl ToString,
        val: impl ToString,
        e: Option<impl StdError + Send + 'static>,
    ) -> Self
    {
        Self {
            source: e.map(|err|
                Box::new(err) as Box<dyn StdError + Send + 'static>),
            key: key.to_string(),
            val: val.to_string(),
        }
    }
}

impl ResponseError {
    pub(crate) fn new(
        text: impl ToString,
        kind: ResponseErrorKind,
        e: Option<impl StdError + Send + 'static>,
    ) -> Self
    {
        Self {
            source: e.map(|err|
                Box::new(err) as Box<dyn StdError + Send + 'static>),
            kind: kind,
            text: text.to_string(),
        }
    }
}

impl StdError for ParamsError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}

impl StdError for ResponseError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref()
            .map(|e| e.as_ref() as &(dyn StdError + 'static))
    }
}