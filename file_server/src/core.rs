use std::path::Path;
use crate::{Cache, FileMap, Database, StreamHash};
use wrappers::{FileMapWrapper, CacheWrapper, DatabaseWrapper};

pub struct Operator<F, C, D> {
    file_map: FileMapWrapper<F, C, D>,
    cache: CacheWrapper<F, C, D>,
    database: DatabaseWrapper<F, C, D>,
}

macro_rules! error {
    () => {
        crate::Error<
            <F as FileMap>::Error,
            <C as Cache>::Error,
            <D as Database>::Error,
        >
    }
}

impl<'c, F, C, D> Operator<F, C, D>
where
    F: FileMap,
    C: Cache,
    D: Database,
{
    fn current_time() -> Result<u64, error!()> {
        unimplemented!()
    }

    fn file_time<P: AsRef<Path>>(
        path: P,
    ) -> Result<u64, error!()>
    {
        unimplemented!()
    }

    fn hash_file<P: AsRef<Path>>(
        path: P,
    ) -> Result<Vec<StreamHash>, error!()>
    {
        unimplemented!()
    }

    fn mux_file<P1: AsRef<Path>, P2: AsRef<Path>>(
        source: P1,
        target: P2,
    ) -> Result<(), error!()>
    {
        unimplemented!()
    }

    // The file should be muxed and combined with the file at target,
    // or created if it does not exist.
    pub fn add_file<P: AsRef<Path>>(
        &mut self,
        alias: &str,
        file: P,
    ) -> Result<(), error!()>
    {
        let file_id = match self.database.get_file_id(alias)?
        {
            Some(i) => i,
            None => {
                let file_id = self.database.create_file_id()?;
                self.database.create_alias(alias, file_id)?;
                file_id
            },
        };
        let target = self.file_map.get(file_id)?;
        Self::mux_file(file, target)
    }

    // Returns None if:
    // - Alias does not exist
    // - File from Alias's file_id does not exist
    pub fn get_hashes(
        &mut self,
        alias: &str,
    ) -> Result<Option<Vec<StreamHash>>, error!()>
    {
        let file_id = match self.database.get_file_id(alias)?
        {
            Some(i) => i,
            None => return Ok(None),
        };
        let file_path = self.file_map.get(file_id)?;
        if !file_path.is_file() {
            return Ok(None);
        }
        if let Some((hashes, cache_time)) = self.cache.get(file_id)?
        {
            let file_time = Self::file_time(&file_path)?;
            if cache_time >= file_time {
                return Ok(Some(hashes));
            }
            else {
                self.cache.remove(file_id)?;
            }
        }
        let hashes = Self::hash_file(&file_path)?;
        self.cache.set(
            file_id,
            &hashes,
            Self::current_time()?
        )?;
        Ok(Some(hashes))
    }

    pub fn get_aliases(
        &mut self,
        alias: &str,
    ) -> Result<Option<Vec<String>>, error!()>
    {
        let file_id = match self.database.get_file_id(alias)?
        {
            Some(i) => i,
            None => return Ok(None),
        };
        self.database.get_aliases(file_id)
    }

    // Returns None if:
    // - Alias does not exist
    pub fn remove_alias(
        &mut self,
        alias: &str
    ) -> Result<Option<()>, error!()>
    {
        self.database.remove_alias(alias)
    }

    // Returns None if:
    // - Alias already exists
    pub fn create_alias(
        &mut self,
        alias: &str
    ) -> Result<Option<()>, error!()>
    {
        let file_id = self.database.create_file_id()?;
        self.database.create_alias(alias, file_id)
    }

    // Adds a new alias that points to another alias.
    //
    // Because the "file_id" is a backend construct that is abstracted away,
    // if the alias points to no "file_id", then create a new one.
    //
    // Returns None if:
    // - New Alias already exists
    pub fn add_alias(
        &mut self,
        alias: &str,
        new_alias: &str,
    ) -> Result<Option<()>, error!()>
    {
        let file_id = match self.database.get_file_id(alias)?
        {
            Some(i) => i,
            None => {
                let i = self.database.create_file_id()?;
                self.database.create_alias(alias, i)?;
                i
            },
        };
        self.database.create_alias(new_alias, file_id)
    }
}

