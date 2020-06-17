use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use futures::stream::Stream;
use std::future::Future;
use serde_json::Value as Json;
use mongodb::bson::Bson;

pub struct ShouldRefresh(bool);
pub type Timestamp = u64;

#[async_trait]
pub trait Client<'de, T> {
    const KIND: &'static str;
    type Error: std::error::Error + 'static;
    type Item: Serialize + Deserialize<'de>;
    type Future: Future<Output = Result<Self::Item, Self::Error>>;
    type Stream: Stream<Item = Self::Future>;
    async fn get(t: T) -> Result<Self::Stream, Self::Error>;
    type Output: Serialize;
    async fn merge<E>(
        items: impl Stream<Item = Result<(T, Timestamp), E>>
    ) -> Result<(Self::Output, ShouldRefresh), Self::Error>;
}

struct Item<T> {
    id: T,
    kind: &'static str,
    timestamp: Timestamp,
    data: Bson,
}