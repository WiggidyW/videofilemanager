use crate::{FileMap, Cache, Database};

pub enum Error<F, C, D> {
    FileMapError(F),
    CacheError(C),
    DatabaseError(D),
}