mod wrappers {
    use crate::{Cache, FileMap, Database, StreamHash, Error};
    use std::marker::PhantomData;

    pub struct FileMapWrapper<F, C, D>{
        inner: F,
        c: PhantomData<C>,
        d: PhantomData<D>,
    }
    pub struct CacheWrapper<F, C, D>{
        f: PhantomData<F>,
        inner: C,
        d: PhantomData<D>,
    }
    pub struct DatabaseWrapper<F, C, D>{
        f: PhantomData<F>,
        c: PhantomData<C>,
        inner: D,
    }

    impl<F, C, D> FileMap for FileMapWrapper<F, C, D>
    where
        F: FileMap,
        C: Cache,
        D: Database,
    {
        type Error = Error<
            <F as FileMap>::Error,
            <C as Cache>::Error,
            <D as Database>::Error,
        >;
        fn get(&self, key: u32) -> Result<std::path::PathBuf, Self::Error> {
            self.inner
                .get(key)
                .map_err(|e| Self::Error::FileMapError(e))
        }
    }

    impl<F, C, D> Cache for CacheWrapper<F, C, D>
    where
        F: FileMap,
        C: Cache,
        D: Database,
    {
        type Error = Error<
            <F as FileMap>::Error,
            <C as Cache>::Error,
            <D as Database>::Error,
        >;
        fn get(
            &mut self,
            key: u32,
        ) -> Result<Option<(Vec<StreamHash>, u64)>, Self::Error>
        {
            self.inner
                .get(key)
                .map_err(|e| Self::Error::CacheError(e))
        }
        fn set<T: AsRef<[StreamHash]>>(
            &mut self,
            key: u32,
            hashes: T,
            timestamp: u64,
        ) -> Result<(), Self::Error>
        {
            self.inner
                .set(key, hashes, timestamp)
                .map_err(|e| Self::Error::CacheError(e))
        }
        fn remove(
            &mut self,
            key: u32,
        ) -> Result<(), Self::Error>
        {
            
            self.inner
                .remove(key)
                .map_err(|e| Self::Error::CacheError(e))
        }
    }

    impl<F, C, D> Database for DatabaseWrapper<F, C, D>
    where
        F: FileMap,
        C: Cache,
        D: Database,
    {
        type Error = Error<
            <F as FileMap>::Error,
            <C as Cache>::Error,
            <D as Database>::Error,
        >;
        fn create_file_id(
            &mut self,
        ) -> Result<u32, Self::Error>
        {
            self.inner
                .create_file_id()
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        fn create_alias(
            &mut self,
            alias: &str,
            file_id: u32,
        ) -> Result<Option<()>, Self::Error>
        {
            self.inner
                .create_alias(alias, file_id)
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        fn get_file_id(
            &mut self,
            alias: &str,
        ) -> Result<Option<u32>, Self::Error>
        {
            self.inner
                .get_file_id(alias)
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        fn remove_file_id(
            &mut self,
            file_id: u32,
        ) -> Result<Option<()>, Self::Error>
        {
            self.inner
                .remove_file_id(file_id)
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        fn get_aliases(
            &mut self,
            file_id: u32,
        ) -> Result<Option<Vec<String>>, Self::Error>
        {
            self.inner
                .get_aliases(file_id)
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        fn remove_alias(
            &mut self,
            alias: &str,
        ) -> Result<Option<()>, Self::Error>
        {
            self.inner
                .remove_alias(alias)
                .map_err(|e| Self::Error::DatabaseError(e))
        }
        // fn commit_transaction(&mut self) -> Result<(), Self::Error> {
        //     self.inner
        //         .commit_transaction()
        //         .map_err(|e| Self::Error::DatabaseError(e))
        // }
        // fn start_transaction(&mut self) {
        //     self.inner.start_transaction()
        // }
        // fn discard_transaction(&mut self) {
        //     self.inner.discard_transaction()
        // }
    }
}