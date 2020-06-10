use std::{sync::RwLock, fmt::{Display, Debug, self}, error::Error as StdError, ops::Deref, hash::Hash, iter, result::Result as StdResult};
use rocket::{State, http::RawStr, response::Responder, data::Data, request::{FromRequest, FromParam, self}};
use rocket_contrib::json::Json;
use serde::{Serialize, Deserialize};
use derive_more::Deref;
use crate::core::{self, FileMap, FileTable, Database, FileId, File, ROFile, RWFile};
use super::{FileContent, Content, Error};

type Result<T> = std::result::Result<T, Error>;

impl<'r, 'db, 't, 'm> FromParam<'r> for FileId<'db, 't, 'm> {
    type Error = &'r RawStr;
    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        if let Ok(i) = u32::from_param(param)
    }
}

#[derive(Deref)]
#[deref(forward)]
struct Id<'db, 't, 'm> {
    inner: FileId<'db, 't, 'm>
}

// impl<'a, 'r, 'db, 't, 'm> FromRequest<'a, 'r> for Id<'db, 't, 'm> {
//     type Error = Error;
//     fn from_request(
//         request: &'a Request<'r>,
//     ) -> request::Outcome<Self, Self::Error>
//     {

//     }
// }

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
fn list<'db, 't, 'm>(states: State<States>) -> Result<Content> {
    states.list_ids_and_aliases()
}

#[get("/files/<id>")]
fn get_file_id<'t>(
    id: u32,
    states: State<States<'_, 't, '_>>,
) -> Result<FileContent<'t>>
{
    let id = Id::from_id(id, &states)?;
    id.stream_file()
}

#[get("/files/<alias>", rank = 2)]
fn get_file_alias<'t>(
    alias: String,
    states: State<States<'_, 't, '_>>,
) -> Result<FileContent<'t>>
{
    let id = Id::from_alias(&alias, &states)?;
    id.stream_file()
}

#[get("/streams/<id>")]
fn get_streams_id(id: u32, states: State<States>) -> Result<Content> {
    let id = Id::from_id(id, &states)?;
    id.display_stream_hashes(*id)
}

#[get("/streams/<alias>", rank = 2)]
fn get_streams_alias(alias: String, states: State<States>) -> Result<Content> {
    let id = Id::from_alias(&alias, &states)?;
    id.display_stream_hashes(alias)
}

#[get("/probe/<id>")]
fn get_probe_id(id: u32, states: State<States>) -> Result<Content> {
    let id = Id::from_id(id, &states)?;
    id.json_probe()
}

#[get("/probe/<alias>", rank = 2)]
fn get_probe_alias(alias: String, states: State<States>) -> Result<Content> {
    let id = Id::from_alias(&alias, &states)?;
    id.json_probe()
}

#[get("/aliases/<id>")]
fn get_aliases_id(id: u32, states: State<States>) -> Result<Content> {
    let id = Id::from_id(id, &states)?;
    id.display_aliases(*id)
}

#[get("/aliases/<alias>", rank = 2)]
fn get_aliases_alias(alias: String, states: State<States>) -> Result<Content> {
    let id = Id::from_alias(&alias, &states)?;
    id.display_aliases(alias)
}

#[post("/aliases/<id>?push", format = "json", data = "<list>")]
fn push_aliases_id(
    id: u32,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    let id = Id::from_id(id, &states)?;
    id.push_aliases(list)
}

#[post("/aliases/<alias>?push", format = "json", data = "<list>", rank = 2)]
fn push_aliases_alias(
    alias: String,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    let id = Id::from_alias(&alias, &states)?;
    id.push_aliases(list)
}

#[post("/aliases/<id>?pop", format = "json", data = "<list>")]
fn pop_aliases_id(
    id: u32,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    let id = Id::from_id(id, &states)?;
    id.pop_aliases(list)
}

#[post("/aliases/<alias>?pop", format = "json", data = "<list>", rank = 2)]
fn pop_aliases_alias(
    alias: String,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    let id = Id::from_alias(&alias, &states)?;
    id.pop_aliases(list)
}

struct States<'db, 't, 'm> {
    database: &'db Database,
    file_table: &'t FileTable,
    file_map: &'m FileMap,
}

impl<'db, 't, 'm> States<'db, 't, 'm> {
    fn list_ids_and_aliases(&self) -> Result<Content> {
        let list = FileId::all(self.database, self.file_table, self.file_map)
            .map_err(|e| Error::internal(e))?
            .into_iter()
            .map(|id| Ok((*id, id.get_aliases()?)))
            .collect::<StdResult<Vec<(u32, Vec<String>)>, core::Error>>()
            .map_err(|e| Error::internal(e))?;
        Ok(Content::alias_list(list.into_iter()))
    }
}

impl<'db, 't, 'm> Id<'db, 't, 'm> {
    fn from_id(
        id: u32,
        states: &States<'db, 't, 'm>,
    ) -> Result<Self>
    {
        match FileId::from_id(
            id,
            states.database,
            states.file_table,
            states.file_map,
        ) {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::id_not_found()),
            Ok(Some(i)) => Ok(Self { inner: i }),
        }
    }
    fn from_alias(alias: &str, states: &States<'db, 't, 'm>) -> Result<Self> {
        match FileId::from_alias(
            alias,
            states.database,
            states.file_table,
            states.file_map,
        ) {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::alias_not_found()),
            Ok(Some(i)) => Ok(Self { inner: i }),
        }
    }
    fn stream_file(&self) -> Result<FileContent<'t>> {
        let file = self.inner.ro_file().map_err(|e| Error::internal(e))?;
        match FileContent::new(file) {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::file_not_found()),
            Ok(Some(f)) => Ok(f),
        }
    }
    fn display_stream_hashes(
        &self,
        display: impl Serialize + Hash + Eq,
    ) -> Result<Content>
    {
        let file = self.inner.ro_file().map_err(|e| Error::internal(e))?;
        // Try to get the hashes from the read-only file
        match file.stream_hashes() {
            Err(e) => return Err(Error::internal(e)),
            Ok(None) => (),
            Ok(Some(h)) => return Ok(Content::stream_hashes(
                iter::once((display, h))
            )),
        };
        // Get the hashes from the read-write file
        let mut file = self.inner.rw_file().map_err(|e| Error::internal(e))?;
        match file.refresh_stream_hashes() {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::file_not_found()),
            Ok(Some(h)) => Ok(Content::stream_hashes(
                iter::once((display, h))
            )),
        }
    }
    fn display_aliases(
        &self,
        display: impl Serialize + Hash + Eq,
    ) -> Result<Content>
    {
        match self.inner.get_aliases() {
            Err(e) => Err(Error::internal(e)),
            Ok(a) => Ok(Content::alias_list(iter::once((display, a)))),
        }
    }
    fn push_aliases(&self, list: Json<Vec<String>>) -> Result<Content> {
        match self.inner.with_aliases(list.into_inner()) {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::invalid_alias_addition()),
            Ok(Some(())) => Ok(Content::okay()),
        }
    }
    fn pop_aliases(&self, list: Json<Vec<String>>) -> Result<Content> {
        match self.inner.without_aliases(list.into_inner()) {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::invalid_alias_removal()),
            Ok(Some(())) => Ok(Content::okay()),
        }
    }
    fn json_probe(&self) -> Result<Content> {
        match self.inner.rw_file()
            .map_err(|e| Error::internal(e))?
            .json_probe()
        {
            Err(e) => Err(Error::internal(e)),
            Ok(None) => Err(Error::file_not_found()),
            Ok(Some(json)) => Ok(Content::json_probe(json)),
        }
    }
}