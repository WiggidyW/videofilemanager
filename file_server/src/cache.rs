// pub use memory_cache::Cache as MemoryCache;

pub trait Cache {
    type Error: std::error::Error + 'static;
    type Hashes: AsRef<[String]>;
    fn get(
        &mut self,
        key: u32,
    ) -> Result<Option<(Self::Hashes, u64)>, Self::Error>;
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

// mod memory_cache {
//     use std::{rc::Rc, ops::Deref, collections::HashMap, convert::Infallible, marker::PhantomData};
//     use super::Cache as CacheTrait;

//     pub struct Cache {
//         inner: HashMap<u32, (Vec<String>, u64)>,
//     }

//     // impl<'c> Deref for Cache<'c> {
//     //     type Target = HashMap<u32, (Vec<String>, u64)>;
//     //     fn deref(&self) -> &Self::Target {
//     //         &self.inner
//     //     }
//     // }

// //     impl<'c> AsRef<HashMap<u32, (Vec<String>, u64)>> for &'c mut Cache<'c> {
// //         fn as_ref(&self) -> &HashMap<u32, (Vec<String>, u64)> {
// //             unimplemented!()
// //         }
// //     }

//     impl Cache {
//         pub fn new() -> Self {
//             Self {
//                 inner: HashMap::new(),
//             }
//         }
//     }

//     impl CacheTrait for Cache {
//         type Error = Infallible;
//         type Hashes = Vec<String>;
//         fn get(
//             &mut self,
//             key: u32,
//         ) -> Result<Option<(Self::Hashes, u64)>, Self::Error>
//         {
//             // match HashMap::get(*self, &key) {
//             // match (**self).get(&key) {
//             // match (*self).inner.get(&key) {
//             match self.inner.get(&key) {
//                 Some(v) => Ok(Some(v.clone())),
//                 None => Ok(None),
//                 _ => unimplemented!()
//             }
//         }
//         fn set<T: AsRef<[String]>>(
//             &mut self,
//             key: u32,
//             hashes: T,
//             timestamp: u64,
//         ) -> Result<(), Self::Error>
//         {
//             // HashMap::
//             self.insert(
//                 key.clone(),
//                 (
//                     Vec::from(hashes.as_ref()),
//                     timestamp,
//                 )
//             );
//             Ok(())
//         }
//         fn remove(
//             &mut self,
//             key: u32,
//         ) -> Result<(), Self::Error>
//         {
//             self.remove(key);
//             Ok(())
//         }
//     }
// }