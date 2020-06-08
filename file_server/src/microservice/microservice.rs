use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError, ops::Deref, hash::Hash, iter};
use rocket::{State, http::RawStr, response::Responder, data::Data};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use crate::core::{self, FileMap, FileTable, Database, FileId, File, ROFile, RWFile};
use super::{FileContent, Content, Error};

pub fn run<'db: 'static, 't: 'static, 'm: 'static>(
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
) {
    rocket::ignite()
        .manage(database)
        .manage(file_table)
        .manage(file_map);
}

#[get("/list")]
fn list<'db, 't, 'm>(
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    let list = FileId::all(&database, &file_table, &file_map)
        .map_err(|e| Error::internal(e))?
        .into_iter()
        .map(|id| Ok((*id, id.get_aliases()?)))
        .collect::<Result<Vec<(u32, Vec<String>)>, core::Error>>()
        .map_err(|e| Error::internal(e))?;
    Ok(Content::alias_list(list.into_iter()))
}

fn get_id_from_alias<'db, 't, 'm>(
    alias: &str,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<FileId<'db, 't, 'm>, Error>
{
    match FileId::from_alias(&alias, &database, &file_table, &file_map) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::alias_not_found()),
        Ok(Some(i)) => Ok(i),
    }
}

fn get_id_from_id<'db, 't, 'm>(
    id: u32,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<FileId<'db, 't, 'm>, Error>
{
    match FileId::from_id(id, &database, &file_table, &file_map) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::id_not_found()),
        Ok(Some(i)) => Ok(i),
    }
}

#[get("/files/<id>")]
fn get_file_from_id<'db, 't, 'm>(
    id: u32,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<FileContent<'t>, Error>
{
    match get_id_from_id(id, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_file(i),
    }
}

#[get("/files/<alias>", rank = 2)]
fn get_file_from_alias<'db, 't, 'm>(
    alias: String,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<FileContent<'t>, Error>
{
    match get_id_from_alias(&alias, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_file(i),
    }
}

fn get_file<'db, 't, 'm>(
    id: FileId<'db, 't, 'm>,
) -> Result<FileContent<'t>, Error>
{
    let file = id.ro_file().map_err(|e| Error::internal(e))?;
    match FileContent::new(file) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::file_not_found()),
        Ok(Some(f)) => Ok(f),
    }
}

#[get("/streams/<id>")]
fn get_streams_from_id<'db, 't, 'm>(
    id: u32,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_id(id, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_streams(i, id),
    }
}

#[get("/streams/<alias>", rank = 2)]
fn get_streams_from_alias<'db, 't, 'm>(
    alias: String,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_alias(&alias, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_streams(i, alias),
    }
}

fn get_streams<'db, 't, 'm>(
    id: FileId<'db, 't, 'm>,
    id_display: impl Serialize + Hash + Eq,
) -> Result<Content, Error> 
{
    let file = id.ro_file().map_err(|e| Error::internal(e))?;
    match file.stream_hashes() {
        Err(e) => return Err(Error::internal(e)),
        Ok(None) => (),
        Ok(Some(h)) => return Ok(Content::stream_hashes(
            iter::once((id_display, h))
        )),
    };
    let mut file = id.rw_file().map_err(|e| Error::internal(e))?;
    match file.refresh_stream_hashes() {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::file_not_found()),
        Ok(Some(h)) => Ok(Content::stream_hashes(
            iter::once((id_display, h))
        )),
    }
}

#[get("/aliases/<id>")]
fn get_aliases_from_id<'db, 't, 'm>(
    id: u32,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_id(id, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_aliases(i, id),
    }
}

#[get("/aliases/<alias>", rank = 2)]
fn get_aliases_from_alias<'db, 't, 'm>(
    alias: String,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_alias(&alias, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => get_aliases(i, alias),
    }
}

fn get_aliases<'db, 't, 'm>(
    id: FileId<'db, 't, 'm>,
    id_display: impl Serialize + Hash + Eq,
) -> Result<Content, Error>
{
    match id.get_aliases() {
        Err(e) => Err(Error::internal(e)),
        Ok(a) => Ok(Content::alias_list(iter::once((id_display, a)))),
    }
}

#[post("/aliases/<id>?push", format = "json", data = "<list>")]
fn post_push_aliases_from_id<'db, 't, 'm>(
    id: u32,
    list: Json<Vec<String>>,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_id(id, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => post_push_aliases(i, list),
    }
}

#[post("/aliases/<alias>?push", format = "json", data = "<list>", rank = 2)]
fn post_push_aliases_from_alias<'db, 't, 'm>(
    alias: String,
    list: Json<Vec<String>>,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_alias(&alias, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => post_push_aliases(i, list),
    }
}

fn post_push_aliases<'db, 't, 'm>(
    id: FileId<'db, 't, 'm>,
    list: Json<Vec<String>>,
) -> Result<Content, Error>
{
    match id.with_aliases(list.into_inner()) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::invalid_alias_addition()),
        Ok(Some(())) => Ok(Content::okay()),
    }
}

#[post("/aliases/<id>?pop", format = "json", data = "<list>")]
fn post_pop_aliases_from_id<'db, 't, 'm>(
    id: u32,
    list: Json<Vec<String>>,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_id(id, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => post_pop_aliases(i, list),
    }
}

#[post("/aliases/<alias>?pop", format = "json", data = "<list>", rank = 2)]
fn post_pop_aliases_from_alias<'db, 't, 'm>(
    alias: String,
    list: Json<Vec<String>>,
    database: State<&'db Database>,
    file_table: State<&'t FileTable>,
    file_map: State<&'m FileMap>,
) -> Result<Content, Error>
{
    match get_id_from_alias(&alias, database, file_table, file_map) {
        Err(e) => Err(e),
        Ok(i) => post_pop_aliases(i, list),
    }
}

fn post_pop_aliases<'db, 't, 'm>(
    id: FileId<'db, 't, 'm>,
    list: Json<Vec<String>>,
) -> Result<Content, Error>
{
    match id.without_aliases(list.into_inner()) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::invalid_alias_removal()),
        Ok(Some(())) => Ok(Content::okay()),
    }
}