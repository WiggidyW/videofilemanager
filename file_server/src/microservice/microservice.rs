use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError, ops::Deref, hash::Hash, iter, result::Result as StdResult};
use rocket::{State, http::RawStr, response::Responder, data::Data, request::{FromRequest, FromParam, self}};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use derive_more::Deref;
use crate::core::{self, FileMap, FileTable, Database, FileId, File, ROFile, RWFile};
use super::{FileContent, Content, Error};

type Result<T> = std::result::Result<T, Error>;

enum Id {
    Alias(String),
    Id(u32),
}
struct States<'db, 't, 'm> {
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
}

impl Id {
    fn into_file_id<'db, 't, 'm>(
        &self,
        states: &States<'db, 't, 'm>,
    ) -> Result<FileId<'db, 't, 'm>>
    {
        match self {
            Self::Alias(s) => FileId::from_alias(
                    s,
                    states.database,
                    states.file_table,
                    states.file_map,
                )
                .map_err(|e| Error::internal(e))?
                .ok_or(Error::alias_not_found()),
            Self::Id(i) => FileId::from_id(
                    *i,
                    states.database,
                    states.file_table,
                    states.file_map,
                )
                .map_err(|e| Error::internal(e))?
                .ok_or(Error::alias_not_found()),
        }
    }
    fn into_key(self) -> String {
        match self {
            Self::Alias(s) => s,
            Self::Id(i) => i.to_string(),
        }
    }
}

impl<'r> FromParam<'r> for Id {
    type Error = &'r RawStr;
    fn from_param(param: &'r RawStr) -> StdResult<Self, Self::Error> {
        if let Ok(i) = u32::from_param(param) {
            return Ok(Id::Id(i));
        }
        String::from_param(param).map(|s| Id::Alias(s))
    }
}

pub fn run<'db: 'static, 't: 'static, 'm: 'static>(
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
) {
    rocket::ignite()
        .manage(States {
            database: database,
            file_table: file_table,
            file_map: file_map,
        });
}

#[get("/list")]
fn list(states: State<States>) -> Result<Content> {
    let list = FileId::all(states.database, states.file_table, states.file_map)
        .map_err(|e| Error::internal(e))?
        .into_iter()
        .map(|id| Ok((
            *id,
            id.get_aliases()?
        )))
        .collect::<StdResult<Vec<(u32, Vec<String>)>, core::Error>>()
        .map_err(|e| Error::internal(e))?;
    Ok(Content::alias_list(list.into_iter()))
}

#[get("/files/<id>")]
fn get_file<'t>(
    id: Id,
    states: State<States<'_, 't, '_>>,
) -> Result<FileContent<'t>>
{
    let file = id.into_file_id(&states)?
        .ro_file()
        .map_err(|e| Error::internal(e))?;
    match FileContent::new(file) {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::file_not_found()),
        Ok(Some(f)) => Ok(f),
    }
}

#[get("/streams/<id>")]
fn get_stream_hashes(id: Id, states: State<States>) -> Result<Content> {
    let fid = id.into_file_id(&states)?;
    match fid.ro_file()
        .map_err(|e| Error::internal(e))?
        .stream_hashes()
    {
        Err(e) => return Err(Error::internal(e)),
        Ok(None) => (),
        Ok(Some(h)) => return Ok(Content::stream_hashes(
            iter::once((id.into_key(), h))
        ))
    };
    match fid.rw_file()
        .map_err(|e| Error::internal(e))?
        .refresh_stream_hashes()
    {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::file_not_found()),
        Ok(Some(h)) => Ok(Content::stream_hashes(
            iter::once((id.into_key(), h))
        ))
    }
}

#[get("/probe/<id>")]
fn get_probe_id(id: Id, states: State<States>) -> Result<Content> {
    match id.into_file_id(&states)?
        .rw_file()
        .map_err(|e| Error::internal(e))?
        .json_probe()
    {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::file_not_found()),
        Ok(Some(json)) => Ok(Content::json_probe(json)),
    }
}

#[get("/aliases/<id>")]
fn get_aliases(id: Id, states: State<States>) -> Result<Content> {
    match id.into_file_id(&states)?
        .get_aliases()
    {
        Err(e) => Err(Error::internal(e)),
        Ok(a) => Ok(Content::alias_list(iter::once((id.into_key(), a)))),
    }
}

#[post("/aliases/<id>?push", format = "json", data = "<list>")]
fn push_aliases(
    id: Id,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    match id.into_file_id(&states)?
        .with_aliases(list.into_inner())
    {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::invalid_alias_addition()),
        Ok(Some(())) => Ok(Content::okay()),
    }
}

#[post("/aliases/<id>?pop", format = "json", data = "<list>")]
fn pop_aliases(
    id: Id,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    match id.into_file_id(&states)?
        .without_aliases(list.into_inner())
    {
        Err(e) => Err(Error::internal(e)),
        Ok(None) => Err(Error::invalid_alias_removal()),
        Ok(Some(())) => Ok(Content::okay()),
    }
}