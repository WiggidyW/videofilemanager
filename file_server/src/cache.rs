pub use memory_cache::Cache as DefaultCache;

pub trait Cache: Send + Sync + 'static {
    type Error: std::error::Error + 'static;
    fn get(
        &self,
        key: u32,
    ) -> Result<Option<(&[String], u64)>, Self::Error>;
    fn set<T: AsRef<[String]>>(
        &mut self,
        key: u32,
        hashes: T,
        timestamp: u64,
    ) -> Result<(), Self::Error>;
    fn remove(
        &mut self,
        key: u32,
    ) -> Result<(), Self::Error>;
}

mod memory_cache {
    use std::{collections::HashMap, convert::Infallible};
    use super::Cache as CacheTrait;

    pub struct Cache {
        inner: HashMap<u32, (Vec<String>, u64)>,
    }

    impl CacheTrait for Cache {
        type Error = Infallible;
        fn get(
            &self,
            key: u32,
        ) -> Result<Option<(&[String], u64)>, Self::Error>
        {
            match self.inner.get(&key) {
                None => Ok(None),
                Some((s, i)) => Ok(Some((s, *i))),
            }
        }
        fn set<T: AsRef<[String]>>(
            &mut self,
            key: u32,
            hashes: T,
            timestamp: u64,
        ) -> Result<(), Self::Error>
        {
            self.inner.insert(
                key,
                (Vec::from(hashes.as_ref()), timestamp),
            );
            Ok(())
        }
        fn remove(
            &mut self,
            key: u32,
        ) -> Result<(), Self::Error>
        {
            self.inner.remove(&key);
            Ok(())
        }
    }
}