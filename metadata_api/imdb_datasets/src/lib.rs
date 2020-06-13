mod stream;
mod kind;
mod write;
mod error;

pub use error::Error;
pub(crate) use kind::Dataset;
pub(crate) use stream::request_stream;