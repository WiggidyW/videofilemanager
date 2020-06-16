use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use futures::stream::Stream;
use std::future::Future;

pub enum Validity {
    Good,
    Bad,
    Useless,
}

pub enum Refresh {
    Never,
    After(u64),
    Age(u64),
    Immediately,
}

pub trait Data {
    type Error: std::error::Error + 'static;
    fn valid(&self) -> Result<(), Self::Error>;
}

#[async_trait]
pub trait Client<'de, T> {
    const KIND: &'static str;
    type Error: std::error::Error + 'static;
    type Item: Serialize + Deserialize<'de> + Data;
    type Future: Future<Output = Result<Self::Item, Self::Error>>;
    type Stream: Stream<Item = Self::Future>;
    async fn get(t: T) -> Result<Self::Stream, Self::Error>;
}

struct Item<'i, 't, D> {
    id: &'i str,
    version: &'static str,
    kind: &'static str,
    transaction: &'t str,
    data: D,
}