use rocket::{State, http::RawStr, response::Responder, data::Data};
use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError};
use crate::{core, microservice_responders::Response};

type FileMap = Box<dyn crate::FileMap<Error = core::Error>>;
type Cache = Box<dyn crate::Cache<Error = core::Error>>;
type Database = Box<dyn crate::Database<Error = core::Error>>;

#[get("/<alias>")]
fn get(
    alias: &RawStr,
    file_map: State<FileMap>,
    database: State<RwLock<Database>>,
) -> Response {
    let file_map = &*file_map;
    let database = &*database.read().unwrap();
    match core::get_file_path(alias, file_map, database) {
        Ok(Some(path)) => Response::file(&path),
        Ok(None) => Response::file_not_found_error(alias),
        Err(e) => Response::internal_error(&e),
    }
}

#[post("/<alias>", data = "<data>")]
fn post(
    alias: &RawStr,
    data: Data,
    file_map: State<FileMap>,
    database: State<RwLock<Database>>,
) -> Response {
    let file_map = &*file_map;
    let database = &mut *database.write().unwrap();
    match core::add_file(alias, data.open(), file_map, database) {
        Ok(()) => Response::okay(),
        Err(e) => Response::internal_error(&e),
    }
}

#[get("/<alias>/streams")]
fn get_streams(
    alias: &RawStr,
    file_map: State<FileMap>,
    cache: State<RwLock<Cache>>,
    database: State<RwLock<Database>>,
) -> Response {
    let file_map = &*file_map;
    let database = &*database.read().unwrap();
    match core::get_file_path(alias, file_map, database) {
        Ok(Some(_)) => (),
        Ok(None) => return Response::file_not_found_error(alias),
        Err(e) => return Response::internal_error(&e),
    };
    let cache_ = &*cache.read().unwrap();
    match core::try_get_hashes(alias, file_map, cache_, database) {
        Ok(Some(hashes)) => return Response::streams(hashes),
        Ok(None) => (),
        Err(e) => return Response::internal_error(&e),
    };
    let cache_ = &mut *cache.write().unwrap();
    match core::refresh_hashes(alias, file_map, cache_, database) {
        Ok(Some(hashes)) => Response::streams(&hashes),
        Ok(None) => Response::file_not_found_error(alias),
        Err(e) => Response::internal_error(&e),
    }
}

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
    let database: RwLock<Database> = RwLock::new(Box::new(database));
    rocket::ignite()
        .manage(file_map)
        .manage(cache)
        .manage(database);
}