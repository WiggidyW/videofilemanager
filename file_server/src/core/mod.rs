use std::{time::SystemTime, path::{PathBuf}, io::{Read, ErrorKind}, fs::File as StdFile, sync::{RwLock, RwLockReadGuard}};
use {database::Database as DB, file_map::FileMap as FM};
use chashmap::{CHashMap, ReadGuard, WriteGuard};
use derive_more::{Deref, DerefMut};

mod media_mixer;
mod database;
mod file_map;
mod error;

pub use error::Error;
pub type FileMap = file_map::local_file_map::LocalFileMap;
pub type Database = database::sqlite_database::SqliteDatabase;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Copy, Deref)]
pub struct FileId<'db, 't, 'm> {
    #[deref]
    id: u32,
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
}

#[derive(Debug)]
pub struct File {
    path: PathBuf,
    streams: RwLock<Option<(Vec<String>, u64)>>,
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

struct Streams<'a>(RwLockReadGuard<'a, Option<(Vec<String>, u64)>>);

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
    ) -> Result<Self>
    {
        match database.id_exists(id)
            .map_err(|e| Error::database_err(e))?
        {
            true => Ok(Self {
                id: id,
                database: database,
                file_table: file_table,
                file_map: file_map,
            }),
            false => Err(Error::IdNotFound(id)),
        }
    }

    pub fn from_alias(
        alias: impl ToString + AsRef<str>,
        database: &'db Database,
        file_table: &'t FileTable,
        file_map: &'m FileMap,
    ) -> Result<Self>
    {
        match database.get_id(alias.as_ref())
            .map_err(|e| Error::database_err(e))?
        {
            Some(id) => Ok(Self {
                id: id,
                database: database,
                file_table: file_table,
                file_map: file_map,
            }),
            None => Err(Error::AliasNotFound(alias.to_string())),
        }
    }

    pub fn with_aliases(
        &self,
        aliases: Vec<String>,
    ) -> Result<()>
    {
        self.database.create_aliases(aliases, **self)
            .map_err(|e| Error::database_err(e))?
            .ok_or(Error::AliasesAlreadyExist)
    }

    pub fn without_aliases(
        &self,
        aliases: Vec<String>,
    ) -> Result<()>
    {
        for alias in &aliases {
            match self.database.get_id(alias.as_ref()) {
                Ok(Some(i)) if i == **self => (),
                Ok(Some(i)) => return Err(
                    Error::AliasDoesNotMatchId(alias.to_string(), i)
                ),
                Err(e) => return Err(Error::database_err(e)),
                Ok(None) => return Err(Error::AliasNotFound(alias.to_string())),
            };
        }
        self.database.remove_aliases(aliases, **self)
            .map_err(|e| Error::database_err(e))?
            .ok_or(Error::Infallible(Some("Aliases are already validated")))
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
            streams: RwLock::new(None),
        });
    }
}

impl File {
    // return None if the file has no streams
    pub fn with_file(&mut self, file: impl Read) -> Result<()> {
        media_mixer::mux_file(file, &self.path)?
            .ok_or(Error::InvalidMediaFile)
    }

    // return None if any of the hashes are not present in the file
    pub fn without_streams(
        &mut self,
        streams: Vec<String>,
    ) -> Result<()>
    {
        let mut indexes: Vec<usize> = Vec::with_capacity(streams.len());
        let mut streams = streams.into_iter();
        self.stream_hashes()?
            .iter()
            .enumerate()
            .for_each(|(i, s)| 
                if let Some(_) = streams.find(|x| x == s) {
                    indexes.push(i);
                }
            );
        if !(indexes.len() == indexes.capacity()) {
            return Err(Error::StreamHashesNotFound);
        }
        Ok(
            media_mixer::partial_demux_file(&indexes, &self.path)?
        )
    }

    pub fn stream_hashes<'a>(
        &'a self,
    ) -> Result<impl std::ops::Deref<Target = [String]> + 'a>
    {
        let streams = self.streams.read().unwrap();
        if let Some(s) = &*streams {
            if s.1 == self.modified_time()?
            {
                return Ok(Streams(streams));
            }
        }
        std::mem::drop(streams);
        self.refresh_stream_hashes()?;
        Ok(Streams(self.streams.read().unwrap()))
    }

    pub fn len(&self) -> Result<Option<u64>> {
        match self.path.metadata() {
            Ok(m) if m.len() == 0 => Ok(None),
            Ok(m) => Ok(Some(m.len())),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(
                Error::FileNotFound
            ),
            Err(e) => Err(Error::FileSystemError(e)),
        }
    }

    pub fn extension(&self) -> Option<String> {
        self.path.extension()
            .map(|s| s.to_str())
            .map(|s| s.map(|s| s.to_string()))
            .flatten()
    }

    pub fn json_probe(&self) -> Result<serde_json::Value> {
        if !self.path.is_file() {
            return Err(Error::FileNotFound);
        }
        Ok(
            media_mixer::json_probe(&self.path)?
        )
    }

    // return None if file does not exist
    fn refresh_stream_hashes(&self) -> Result<()> {
        *self.streams.write().unwrap() = Some((
            media_mixer::try_hash_file(&self.path)?,
            self.modified_time()?
        ));
        Ok(())
    }

    fn modified_time(&self) -> Result<u64> {
        match self.path.metadata()
            .map(|m| m.modified())
        {
            Ok(Ok(m)) => Ok(m
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| Error::SystemTimeError(e))?
                .as_secs()
            ),
            Err(e) | Ok(Err(e)) if e.kind() == ErrorKind::NotFound => Err(
                Error::FileNotFound
            ),
            Err(e) | Ok(Err(e)) => Err(Error::FileSystemError(e)),
        }
    }
}

impl<'t> ROFile<'t> {
    pub fn into_read(self) -> Result<impl Read + 't> {
        match StdFile::open(&self.path) {
            Ok(f) => Ok(Reader { file: f, rolock: self }),
            Err(e) if e.kind() == ErrorKind::NotFound => Err(
                Error::FileNotFound
            ),
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

impl<'a> std::ops::Deref for Streams<'a> {
    type Target = [String];
    fn deref(&self) -> &Self::Target {
        match &*self.0 {
            Some(s) => &s.0,
            None => unreachable!(), // private type, should never happen
        }
    }
}