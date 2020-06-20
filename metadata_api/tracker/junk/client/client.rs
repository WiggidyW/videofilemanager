use async_trait::async_trait;
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::future::Future;
use futures::stream::Stream;
use std::sync::Arc;
use mongodb::Collection;

pub type ShouldRefresh = bool;
pub type Timestamp = i64;

#[async_trait]
pub trait Client<'de, P> {
    const KIND: &'static str;
    type Error: std::error::Error + 'static;
    type Item: Serialize + Deserialize<'de>;
    type Future: Future<Output = Result<Self::Item, Self::Error>>;
    type Stream: Stream<Item = Self::Future>;
    async fn get(&self, p: P) -> Result<Self::Stream, Self::Error>;
    type Output: Serialize;
    async fn merge<E>(
        &self,
        items: impl Stream<Item = Result<(Self::Item, Timestamp), E>>,
    ) -> Result<(Self::Output, ShouldRefresh), Self::Error>;
}

#[async_trait]
pub trait Pipe<T, D> {
    type Error: std::error::Error + 'static;
    type Stream: Stream<Item = Result<D, Self::Error>>;
    async fn get(&self, token: T) -> Result<Self::Stream, Self::Error>;
}

// pub struct Pillar<C, P> {
//     client: Arc<C>,
//     conn: mongodb::Collection,
//     p: std::marker::PhantomData<P>,
// }

// impl<C, P> ClientWrapper<C, P>
// where
//     C: Client<P>,
//     P: Into<Bson>,
// {
//     async fn get
// }