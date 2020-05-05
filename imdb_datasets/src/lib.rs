mod imdb_dataset;

use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::convert::TryFrom;
use bincode;
use chrono;

type DateTime = chrono::DateTime<chrono::offset::Utc>;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct MetadataRes<T> {
    timestamp: DateTime,
    inner: T,
}

pub trait MetadataRequest: Sized {
    type Error;
    fn request() -> Result<Self, Self::Error>;
}

impl<T> From<T> for MetadataRes<T> where
    T: MetadataRequest,
{
    fn from(value: T) -> Self {
        Self {
            timestamp: chrono::offset::Utc::now(),
            inner: value,
        }
    }
}

pub enum Error<T, U> {
    RequestError(T),
    ParseError(U),
}

type RequestError<T> = <<T as Metadata>::Request as MetadataRequest>::Error;
type ParseError<T> = <T as TryFrom<MetadataRes<<T as Metadata>::Request>>>::Error;
pub trait Metadata:
    TryFrom<MetadataRes<<Self as Metadata>::Request>> +
    Serialize +
    DeserializeOwned
{
    type Request: MetadataRequest;
    fn timestamp(&self) -> &DateTime;
    fn request() -> Result<MetadataRes<Self::Request>, RequestError<Self>> {
        Ok(MetadataRes::from(<Self::Request as MetadataRequest>::request()?))
    }
    fn new() -> Result<Self, Error<RequestError<Self>, ParseError<Self>>> {
        match Self::request() {
            Err(e) => Err(Error::RequestError(e)),
            Ok(res) => 
        match <Self as TryFrom<MetadataRes<Self::Request>>>::try_from(res) {
            Err(e) => Err(Error::ParseError(e)),
            Ok(self_) => Ok(self_),
        }}
    }
    fn deserialize_from<R: std::io::Read>(reader: R) -> bincode::Result<Self> {
        bincode::deserialize_from(reader)
    }
    fn serialize_into<W: std::io::Write>(&self, writer: W) -> bincode::Result<()> {
        bincode::serialize_into(writer, self)
    }
}