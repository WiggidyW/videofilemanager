use std::{error::Error as StdError, fmt::{Display, Formatter, Error as FmtError, Debug}};

#[derive(Debug)]
pub enum Error<F, C, D> {
    FileMapError(F),
    CacheError(C),
    DatabaseError(D),
}

impl<F, C, D> Display for Error<F, C, D>
where
    F: StdError,
    C: StdError,
    D: StdError,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Ok(())
    }
}

impl<F, C, D> StdError for Error<F, C, D>
where
    F: StdError,
    C: StdError,
    D: StdError,
{}