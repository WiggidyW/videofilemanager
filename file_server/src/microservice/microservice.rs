use rocket::{State, http::RawStr, data::Data, request::FromParam};
use crate::core::{self, FileMap, FileTable, Database, FileId};
use std::{iter, result::Result as StdResult};
use super::{FileContent, Content, Error};
use rocket_contrib::json::Json;

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
    fn as_file_id<'db, 't, 'm>(
        &self,
        states: &States<'db, 't, 'm>,
    ) -> Result<FileId<'db, 't, 'm>>
    {
        Ok(match self {
            Self::Alias(s) => FileId::from_alias(
                s,
                states.database,
                states.file_table,
                states.file_map,
            )?,
            Self::Id(i) => FileId::from_id(
                *i,
                states.database,
                states.file_table,
                states.file_map,
            )?,
        })
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
    Ok(Content::alias_list(FileId::all(
        states.database,
        states.file_table,
        states.file_map,
    )?
        .into_iter()
        .map(|id| Ok((
            *id,
            id.get_aliases()?
        )))
        .collect::<StdResult<Vec<(u32, Vec<String>)>, core::Error>>()?
        .into_iter()
    ))
}

#[get("/files/<id>")]
fn get_file<'t>(
    id: Id,
    states: State<States<'_, 't, '_>>,
) -> Result<FileContent<'t>>
{
    Ok(FileContent::new(
        id.as_file_id(&states)?
            .ro_file()?
        )?
    )
}

#[post("/files/<id>?push", data = "<data>")]
fn push_file(id: Id, states: State<States>, data: Data) -> Result<Content> {
    Ok(id.as_file_id(&states)?
        .rw_file()?
        .with_file(data.open())
        .map(|_| Content::okay())?
    )
}

#[post("/files/<id>?pop", format = "json", data = "<list>")]
fn pop_file(
    id: Id,
    states: State<States>,
    list: Json<Vec<String>>,
) -> Result<Content>
{
    Ok(id.as_file_id(&states)?
        .rw_file()?
        .without_streams(list.into_inner())
        .map(|_| Content::okay())?
    )
}

#[get("/streams/<id>")]
fn get_stream_hashes(id: Id, states: State<States>) -> Result<Content> {
    Ok(id.as_file_id(&states)?
        .ro_file()?
        .stream_hashes()
        .map(|hashes| Content::stream_hashes(
            iter::once((id.into_key(), &*hashes))
        ))?
    )
}

#[get("/probe/<id>")]
fn get_probe(id: Id, states: State<States>) -> Result<Content> {
    Ok(id.as_file_id(&states)?
        .rw_file()?
        .json_probe()
        .map(|json| Content::json_probe(json))?
    )
}

#[get("/aliases/<id>")]
fn get_aliases(id: Id, states: State<States>) -> Result<Content> {
    Ok(id.as_file_id(&states)?
        .get_aliases()
        .map(|aliases| Content::alias_list(
            iter::once((id.into_key(), aliases))
        ))?
    )
}

#[post("/aliases/<id>?push", format = "json", data = "<list>")]
fn push_aliases(
    id: Id,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    Ok(id.as_file_id(&states)?
        .with_aliases(list.into_inner())
        .map(|_| Content::okay())?
    )
}

#[post("/aliases/<id>?pop", format = "json", data = "<list>")]
fn pop_aliases(
    id: Id,
    list: Json<Vec<String>>,
    states: State<States>,
) -> Result<Content>
{
    Ok(id.as_file_id(&states)?
        .without_aliases(list.into_inner())
        .map(|_| Content::okay())?
    )
}