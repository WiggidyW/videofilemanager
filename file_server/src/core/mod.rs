mod media_mixer;
mod cache;
mod database;
mod file_map;
mod error;

pub use error::Error;
pub type FileMap = file_map::local_file_map::LocalFileMap;
pub type Cache = cache::memory_cache::MemoryCache;
pub type Database = database::sqlite_database::SqliteDatabase;

use std::{time::SystemTime, path::{Path, PathBuf}, io::Read};
use derive_more::Deref;

#[derive(Debug, Clone, Copy, Deref)]
pub struct FileId {
    file_id: u32,
}

#[derive(Debug)]
pub struct File {
    file_id: u32,
    path: PathBuf,
}

impl FileId {
    pub fn from_file_id(
        file_id: u32,
        database: &Database,
    ) -> Result<Option<Self>, Error>
    {
        unimplemented!()
    }

    pub fn from_alias(
        alias: &str,
        database: &Database,
    ) -> Result<Option<Self>, Error>
    {
        unimplemented!()
    }

    pub fn new(
        database: &mut Database,
    ) -> Result<Option<Self>, Error>
    {
        unimplemented!()
    }

    pub fn with_aliases(
        &self,
        aliases: &[impl AsRef<str>],
        database: &mut Database,
    ) -> Result<Option<()>, Error>
    {
        unimplemented!()
    }

    pub fn without_aliases(
        &self,
        aliases: &[impl AsRef<str>],
        database: &mut Database,
    ) -> Result<Option<()>, Error>
    {
        unimplemented!()
    }

    pub fn get_aliases(
        &self,
        database: &Database,
    ) -> Result<Vec<String>, Error>
    {
        unimplemented!()
    }
}

impl File {
    pub fn from_file_id(
        file_id: u32,
        file_map: &FileMap,
    ) -> Result<Self, Error>
    {
        unimplemented!()
    }

    pub fn with_file(&self, file: impl Read) -> Result<(), Error> {
        unimplemented!()
    }

    pub fn without_streams(
        &self,
        streams: Vec<String>,
    ) -> Result<Option<()>, Error>
    {
        unimplemented!()
    }

    pub fn cached_stream_hashes<'c>(
        &self,
        cache: &'c Cache,
    ) -> Result<Option<&'c [String]>, Error>
    {
        unimplemented!()
    }

    pub fn new_stream_hashes<'c>(
        &self,
        cache: &'c mut Cache,
    ) -> Result<&'c [String], Error>
    {
        unimplemented!()
    }

    pub fn path(&self) -> Result<Option<PathBuf>, Error> {
        unimplemented!()
    }
}

mod stream_hashes {
    use std::path::Path;
    use super::{Error, Cache};

    fn from_path(path: impl AsRef<Path>) -> Result<(Vec<String>, u64), Error> {
        unimplemented!()
    }

    fn from_cache<'c>(
        file_id: u32,
        cache: &'c Cache,
    ) -> Result<Option<(&'c [String], u64)>, Error>
    {
        unimplemented!()
    }
}

mod time {
    use std::{path::Path, time::SystemTime};
    use super::Error;

    fn now() -> Result<u64, Error> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| Error::SystemTimeError(e))?
            .as_secs();
        Ok(timestamp)
    }

    fn modified(path: impl AsRef<Path>) -> Result<u64, Error> {
        let timestamp = path.as_ref()
            .metadata()
            .map_err(|e| Error::FileSystemError(e))?
            .modified()
            .map_err(|e| Error::FileSystemError(e))?
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| Error::SystemTimeError(e))?
            .as_secs();
        Ok(timestamp)
    }
}