use std::{error::Error as StdError, convert::TryFrom, collections::HashMap, io::BufReader, hash::Hash};
use rocket::{response::{Response, Responder, Body}, http::ContentType};
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
    #[response(status = 400, content_type = "json")]
    FileNotFound(&'static str),
    #[response(status = 400, content_type = "json")]
    AliasNotFound(String),
    #[response(status = 400, content_type = "json")]
    IdNotFound(String),
    #[response(status = 400, content_type = "json")]
    StreamHashesNotFound(&'static str),
    #[response(status = 400, content_type = "json")]
    InvalidMediaFile(&'static str),
    #[response(status = 400, content_type = "json")]
    AliasesAlreadyExist(&'static str),
    #[response(status = 400, content_type = "json")]
    AliasDoesNotMatchId(String),
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
    pub fn stream_hashes<'a>(iter: impl Iterator<Item = (
        impl Serialize + Hash + Eq,
        &'a [String],
    )>) -> Self
    {
        Self::StreamHashes(Field {
            content: iter.collect::<HashMap<_, _>>(),
            error: Json::Null,
            status: true,
        }.to_json())
    }
    pub fn alias_list(iter: impl Iterator<Item = (
        impl Serialize + Hash + Eq,
        Vec<String>,
    )>) -> Self
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
}

impl<'r> FileContent<'r> {
    pub fn new(value: core::ROFile<'r>) -> Result<Self, core::Error> {
        Self::try_from(value)
    }
}

impl<'r> TryFrom<core::ROFile<'r>> for FileContent<'r> {
    type Error = core::Error;
    fn try_from(value: core::ROFile<'r>) -> Result<Self, Self::Error> {
        let len = value.len()?;
        let ext = value.extension();
        match value.into_read() {
            Err(e) => Err(e),
            Ok(r) => Ok(FileContent {
                len: len,
                ext: ext,
                reader: Box::new(r),
            }),
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

impl From<core::Error> for Error {
    fn from(value: core::Error) -> Self {
        
        #[derive(Serialize)]
        struct ErrorInner<T> {
            content: T,
            text: &'static str,
            kind: &'static str,
        }
        #[derive(Serialize)]
        struct IdNotFound {
            id: u32,
        }
        #[derive(Serialize)]
        struct AliasNotFound {
            alias: String,
        }
        #[derive(Serialize)]
        struct BothNotFound {
            id: u32,
            alias: String,
        }

        lazy_static! {
            static ref STREAM_HASHES_NOT_FOUND: String = Field {
                content: Json::Null,
                error: ErrorInner {
                    content: Json::Null,
                    text: "All provided stream hash values could not be located.",
                    kind: "StreamHashesNotFound",
                },
                status: true
            }.to_json();
            static ref FILE_NOT_FOUND: String = Field {
                content: Json::Null,
                error: ErrorInner {
                    content: Json::Null,
                    text: "The requested file could not be located on the server.",
                    kind: "FileNotFound",
                },
                status: true
            }.to_json();
            static ref INVALID_MEDIA_FILE: String = Field {
                content: Json::Null,
                error: ErrorInner {
                    content: Json::Null,
                    text: "The provided file was not recognized as media by the server.",
                    kind: "InvalidMediaFile",
                },
                status: true
            }.to_json();
            static ref ALIASES_ALREADY_EXIST: String = Field {
                content: Json::Null,
                error: ErrorInner {
                    content: Json::Null,
                    text: "At least one of the provided aliases already exist on the server.",
                    kind: "AliasesAlreadyExist",
                },
                status: true
            }.to_json();
        }
        
        match value {
            core::Error::StreamHashesNotFound => Self::StreamHashesNotFound(
                &STREAM_HASHES_NOT_FOUND
            ),
            core::Error::FileNotFound => Self::FileNotFound(&FILE_NOT_FOUND),
            core::Error::IdNotFound(id) => Self::IdNotFound(Field {
                content: Json::Null,
                error: ErrorInner {
                    content: IdNotFound { id: id },
                    text: "The provided ID did not exist on the server.",
                    kind: "IDNotFound",
                },
                status: true,
            }.to_json()),
            core::Error::AliasNotFound(alias) => Self::AliasNotFound(Field {
                content: Json::Null,
                error: ErrorInner {
                    content: AliasNotFound { alias: alias },
                    text: "The provided Alias did not exist on the server.",
                    kind: "AliasNotFound",
                },
                status: true,
            }.to_json()),
            core::Error::AliasesAlreadyExist => Self::AliasesAlreadyExist(
                &ALIASES_ALREADY_EXIST
            ),
            core::Error::AliasDoesNotMatchId(alias, id) => Self::AliasDoesNotMatchId(Field {
                content: Json::Null,
                error: ErrorInner {
                    content: BothNotFound { id: id, alias: alias },
                    text: "The provided Alias matches a different id.",
                    kind: "AliasDoesNotMatchID",
                },
                status: true,
            }.to_json()),
            core::Error::InvalidMediaFile => Self::InvalidMediaFile(
                &INVALID_MEDIA_FILE
            ),
            any => Self::InternalError(Field {
                content: Json::Null,
                error: ErrorInner {
                    content: InternalError::from(&any),
                    text: "Something went wrong!",
                    kind: "Internal",
                },
                status: true,
            }.to_json()),
        }
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