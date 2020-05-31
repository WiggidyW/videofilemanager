#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

mod error;
mod core;
pub mod request;
pub use crate::core::{Requestor, Operator};
pub use crate::error::{Error, HtmlError};
pub mod response;