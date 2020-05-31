use std::path::Path;
use crate::{Cache, FileMap, Database};

pub struct Operator<F, C, D> {
    file_map: F,
    cache: C,
    database: D,
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

impl<F, C, D> Operator<F, C, D>
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
    ) -> Result<Vec<String>, error!()>
    {
        unimplemented!()
    }

    pub fn get_hashes(
        &mut self,
        alias: &str,
    ) -> Result<Option<Vec<String>>, error!()>
    {
        let file_id = match self.database
            .get_file_id(alias)
            .map_err(|e| <error!()>::DatabaseError(e))?
        {
            Some(i) => i,
            None => return Ok(None),
        };
        let file_path = self.file_map
            .get(file_id)
            .map_err(|e| <error!()>::FileMapError(e))?;
        if !file_path.is_file() {
            return Ok(None);
        }
        if let Some((hashes, cache_time)) = self.cache
            .get(file_id)
            .map_err(|e| <error!()>::CacheError(e))?
        {
            let file_time = Self::file_time(&file_path)?;
            if cache_time >= file_time {
                let hashes = Vec::from(hashes.as_ref());
                return Ok(Some(hashes));
            }
            else {
                self.cache
                    .remove(file_id)
                    .map_err(|e| <error!()>::CacheError(e))?;
            }
        }
        let hashes = Self::hash_file(&file_path)?;
        self.cache.set(
            file_id,
            &hashes,
            Self::current_time()?
        ).map_err(|e| <error!()>::CacheError(e))?;
        Ok(Some(hashes))
    }

    pub fn get_aliases(
        &mut self,
        alias: &str,
    ) -> Result<Option<Vec<String>>, error!()>
    {
        let file_id = match self.database
            .get_file_id(alias)
            .map_err(|e| <error!()>::DatabaseError(e))?
        {
            Some(i) => i,
            None => return Ok(None),
        };
        self.database
            .get_aliases(file_id)
            .map_err(|e| <error!()>::DatabaseError(e))
    }

    pub fn add_alias(
        &mut self,
        alias: &str,
        new_alias: &str,
    ) -> Result<Option<()>, error!()>
    {
        let file_id = match self.database
            .get_file_id(alias)
            .map_err(|e| <error!()>::DatabaseError(e))?
        {
            Some(i) => i,
            None => self.database
                .create_file_id()
                .map_err(|e| <error!()>::DatabaseError(e))?,
        };
        self.database
            .create_alias(new_alias, file_id)
            .map_err(|e| <error!()>::DatabaseError(e))
    }

    pub fn remove_alias(
        &mut self,
        alias: &str
    ) -> Result<Option<()>, error!()>
    {
        self.database
            .remove_alias(alias)
            .map_err(|e| <error!()>::DatabaseError(e))
    }
}