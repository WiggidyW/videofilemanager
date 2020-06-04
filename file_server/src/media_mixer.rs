use std::{io::Read, path::Path};

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