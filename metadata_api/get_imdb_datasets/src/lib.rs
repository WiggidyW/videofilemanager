#![allow(dead_code)]
#![allow(unused_imports)]

pub(crate) const CSV_TERM: &'static u8 = &b'\t';

pub mod error;

pub(crate) mod core;

mod stream;

mod model;