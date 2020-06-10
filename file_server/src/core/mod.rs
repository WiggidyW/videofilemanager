mod media_mixer;
mod database;
mod file_map;
mod error;

pub use error::Error;
pub type FileMap = file_map::local_file_map::LocalFileMap;
pub type Database = database::sqlite_database::SqliteDatabase;

type Result<T> = std::result::Result<T, Error>;

use {database::Database as DB, file_map::FileMap as FM};
use std::{time::SystemTime, path::{Path, PathBuf}, io::{Read, ErrorKind}, fs::File as StdFile};
use derive_more::{Deref, DerefMut};
use chashmap::{CHashMap, ReadGuard, WriteGuard};

#[derive(Debug, Clone, Copy, Deref)]
pub struct FileId<'db, 't, 'm> {
    #[deref]
    id: u32,
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
}

#[derive(Debug, PartialEq, Hash)]
pub struct File {
    path: PathBuf,
    streams: Option<(Vec<String>, u64)>,
}

#[derive(Debug, Deref)]
pub struct FileTable {
    inner: CHashMap<u32, File>,
}

#[derive(Debug, Deref)]
#[deref(forward)]
pub struct ROFile<'t> {
    inner: ReadGuard<'t, u32, File>,
}

#[derive(Debug, Deref, DerefMut)]
#[deref(forward)]
#[deref_mut(forward)]
pub struct RWFile<'t> {
    inner: WriteGuard<'t, u32, File>,
}

#[derive(Deref, DerefMut)]
struct Reader<'t> {
    #[deref]
    #[deref_mut]
    file: StdFile,
    rolock: ROFile<'t>
}

impl<'db, 't, 'm> FileId<'db, 't, 'm> {
    pub fn new(
        database: &'db Database,
        file_table: &'t FileTable,
        file_map: &'m FileMap,
    ) -> Result<Self>
    {
        Ok(Self {
            id: database.create_id()
                .map_err(|e| Error::database_err(e))?,
            database: database,
            file_table: file_table,
            file_map: file_map,
        })
    }

    pub fn all(
        database: &'db Database,
        file_table: &'t FileTable,
        file_map: &'m FileMap,
    ) -> Result<Vec<Self>>
    {
        Ok(database.list_ids()
            .map_err(|e| Error::database_err(e))?
            .into_iter()
            .map(|id| Self {
                id: id,
                database: database,
                file_table: file_table,
                file_map: file_map,
            })
            .collect()
        )
    }

    pub fn from_id(
        id: u32,
        database: &'db Database,
        file_table: &'t FileTable,
        file_map: &'m FileMap,
    ) -> Result<Option<Self>>
    {
        match database.id_exists(id)
            .map_err(|e| Error::database_err(e))?
        {
            true => Ok(Some(Self {
                id: id,
                database: database,
                file_table: file_table,
                file_map: file_map,
            })),
            false => Ok(None),
        }
    }

    pub fn from_alias(
        alias: &str,
        database: &'db Database,
        file_table: &'t FileTable,
        file_map: &'m FileMap,
    ) -> Result<Option<Self>>
    {
        match database.get_id(alias)
            .map_err(|e| Error::database_err(e))?
        {
            Some(id) => Ok(Some(Self {
                id: id,
                database: database,
                file_table: file_table,
                file_map: file_map,
            })),
            None => Ok(None),
        }
    }

    pub fn with_aliases(
        &self,
        aliases: Vec<String>,
    ) -> Result<Option<()>>
    {
        self.database.create_aliases(aliases, **self)
            .map_err(|e| Error::database_err(e))
    }

    pub fn without_aliases(
        &self,
        aliases: Vec<String>,
    ) -> Result<Option<()>>
    {
        for alias in &aliases {
            match self.database.get_id(alias.as_ref()) {
                Ok(Some(i)) if i == **self => (),
                Err(e) => return Err(Error::database_err(e)),
                _ => return Ok(None),
            };
        }
        self.database.remove_aliases(aliases, **self)
            .map_err(|e| Error::database_err(e))
    }

