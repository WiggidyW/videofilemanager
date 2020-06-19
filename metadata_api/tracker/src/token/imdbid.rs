use lazy_static::lazy_static;
use std::sync::RwLock;
use std::collections::HashMap;

#[derive(Clone, Copy)]
pub struct ImdbId(u32);

lazy_static! {
    static ref IMDBIDS: RwLock<HashMap<u32, bool>> = RwLock::new(HashMap::new());
}

impl ImdbId {
    pub fn new_forced(id: u32) -> Self {
        IMDBIDS.write().unwrap().insert(id, true);
        Self(id)
    }
    fn new_invalid(id: u32) -> Option<Self> {
        IMDBIDS.write().unwrap().insert(id, false);
        None
    }
    pub fn new_unchecked(id: u32) -> Self {
        Self(id)
    }
    pub async fn new(id: u32) -> Result<Option<Self>, reqwest::Error> {
        match IMDBIDS.read().unwrap().get(&id) {
            Some(true) => return Ok(Some(Self(id))),
            Some(false) => return Ok(None),
            None if id >= 100_000_000 => return Ok(None),
            None => (),
        };
        match reqwest::Client::builder()
            .build()?
            .head(&format!("https://www.imdb.com/title/tt{:0>7}/", id))
            .send()
            .await?
        {
            r if r.status().is_success() => Ok(Some(Self::new_forced(id))),
            r if r.status().as_u16() == 404 => Ok(Self::new_invalid(id)),
            r => Err(r.error_for_status().unwrap_err()),
        }
    }
    pub fn to_raw_string(&self) -> String {
        self.0.to_string()
    }
}

impl AsRef<u32> for ImdbId {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl ToString for ImdbId {
    fn to_string(&self) -> String {
        format!("tt{0:7}", self.0)
    }
}