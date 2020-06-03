use rocket::response::Responder;
use crate::core;
use std::{error::Error as StdError, fmt::{Display, self}};

struct Field {
    
}

#[derive(Responder)]
enum Error {
    #[response(status = 500, content_type = "json")]
    InternalError(String),
    // #[response(status = 404, content_type = "json")]
}

#[derive(Debug)]
struct InternalError<'a>(&'a core::Error);

impl From<&core::Error> for Error {
    fn from(value: &core::Error) -> Self {
        let s = InternalError::from(value)
            .to_string();
        Self::InternalError(s)
    }
}

impl<'a> From<&'a core::Error> for InternalError<'a> {
    fn from(value: &'a core::Error) -> Self {
        Self(value)
    }
}

impl Display for InternalError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn recurse(e: &(impl StdError + ?Sized)) -> String {
            format!("{{ \"{}\": \"{}\", \"{}\": \"{:?}\", \"{}\": {{{}}} }}",
                "display",
                {
                    let old = e.to_string();
                    let mut new = String::with_capacity(old.len());
                    old.chars().for_each(|c| match c 
                    {
                        '\n' => new.push_str("\\n"),
                        '\t' => new.push_str("\\t"),
                        '"' => new.push_str("\\\""),
                        '\r' => new.push_str("\\r"),
                        _ => new.push(c),
                    });
                    new
                },
                "debug",
                e,
                "source",
                {
                    match e.source() {
                        Some(src) => recurse(src),
                        None => " ".to_string(),
                    }
                },
            )
        }
        write!(f, "{}", recurse(self.0))
    }
}