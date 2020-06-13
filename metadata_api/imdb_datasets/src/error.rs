use std::error::Error as StdError;
use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error<W: writer::Writer> {
    StreamError(std::io::Error),
    WriterError(writer::MongoError),
}