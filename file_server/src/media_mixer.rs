use std::path::Path;

#[derive(Debug)]
pub struct Error {}

pub fn mux_file(
    source: impl AsRef<Path>,
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