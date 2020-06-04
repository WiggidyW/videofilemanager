use std::{error::Error as StdError, fmt::{Display, self}, convert::TryFrom, path::Path};
use rocket::response::{Responder, NamedFile};
use derive_more::From;
use serde_json::Value as Json;
use serde::Serialize;
use lazy_static::lazy_static;
use crate::core;

#[derive(Responder)]
pub enum Response {
    #[response(status = 500, content_type = "json")]
    InternalError(String),
    #[response(status = 404, content_type = "json")]
    NotFoundError(String),
    #[response(status = 200)]
    File(NamedFile),
    #[response(status = 200)]
    Streams(String),
    #[response(status = 200)]
    Okay(&'static str),
}

impl Response {
    pub fn internal_error(value: &core::Error) -> Self {
        Self::from(value)
    }
    pub fn file(value: &Path) -> Self {
        Self::from(value)
    }
    pub fn not_found_error(value: impl Serialize) -> Self {
        let field = Field {
            content: Json::Null,
            error: value,
            status: true,
        };
        Self::NotFoundError(field.to_json())
    }
    pub fn file_not_found_error(alias: &str) -> Self {
        Self::not_found_error(format!(
            "The alias '{}' has no associated file!", alias
        ))
    }
    pub fn streams(value: &[String]) -> Self {
        let field = Field {
            content: value,
            error: Json::Null,
            status: true,
        };
        Self::Streams(field.to_json())
    }
    pub fn okay() -> Self {
        lazy_static! {
            static ref OKAY: String = serde_json::to_string(
                &Field {
                    content: Json::Null,
                    error: Json::Null,
                    status: true,
                }
            ).unwrap();
        }
        Self::Okay(&OKAY)
    }
}

#[derive(Serialize)]
struct Field<T, E> {
    content: T,
    error: E,
    status: bool,
}

#[derive(Serialize)]
struct InternalError {
    display: String,
    debug: String,
    source: Option<Box<InternalError>>,
}

struct Streams {

}

impl From<&core::Error> for Response {
    fn from(value: &core::Error) -> Self {
        let field: Field<Json, InternalError> = Field {
            content: Json::Null,
            error: InternalError::from(value),
            status: true,
        };
        Self::InternalError(field.to_json())
    }
}

impl From<&Path> for Response {
    fn from(value: &Path) -> Self {
        match NamedFile::open(value) {
            Ok(f) => Self::File(f),
            Err(e) => Self::internal_error(&core::Error::FilesystemError(e)),
        }
    }
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

impl<T: StdError + ?Sized> From<&T> for InternalError {
    fn from(value: &T) -> Self {
        Self {
            display: value.to_string(),
            debug: format!("{:?}", value),
            source: value.source().map(|e| Box::new(Self::from(e))),
        }
    }
}

// impl InternalError {
//     fn from_
// }

// #[derive(Debug)]
// struct InternalError<'a>(&'a core::Error);

// impl From<&core::Error> for Error {
//     fn from(value: &core::Error) -> Self {
//         let s = InternalError::from(value)
//             .to_string();
//         Self::InternalError(s)
//     }
// }

// impl<'a> From<&'a core::Error> for InternalError<'a> {
//     fn from(value: &'a core::Error) -> Self {
//         Self(value)
//     }
// }

// impl Display for InternalError<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         fn recurse(e: &(impl StdError + ?Sized)) -> String {
//             format!("{{ \"{}\": \"{}\", \"{}\": \"{:?}\", \"{}\": {{{}}} }}",
//                 "display",
//                 {
//                     let old = e.to_string();
//                     let mut new = String::with_capacity(old.len());
//                     old.chars().for_each(|c| match c 
//                     {
//                         '\n' => new.push_str("\\n"),
//                         '\t' => new.push_str("\\t"),
//                         '"' => new.push_str("\\\""),
//                         '\r' => new.push_str("\\r"),
//                         _ => new.push(c),
//                     });
//                     new
//                 },
//                 "debug",
//                 e,
//                 "source",
//                 {
//                     match e.source() {
//                         Some(src) => recurse(src),
//                         None => " ".to_string(),
//                     }
//                 },
//             )
//         }
//         write!(f, "{}", recurse(self.0))
//     }
// }