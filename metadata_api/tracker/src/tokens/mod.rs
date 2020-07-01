use derive_more::{Deref, AsRef};

mod imdbid;

pub use imdbid::ImdbId;
pub use imdbid::ValidationError as ImdbIdError;

#[derive(Debug, Deref, AsRef)]
pub struct Filename(pub String);

pub struct Refresh;