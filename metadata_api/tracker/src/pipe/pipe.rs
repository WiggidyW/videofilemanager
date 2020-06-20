use async_trait::async_trait;
use futures::stream::Stream;

#[async_trait]
pub trait Pipe<T, D>: Send + Sync {
    type Error: std::error::Error + Send + 'static;
    type Stream: Stream<Item = Result<D, Self::Error>> + Send + Unpin;
    async fn get(&self, token: T) -> Result<Self::Stream, Self::Error>;
}