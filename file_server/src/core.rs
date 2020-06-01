use std::{path::Path, time::SystemTime, error::Error as StdError, fmt::{Display, Formatter, Error as FmtError, Debug}};
use crate::{Cache, FileMap, Database};

#[derive(Debug)]
pub enum Error {
    FileMapError(Box<dyn StdError>, Option<&'static str>),
    CacheError(Box<dyn StdError>, Option<&'static str>),
    DatabaseError(Box<dyn StdError>, Option<&'static str>),
    FilesystemError(std::io::Error, Option<&'static str>),
    SystemTimeError(std::time::SystemTimeError),
    Infallible(Option<&'static str>),
}

impl Error {
    fn database_err(e: impl StdError + 'static, s: Option<&'static str>) -> Self {
        Self::DatabaseError(Box::new(e), s)
    }
    fn cache_err(e: impl StdError + 'static, s: Option<&'static str>) -> Self {
        Self::CacheError(Box::new(e), s)
    }
    fn filemap_err(e: impl StdError + 'static, s: Option<&'static str>) -> Self {
        Self::FileMapError(Box::new(e), s)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Ok(())
    }
}

impl StdError for Error {}

impl From<crate::media_mixer::Error> for Error {
    fn from(value: crate::media_mixer::Error) -> Self {
        unimplemented!()
    }
}

pub fn list_aliases<D: Database>(
    database: &D,
) -> Result<Vec<Vec<String>>, Error>
{
    database.list_aliases()
        .map_err(|e| Error::database_err(e, None))
}

// return None:
//   - new_alias Already Exists.
pub fn add_alias<D: Database>(
    alias: &str,
    new_alias: &str,
    database: &D,
) -> Result<Option<()>, Error>
{
    let file_id = get_file_id(alias, database)?;
    database.create_alias(new_alias, file_id)
        .map_err(|e| Error::database_err(e, None))
}
// return None:
//   - alias Does Not Exist.
pub fn remove_alias<D: Database>(
    alias: &str,
    database: &D,
) -> Result<Option<()>, Error>
{
    database.remove_alias(alias)
        .map_err(|e| Error::database_err(e, None))
}

pub fn get_aliases<D: Database>(
    alias: &str,
    database: &D,
) -> Result<Vec<String>, Error>
{
    let file_id = get_file_id(alias, database)?;
    match database.get_aliases(file_id)
        .map_err(|e| Error::database_err(e, None))?
    {
        Some(aliases) => Ok(aliases),
        None => Err(Error::Infallible(Some(
            "Created an entry in the DB, and then it vanished. This shouldn't happen."
        ))),
    }
}

pub fn add_file<F: FileMap, D: Database>(
    alias: &str,
    source: impl AsRef<Path>,
    file_map: &F,
    database: &D,
) -> Result<(), Error>
{
    let file_id = get_file_id(alias, database)?;
    let target = file_map.get(file_id)
        .map_err(|e| Error::filemap_err(e, None))?;
    mux_file(source, target)
}

fn current_time() -> Result<u64, Error> {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| Error::SystemTimeError(e))?
        .as_secs();
    Ok(time)
}

// return None:
//   - file_path from alias is not a file.
//   - cache is empty.
//   - cache is expired.
pub fn try_get_hashes<'c, F: FileMap, C: Cache, D: Database>(
    alias: &str,
    file_map: &F,
    cache: &'c C,
    database: &D,
) -> Result<Option<&'c [String]>, Error>
{
    let file_id = get_file_id(alias, database)?;
    let file_path = file_map.get(file_id)
        .map_err(|e| Error::filemap_err(e, None))?;
    if !file_path.is_file() {
        return Ok(None);
    }
    try_hash_cache(file_id, file_path, cache)
}

pub fn refresh_hashes<F: FileMap, C: Cache, D: Database>(
    alias: &str,
    file_map: &F,
    cache: &mut C,
    database: &D,
) -> Result<Vec<String>, Error>
{
    let file_id = get_file_id(alias, database)?;
    let file_path = file_map.get(file_id)
        .map_err(|e| Error::filemap_err(e, None))?;
    let hashes = try_hash_file(file_path)?;
    let current_time = current_time()?;
    cache.set(file_id, &hashes, current_time)
        .map_err(|e| Error::cache_err(e, None))?;
    Ok(hashes)
}

fn file_time(
    path: impl AsRef<Path>,
) -> Result<u64, Error>
{
    let time = path
        .as_ref()
        .metadata()
        .map_err(|e| Error::FilesystemError(e, None))?
        .modified()
        .map_err(|e| Error::FilesystemError(e, None))?
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| Error::SystemTimeError(e))?
        .as_secs();
    Ok(time)
}

fn get_file_id<D: Database>(
    alias: &str,
    database: &D,
) -> Result<u32, Error>
{
    if let Some(file_id) = database.get_file_id(alias)
        .map_err(|e| Error::database_err(e, None))?
    {
        return Ok(file_id);
    }
    let file_id = database.create_file_id()
        .map_err(|e| Error::database_err(e, None))?;
    database.create_alias(alias, file_id)
        .map_err(|e| Error::database_err(e, None))?;
    Ok(file_id)
}

fn try_hash_file(
    path: impl AsRef<Path>,
) -> Result<Vec<String>, Error>
{
    Ok(crate::media_mixer::try_hash_file(path)?)
}

// return None:
//   - cache is empty.
//   - cache is expired.
fn try_hash_cache<'c, C: Cache>(
    file_id: u32,
    path: impl AsRef<Path>,
    cache: &'c C,
) -> Result<Option<&'c [String]>, Error>
{
    match cache.get(file_id)
        .map_err(|e| Error::cache_err(e, None))?
    {
        None => Ok(None),
        Some((h, t)) if t < file_time(path)? => Ok(Some(h)),
        Some(_) => Ok(None),
    }
}

fn mux_file(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> Result<(), Error>
{
    Ok(crate::media_mixer::mux_file(source, target)?)
}