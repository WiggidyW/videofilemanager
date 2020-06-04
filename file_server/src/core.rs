use std::{path::{Path, PathBuf}, time::SystemTime, error::Error as StdError, fmt::{Display, Formatter, Error as FmtError, Debug}};
use crate::{Cache, FileMap, Database};

#[derive(Debug)]
pub enum Error {
    FileMapError(Box<dyn StdError>),
    CacheError(Box<dyn StdError>),
    DatabaseError(Box<dyn StdError>),
    FilesystemError(std::io::Error, Option<&'static str>),
    SystemTimeError(std::time::SystemTimeError),
    Infallible(Option<&'static str>),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FmtError> {
        Ok(())
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::FileMapError(e) => Some(&**e),
            Self::CacheError(e) => Some(&**e),
            Self::DatabaseError(e) => Some(&**e),
            Self::FilesystemError(e, _) => Some(e),
            Self::SystemTimeError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<crate::media_mixer::Error> for Error {
    fn from(value: crate::media_mixer::Error) -> Self {
        unimplemented!()
    }
}

// return None:
//   - new_alias Already Exists.
pub fn add_alias(
    alias: &str,
    new_alias: &str,
    database: &mut impl Database<Error = Error>,
) -> Result<Option<()>, Error>
{
    let file_id = get_or_create_file_id(alias, database)?;
    database.create_alias(new_alias, file_id)
}
// return None:
//   - alias Does Not Exist.
pub fn remove_alias(
    alias: &str,
    database: &mut impl Database<Error = Error>,
) -> Result<Option<()>, Error>
{
    database.remove_alias(alias)
}

// return None:
//   - alias does not exist.
//   - file_path from alias is not a file.
//   - cache is empty.
//   - cache is expired.
pub fn try_get_hashes<'c>(
    alias: &str,
    file_map: &impl FileMap<Error = Error>,
    cache: &'c impl Cache<Error = Error>,
    database: &impl Database<Error = Error>,
) -> Result<Option<&'c [String]>, Error>
{
    let file_id = match database.get_file_id(alias)?
    {
        Some(i) => i,
        None => return Ok(None),
    };
    let file_path = file_map.get(file_id)?;
    if !file_path.is_file() {
        return Ok(None);
    }
    try_hash_cache(file_id, file_path, cache)
}

// return None:
//   - alias does not exist.
//   - file_path from alias is not a file.
pub fn refresh_hashes(
    alias: &str,
    file_map: &impl FileMap<Error = Error>,
    cache: &mut impl Cache<Error = Error>,
    database: &impl Database<Error = Error>,
) -> Result<Option<Vec<String>>, Error>
{
    let file_id = match database.get_file_id(alias)?
    {
        Some(i) => i,
        None => return Ok(None),
    };
    let file_path = file_map.get(file_id)?;
    if !file_path.is_file() {
        return Ok(None);
    }
    let hashes = try_hash_file(file_path)?;
    let current_time = current_time()?;
    cache.set(file_id, &hashes, current_time)?;
    Ok(Some(hashes))
}

// return None:
//   - alias does not exist.
pub fn get_aliases(
    alias: &str,
    database: &impl Database<Error = Error>,
) -> Result<Option<Vec<String>>, Error>
{
    let file_id = match database.get_file_id(alias)?
    {
        Some(i) => i,
        None => return Ok(None),
    };
    match database.get_aliases(file_id)?
    {
        Some(aliases) => Ok(Some(aliases)),
        None => Err(Error::Infallible(Some(
            "Confirmed Alias is in DB, but it vanished. This shouldn't happen."
        ))),
    }
}

pub fn list_aliases(
    database: &impl Database<Error = Error>,
) -> Result<Vec<Vec<String>>, Error>
{
    database.list_aliases()
}

// return None:
//   - alias does not exist.
//   - file_path from alias is not a file.
pub fn get_file_path(
    alias: &str,
    file_map: &impl FileMap<Error = Error>,
    database: &impl Database<Error = Error>,
) -> Result<Option<PathBuf>, Error>
{
    let file_id = match database.get_file_id(alias)?
    {
        Some(i) => i,
        None => return Ok(None),
    };
    let file_path = file_map.get(file_id)?;
    match file_path.is_file() {
        true => Ok(Some(file_path)),
        false => Ok(None),
    }
}

// return None:
//   - cache is empty.
//   - cache is expired.
fn try_hash_cache<'c>(
    file_id: u32,
    path: impl AsRef<Path>,
    cache: &'c impl Cache<Error = Error>,
) -> Result<Option<&'c [String]>, Error>
{
    match cache.get(file_id)?
    {
        None => Ok(None),
        Some((h, t)) if t < file_time(path)? => Ok(Some(h)),
        Some(_) => Ok(None),
    }
}

pub fn add_file(
    alias: &str,
    source: impl AsRef<Path>,
    file_map: &impl FileMap<Error = Error>,
    database: &mut impl Database<Error = Error>,
) -> Result<(), Error>
{
    let file_id = get_or_create_file_id(alias, database)?;
    let target = file_map.get(file_id)?;
    mux_file(source, target)
}

fn current_time() -> Result<u64, Error> {
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .map_err(|e| Error::SystemTimeError(e))?
        .as_secs();
    Ok(time)
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

fn get_or_create_file_id(
    alias: &str,
    database: &mut impl Database<Error = Error>,
) -> Result<u32, Error>
{
    if let Some(file_id) = database.get_file_id(alias)?
    {
        return Ok(file_id);
    }
    let file_id = database.create_file_id()?;
    database.create_alias(alias, file_id)?;
    Ok(file_id)
}

fn try_hash_file(
    path: impl AsRef<Path>,
) -> Result<Vec<String>, Error>
{
    Ok(crate::media_mixer::try_hash_file(path)?)
}

fn mux_file(
    source: impl AsRef<Path>,
    target: impl AsRef<Path>,
) -> Result<(), Error>
{
    Ok(crate::media_mixer::mux_file(source, target)?)
}