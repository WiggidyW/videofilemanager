#![allow(dead_code)]
#![feature(type_alias_impl_trait)]

pub mod pipes;
pub mod tokens;

use async_trait::async_trait;
use futures::stream::Stream;
use std::sync::Arc;

#[async_trait]
pub trait Pipe<T, D>: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    type Stream: Stream<Item = Result<D, Self::Error>> + Send + Unpin;
    async fn pull(self: &Arc<Self>, token: T) -> Result<Self::Stream, Self::Error>;
}