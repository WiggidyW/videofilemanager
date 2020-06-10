use std::{error::Error as StdError, fmt::{Display, self}, convert::TryFrom, path::Path, collections::HashMap, io::BufReader, hash::Hash};
use rocket::{response::{Response, Responder, NamedFile, Body}, http::ContentType};
use serde_json::Value as Json;
use lazy_static::lazy_static;
use serde::Serialize;
use crate::core;

#[derive(Serialize)]
struct Field<T, E> {
    content: T,
    error: E,
    status: bool,
}

#[derive(Responder)]
pub enum Content {
    #[response(status = 200, content_type = "json")]
    Okay(&'static str),
    #[response(status = 200, content_type = "json")]
    StreamHashes(String),
    #[response(status = 200, content_type = "json")]
    AliasList(String),
    #[response(status = 200, content_type = "json")]
    JsonProbe(String),
}

pub struct FileContent<'r> {
    len: Option<u64>,
    ext: Option<String>,
    reader: Box<dyn std::io::Read + 'r>,
}

#[derive(Debug, Responder)]
pub enum Error {
    #[response(status = 500, content_type = "json")]
    InternalError(String),
    #[response(status = 404, content_type = "json")]
    FileNotFoundError(&'static str),
    #[response(status = 404, content_type = "json")]
    AliasNotFoundError(&'static str),
    #[response(status = 404, content_type = "json")]
    IDNotFoundError(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidAliasRemoval(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidAliasAddition(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidStreamHashes(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidFile(&'static str),
}

#[derive(Serialize)]
struct InternalError {
    display: String,
    debug: String,
    source: Option<Box<InternalError>>,
}

impl<T: Serialize, E: Serialize> Field<T, E> {
    fn to_json(&self) -> String {
        if let Ok(s) = serde_json::to_string(self) {
            return s;
        }
        lazy_static! {
            static ref ERR: String = serde_json::to_string(
                &Field {
                    content: Json::Null,
                    error: Json::Null,
                    status: false,
                }
            ).unwrap();
        }
        ERR.to_string()
    }
}

impl Content {
    pub fn okay() -> Self {
        lazy_static! { static ref CON: String = Field {
            content: Json::Null,
            error: Json::Null,
            status: true,
        }.to_json(); }
        Self::Okay(&CON)
    }
    pub fn stream_hashes<'a>(
        iter: impl Iterator<Item = (
            impl Serialize + Hash + Eq,
            &'a [String],
        )>
    ) -> Self
    {
        Self::StreamHashes(Field {
            content: iter.collect::<HashMap<_, _>>(),
            error: Json::Null,
            status: true,
        }.to_json())
    }
    pub fn alias_list(
        iter: impl Iterator<Item = (
            impl Serialize + Hash + Eq,
            Vec<String>,
        )>
    ) -> Self
    {
        Self::AliasList(Field {
            content: iter.collect::<HashMap<_, _>>(),
            error: Json::Null,
            status: true,
        }.to_json())
    }
    pub fn json_probe(json: Json) -> Self {
        Self::JsonProbe(Field {
            content: json,
            error: Json::Null,
            status: true,
        }.to_json())
    }
    fn str_content(c: &str) -> String {
        Field {
            content: c,
            error: Json::Null,
            status: true,
        }.to_json()
    }
}

impl<'r> FileContent<'r> {
    pub fn new(value: core::ROFile<'r>) -> Result<Option<Self>, core::Error> {
        Option::<Self>::try_from(value)
    }
}

impl<'r> TryFrom<core::ROFile<'r>> for Option<FileContent<'r>> {
    type Error = core::Error;
    fn try_from(value: core::ROFile<'r>) -> Result<Self, Self::Error> {
        let len = value.len()?;
        let ext = value.extension();
        match value.into_read() {
            Err(e) => Err(e),
            Ok(None) => Ok(None),
            Ok(Some(r)) => Ok(Some(FileContent {
                len: len,
                ext: ext,
                reader: Box::new(r),
            })),
        }
    }
}

impl<'r> Responder<'r> for FileContent<'r> {
    fn respond_to(self, _: &rocket::Request) -> rocket::response::Result<'r> {
        let file = BufReader::new(self.reader);
        let mut response = Response::build();
        match self.len {
            Some(i) => response.raw_body(Body::Sized(file, i)),
            None => response.streamed_body(file),
        };
        if let Some(ext) = self.ext {
            if let Some(ct) = ContentType::from_extension(&ext) {
                response.header(ct);
            }
        }
        response.ok()
    }
}

impl Error {
    pub fn internal(value: core::Error) -> Self {
        Self::InternalError(Field {
            content: Json::Null,
            error: InternalError::from(&value),
            status: true,
        }.to_json())
    }
    fn str_err(e: &str) -> String {
        Field {
            content: Json::Null,
            error: e,
            status: true,
        }.to_json()
    }
    pub fn file_not_found() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
                "The requested file could not be found."
        ); }
        Self::FileNotFoundError(&ERR)
    }
    pub fn alias_not_found() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The requested file could not be found."
        ); }
        Self::AliasNotFoundError(&ERR)
    }
    pub fn id_not_found() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The requested file could not be found."
        ); }
        Self::IDNotFoundError(&ERR)
    }
    pub fn invalid_alias_removal() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The requested file could not be found."
        ); }
        Self::InvalidAliasRemoval(&ERR)
    }
    pub fn invalid_alias_addition() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The requested file could not be found."
        ); }
        Self::InvalidAliasAddition(&ERR)
    }
    pub fn invalid_stream_hashes() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The provided stream hash values were invalid."
        ); }
        Self::InvalidStreamHashes(&ERR)
    }
    pub fn invalid_file() -> Self {
        lazy_static! { static ref ERR: String = Error::str_err(
            "The provided file was found to be invalid."
        ); }
        Self::InvalidFile(&ERR)
    }
}

impl<T: StdError + ?Sized> From<&T> for InternalError {
    fn from(value: &T) -> Self {
        Self {
            display: value.to_string(),
            debug: format!("{:?}", value),
            source: value.source().map(|e| Box::new(Self::from(e))),
        }
    }
}