use std::{error::Error as StdError, fmt::{Display, Formatter, Error as FmtError, Debug}};

#[derive(Debug)]
pub enum Error {
    FileMapError(Box<dyn StdError>),
    CacheError(Box<dyn StdError>),
    DatabaseError(Box<dyn StdError>),
    FileSystemError(std::io::Error),
    SystemTimeError(std::time::SystemTimeError),
    Infallible(Option<&'static str>),
    StreamHashesNotFound,
    FileNotFound,
    IdNotFound(u32),
    AliasNotFound(String),
    AliasesAlreadyExist,
    AliasDoesNotMatchId(String, u32),
    InvalidMediaFile,
}

impl Error {
    pub(crate) fn file_map_err(e: impl StdError + 'static) -> Self {
        Self::FileMapError(Box::new(e))
    }
    pub(crate) fn database_err(e: impl StdError + 'static) -> Self {
        Self::DatabaseError(Box::new(e))
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::FileMapError(e) => Some(&**e),
            Self::CacheError(e) => Some(&**e),
            Self::DatabaseError(e) => Some(&**e),
            Self::FileSystemError(e) => Some(e),
            Self::SystemTimeError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<super::media_mixer::Error> for Error {
    fn from(value: super::media_mixer::Error) -> Self {
        unimplemented!()
    }
}