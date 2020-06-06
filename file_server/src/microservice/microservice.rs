use rocket::{State, http::RawStr, response::Responder, data::Data};
use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError};
use crate::core::{self, Error, FileMap, Cache, Database};
use super::Response;

pub fn run(file_map: FileMap, cache: Cache, database: Database) {
    rocket::ignite()
        .manage(file_map)
        .manage(RwLock::new(cache))
        .manage(RwLock::new(database));
}

#[get("/<id>")]
fn get_from_file_id(
    id: u32,
    file_map: State<FileMap>,
) -> Response
{
    let file_map = &*file_map;
    let file = match core::File::from_file_id(id, file_map) {
        Ok(f) => f,
        Err(e) => return Response::internal_error(&e),
    };
    match file.path() {
        Ok(Some(path)) => Response::file(&path),
        Ok(None) => Response::id_file_not_found(id),
        Err(e) => Response::internal_error(&e),
    }
}

#[get("/<id>", rank = 2)]
fn get_from_alias(
    id: String,
    file_map: State<FileMap>,
    database: State<RwLock<Database>>,
) -> Response
{
    let database = &*database.read().unwrap();
    let file_id = match core::FileId::from_alias(&id, database) {
        Ok(Some(i)) => i,
        Ok(None) => return Response::alias_not_found(&id),
        Err(e) => return Response::internal_error(&e),
    };
    match get_from_file_id(*file_id, file_map) {
        Response::NotFoundError(_) => Response::alias_file_not_found(&id),
        any => any,
    }
}

// #[post("/<alias>", data = "<data>")]
// fn post(
//     alias: String,
//     data: Data,
//     file_map: State<FileMap>,
//     database: State<RwLock<Database>>,
// ) -> Response
// {
//     let file_map = &*file_map;
//     let database = &mut *database.write().unwrap();
//     match core::add_file(&alias, data.open(), file_map, database) {
//         Ok(()) => Response::okay(),
//         Err(e) => Response::internal_error(&e),
//     }
// }

// #[get("/<alias>/streams")]
// fn get_streams(
//     alias: String,
//     file_map: State<FileMap>,
//     cache: State<RwLock<Cache>>,
//     database: State<RwLock<Database>>,
// ) -> Response
// {
//     let file_map = &*file_map;
//     let database = &*database.read().unwrap();
//     match core::get_file_path(&alias, file_map, database) {
//         Ok(Some(_)) => (),
//         Ok(None) => return Response::file_not_found_error(&alias),
//         Err(e) => return Response::internal_error(&e),
//     };
//     let cache_ = &*cache.read().unwrap();
//     match core::try_get_hashes(&alias, file_map, cache_, database) {
//         Ok(Some(hashes)) => return Response::streams(hashes),
//         Ok(None) => (),
//         Err(e) => return Response::internal_error(&e),
//     };
//     let cache_ = &mut *cache.write().unwrap();
//     match core::refresh_hashes(&alias, file_map, cache_, database) {
//         Ok(Some(hashes)) => Response::streams(&hashes),
//         Ok(None) => Response::file_not_found_error(&alias),
//         Err(e) => Response::internal_error(&e),
//     }
// }

// #[get("/<alias>/aliases")]
// fn get_aliases(
//     alias: String,
//     database: State<RwLock<Database>>,
// ) -> Response
// {
//     let database = &*database.read().unwrap();
//     match core::get_aliases(&alias, database) {
//         Ok(Some(aliases)) => unimplemented!(),
//         Ok(None) => Response::alias_not_found_error(&alias),
//         Err(e) => Response::internal_error(&e),
//     }
// }

// #[post("/<alias>/aliases?<new_aliases..>")]
// fn post_alias(
//     alias: &RawStr,
//     database: State<RwLock<Database>>,
//     new_aliases: Vec<&RawStr>,
// ) -> Response
// {
//     unimplemented!()
// }