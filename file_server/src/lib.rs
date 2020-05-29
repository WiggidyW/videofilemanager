use std::{error::Error as StdError, path::PathBuf};

pub trait FileMap {
    type Reader: Read + 'static;
    fn get(key: &str) -> Option<PathBuf>;
}

pub trait Cache {
    type Error: StdError + 'static;
}