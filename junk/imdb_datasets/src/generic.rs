use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::convert::TryFrom;
use bincode;
use chrono;

type DateTime = chrono::DateTime<chrono::offset::Utc>;

pub trait Request {
    type Error;
    type Response;
    fn request() -> Result<Self::Response, Self::Error>;
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct RawMetadata<RAW> {
    pub timestamp: DateTime,
    pub data: RAW,
}

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Metadata<M> {
    timestamp: DateTime,
    data: M,
}

#[derive(Debug)]
pub enum Error<T, U> {
    RequestError(T),
    ParseError(U),
}

impl<RAW> From<RAW> for RawMetadata<RAW> {
    fn from(value: RAW) -> Self {
        Self {
            timestamp: chrono::offset::Utc::now(),
            data: value,
        }
    }
}

impl<M, RAW> TryFrom<RawMetadata<RAW>> for Metadata<M> where
    M: TryFrom<RAW>,
{
    type Error = <M as TryFrom<RAW>>::Error;
    fn try_from(value: RawMetadata<RAW>) -> Result<Self, Self::Error> {
        match <M as TryFrom<RAW>>::try_from(value.data) {
            Ok(v) => Ok(Self { timestamp: value.timestamp, data: v }),
            Err(e) => Err(e),
        }
    }
}

impl<M> Request for Metadata<M> where
    M: Request,
{
    type Error = <M as Request>::Error;
    type Response = RawMetadata<<M as Request>::Response>;
    fn request() -> Result<Self::Response, Self::Error> {
        match <M as Request>::request() {
            Ok(v) => Ok(RawMetadata::from(v)),
            Err(e) => Err(e),
        }
    }
}

type RequestError<T> = <T as Request>::Error;
type ParseError<T> = <T as TryFrom<<T as Request>::Response>>::Error;
impl<M> Metadata<M> where
    M: TryFrom<<M as Request>::Response>,
    M: DeserializeOwned,
    M: Serialize,
    M: Request,
{
    pub fn data(&self) -> &M {
        &self.data
    }
    pub fn timestamp(&self) -> &DateTime {
        &self.timestamp
    }
    pub fn new() -> Result<Self, Error<RequestError<Self>, ParseError<Self>>> {
        match Self::request() {
            Err(e) => Err(Error::RequestError(e)),
            Ok(response) =>
        match Self::try_from(response) {
            Err(e) => Err(Error::ParseError(e)),
            Ok(self_) => Ok(self_)
        }}
    }
    pub fn deserialize_from<R: std::io::Read>(reader: R) -> bincode::Result<Self> {
        bincode::deserialize_from(reader)
    }
    pub fn serialize_into<W: std::io::Write>(&self, writer: W) -> bincode::Result<()> {
        bincode::serialize_into(writer, self)
    }
}