    pub fn get_aliases(&self) -> Result<Vec<String>> {
        self.database.get_aliases(**self)
            .map(|a| a.unwrap_or(Vec::new()))
            .map_err(|e| Error::database_err(e))
    }

    // Ensures that FileMap and FileTable return the same path, then returns
    // a file for reading only.
    pub fn ro_file(&self) -> Result<ROFile<'t>> {
        let path = self.path()?;
        let inner = match self.file_table.get(self) {
            Some(f) if f.path == path => f,
            _ => {
                self.insert_file(path);
                self.file_table.get(self).unwrap()
            },
        };
        Ok(ROFile { inner: inner })
    }

    // Ensures that FileMap and FileTable return the same path, then returns
    // a file for reading or writing.
    pub fn rw_file(&self) -> Result<RWFile<'t>> {
        let path = self.path()?;
        let inner = match self.file_table.get_mut(self) {
            Some(f) if f.path == path => f,
            _ => {
                self.insert_file(path);
                self.file_table.get_mut(self).unwrap()
            },
        };
        Ok(RWFile { inner: inner })
    }

    fn path(&self) -> Result<PathBuf> {
        self.file_map.get(self)
            .map_err(|e| Error::file_map_err(e))
    }

    fn insert_file(&self, path: PathBuf) {
        self.file_table.insert(**self, File {
            path: path,
            streams: None,
        });
    }
}

impl File {
    // return None if the file has no streams
    pub fn with_file(&mut self, file: impl Read) -> Result<Option<()>> {
        unimplemented!()
    }

    // return None if any of the hashes are not present in the file
    pub fn without_streams(
        &mut self,
        streams: Vec<String>,
    ) -> Result<Option<()>>
    {
        unimplemented!()
    }

    // return None if the hashes are stale or empty
    pub fn stream_hashes<'a>(&'a self) -> Result<Option<&'a [String]>> {
        match &self.streams {
            None => Ok(None),
            Some(streams) =>
        match self.modified_time()?
        {
            None => Ok(None),
            Some(t) if t == streams.1 => Ok(Some(&streams.0)),
            Some(_) => Ok(None),
        }}
    }

    // return None if file does not exist
    pub fn refresh_stream_hashes<'a>(
        &'a mut self,
    ) -> Result<Option<&'a [String]>> {
        unimplemented!()
    }

    pub fn len(&self) -> Result<Option<u64>> {
        match self.path.metadata() {
            Ok(m) if m.len() == 0 => Ok(None),
            Ok(m) => Ok(Some(m.len())),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(Error::FileSystemError(e)),
        }
    }

    pub fn extension(&self) -> Option<String> {
        self.path.extension()
            .map(|s| s.to_str())
            .map(|s| s.map(|s| s.to_string()))
            .flatten()
    }

    pub fn json_probe(&mut self) -> Result<Option<serde_json::Value>> {
        if !self.path.is_file() {
            return Ok(None);
        }
        Ok(Some(
            media_mixer::json_probe(&self.path)?
        ))
    }

    fn modified_time(&self) -> Result<Option<u64>> {
        match self.path.metadata()
            .map(|m| m.modified())
        {
            Ok(Ok(m)) => Ok(Some(m
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| Error::SystemTimeError(e))?
                .as_secs()
            )),
            Err(e) | Ok(Err(e)) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) | Ok(Err(e)) => Err(Error::FileSystemError(e)),
        }
    }
}

impl<'t> ROFile<'t> {
    pub fn into_read(self) -> Result<Option<impl Read + 't>> {
        match StdFile::open(&self.path) {
            Ok(f) => Ok(Some(Reader { file: f, rolock: self })),
            Err(e) if e.kind() == ErrorKind::NotFound => Ok(None),
            Err(e) => Err(Error::FileSystemError(e)),
        }
    }
}

impl Read for Reader<'_> {
    fn read(
        &mut self,
        buf: &mut [u8],
    ) -> std::result::Result<usize, std::io::Error>
    {
        (**self).read(buf)
    }
}