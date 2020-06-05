use std::{
    path::{Path, PathBuf},
    time::SystemTime,
    error::Error as StdError,
    fmt::{Display, Formatter, Error as FmtError, Debug},
    io::Read,
};

#[derive(Debug)]
pub enum Error {
    FileMapError(Box<dyn StdError>),
    CacheError(Box<dyn StdError>),
    DatabaseError(Box<dyn StdError>),
    FileSystemError(std::io::Error),
    SystemTimeError(std::time::SystemTimeError),
    Infallible(Option<&'static str>),
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