mod error;
mod core;
pub mod request;
pub use crate::core::{Requestor, Cache, Operator, Torrent};
pub use crate::error::{Error, HtmlError};