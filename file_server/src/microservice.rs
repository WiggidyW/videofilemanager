use std::sync::RwLock;
use crate::{FileMap, Cache, Database};

pub fn run<F, C, D>(file_map: F, cache: RwLock<C>, database: D)
where
    F: FileMap,
    C: Cache,
    D: Database,
{
    rocket::ignite()
        .manage(file_map)
        .manage(cache)
        .manage(database);
}