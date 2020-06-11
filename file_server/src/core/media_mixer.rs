use std::{io::{self, Read}, path::{Path, PathBuf}, sync::RwLock, collections::HashMap, process::{Command, Stdio}};
use lazy_static::lazy_static;

lazy_static! {
    static ref FLOCK: RwLock<HashMap<PathBuf, RwLock<()>>> = RwLock::new(
        HashMap::new()
    );
}

#[derive(Debug)]
pub enum Error {
    ProcessError(io::Error),
    InvalidOutput(serde_json::Error),
}

pub fn mux_file(
    source: impl Read,
    target: impl AsRef<Path>,
) -> Result<Option<()>, Error>
{
    unimplemented!()
}

pub fn partial_demux_file(
    target_hashes: &[usize],
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

pub fn json_probe(
    path: impl AsRef<Path>,
) -> Result<serde_json::Value, Error>
{
    let bytes = Command::new("ffprobe")
        .arg(path.as_ref())
        .arg("-loglevel").arg("quiet")
        .arg("-show_versions")
        .arg("-show_format")
        .arg("-show_streams")
        .arg("-show_chapters")
        .arg("-of").arg("json='c=0'")
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| Error::ProcessError(e))?
        .wait_with_output()
        .map_err(|e| Error::ProcessError(e))?
        .stdout;
    serde_json::from_slice(&bytes).map_err(|e| Error::InvalidOutput(e))
}