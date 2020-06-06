use std::{io::{self, Read}, path::{Path, PathBuf}, sync::RwLock, collections::HashMap};
use lazy_static::lazy_static;

lazy_static! {
    static ref FLOCK: RwLock<HashMap<PathBuf, RwLock<()>>> = RwLock::new(
        HashMap::new()
    );
}

#[derive(Debug)]
pub struct Error {}

pub fn mux_file(
    source: impl Read,
    target: impl AsRef<Path>,
) -> Result<(), Error>
{
    unimplemented!()
}

pub fn try_hash_file(
    path: impl AsRef<Path>,
) -> Result<Vec<String>, Error>
{
    unimplemented!()
}