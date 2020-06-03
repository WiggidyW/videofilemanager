pub use memory_cache::Cache as DefaultCache;

pub trait Cache: Send + Sync + 'static {
    type Error: std::error::Error + 'static;

    fn get(&self, key: u32) -> Result<Option<(&[String], u64)>, Self::Error>;

    fn set(
        &mut self,
        key: u32,
        hashes: &[String],
        timestamp: u64,
    ) -> Result<(), Self::Error>;

    fn remove(&mut self, key: u32) -> Result<(), Self::Error>;
}

impl<T: Cache + ?Sized> Cache for Box<T> {
    type Error = <T as Cache>::Error;
    fn get(&self, key: u32) -> Result<Option<(&[String], u64)>, Self::Error> {
        (**self).get(key)
    }
    fn set(
        &mut self,
        key: u32,
        hashes: &[String],
        timestamp: u64,
    ) -> Result<(), Self::Error>
    {
        (**self).set(key, hashes, timestamp)
    }
    fn remove(&mut self, key: u32) -> Result<(), Self::Error> {
        (**self).remove(key)
    }
}

mod impl_cache {
    use super::Cache;
    use crate::core::Error;

    struct ImplCache<T>(T);

    impl<T: Cache> ImplCache<T> {
        pub fn new(t: T) -> Self {
            Self(t)
        }
    }

    impl<T: Cache + Sized> Cache for ImplCache<T> {
        type Error = Error;
        fn get(
            &self,
            key: u32,
        ) -> Result<Option<(&[String], u64)>, Self::Error>
        {
            unimplemented!()
        }
        fn set(
            &mut self,
            key: u32,
            hashes: &[String],
            timestamp: u64,
        ) -> Result<(), Self::Error>
        {
            unimplemented!()
        }
        fn remove(
            &mut self,
            key: u32,
        ) -> Result<(), Self::Error>
        {
            unimplemented!()
        }
    }
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
        fn set(
            &mut self,
            key: u32,
            hashes: &[String],
            timestamp: u64,
        ) -> Result<(), Self::Error>
        {
            self.inner.insert(
                key,
                (Vec::from(hashes), timestamp),
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