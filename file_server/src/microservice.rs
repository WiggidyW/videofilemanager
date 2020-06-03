use rocket::{State, http::RawStr, response::Responder};
use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError};
use crate::core;

type FileMap = Box<dyn crate::FileMap<Error = core::Error>>;
type Cache = Box<dyn crate::Cache<Error = core::Error>>;
type Database = Box<dyn crate::Database<Error = core::Error>>;

// #[get("/<alias>")]
// fn get(
//     alias: &RawStr,
//     file_map: State<FileMap>,
//     database: State<Database>,
// ) -> Error {
//     match core::get_file_path(alias, &*file_map, &*database) {
//         _ => Error::NotFound("this is a test".to_string())
//     }
// }

pub fn run<F, C, D>(
    file_map: F,
    cache: C,
    database: D,
) where
    F: crate::FileMap<Error = core::Error>,
    C: crate::Cache<Error = core::Error>,
    D: crate::Database<Error = core::Error>,
{
    let file_map: FileMap = Box::new(file_map);
    let cache: RwLock<Cache> = RwLock::new(Box::new(cache));
    let database: Database = Box::new(database);
    rocket::ignite()
        .manage(file_map)
        .manage(cache)
        .manage(database);
}