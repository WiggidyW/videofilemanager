use async_trait::async_trait;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::future::Future;
use futures::stream::Stream;

pub type ShouldRefresh = bool;
pub type Timestamp = i64;

#[async_trait]
pub trait Client<P> {
    const KIND: &'static str;
    type Error: std::error::Error + 'static;
    type Item: Serialize + DeserializeOwned;
    type Future: Future<Output = Result<Self::Item, Self::Error>>;
    type Stream: Stream<Item = Self::Future>;
    async fn get(&self, p: P) -> Result<Self::Stream, Self::Error>;
    type Output: Serialize;
    async fn merge<E>(
        &self,
        items: impl Stream<Item = Result<(Self::Item, Timestamp), E>>,
    ) -> Result<(Self::Output, ShouldRefresh), Self::Error>;
}