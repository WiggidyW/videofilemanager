use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub enum Error {
    RequestError(reqwest::Error),
    StreamError(std::io::Error),
    WriterError(writer::Error),
    AsyncJoinError(tokio::task::JoinError)
}