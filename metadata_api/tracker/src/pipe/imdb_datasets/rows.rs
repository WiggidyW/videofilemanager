use std::sync::Arc;
use derive_more::{Display, Error};

#[derive(Debug, Clone)]
pub struct ChunkRowPipe<P> {
    chunk_pipe: Arc<P>,
}

#[derive(Debug, Display, Error)]
pub enum ChunkRowPipeError<E> {
    ChunkPipeError(E),
}

mod chunk_row_pipe {
    use super::{ChunkRowPipe, ChunkRowPipeError};
    use crate::pipe::imdb_datasets::{DatasetKind, Chunk, ChunkRow, ChunkExtra};
    use crate::pipe::Pipe;
    use async_trait::async_trait;
    use std::sync::Arc;
    use bytes::{Bytes, BytesMut};
    use futures::stream::Stream;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::collections::VecDeque;

    struct ChunkRowStream<S> {
        stream: S,
        prev: ChunkExtra,
        rows: std::vec::IntoIter<ChunkRow>,
    }

    impl<P: Pipe<DatasetKind, Chunk>> ChunkRowPipe<P> {
        pub fn new(p: Arc<P>) -> Self {
            Self { chunk_pipe: p }
        }
    }

    #[async_trait]
    impl<P: Pipe<DatasetKind, Chunk>> Pipe<DatasetKind, ChunkRow> for ChunkRowPipe<P> {
        type Error = ChunkRowPipeError<P::Error>;
        type Stream = impl Stream<Item = Result<ChunkRow, Self::Error>> + Send + Unpin;
        async fn get(self: &Arc<Self>, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
            Ok(ChunkRowStream::new(
                self.chunk_pipe
                    .get(token)
                    .await
                    .map_err(|e| ChunkRowPipeError::ChunkPipeError(e))?
            ))
        }
    }

    impl<E, S: Stream<Item = Result<Chunk, E>> + Unpin> ChunkRowStream<S> {
        fn new(stream: S) -> Self {
            Self {
                stream: stream,
                prev: ChunkExtra::default(),
                rows: Vec::new().into_iter(),
            }
        }
        fn refresh(&mut self, chunk: Chunk) {
            self.rows = chunk
                .into_chunk_rows(&mut self.prev)
                .into_iter();
        }
        fn next(&mut self) -> Option<ChunkRow> {
            self.rows.next()
        }
    }

    impl<E, S: Stream<Item = Result<Chunk, E>> + Unpin> Stream for ChunkRowStream<S> {
        type Item = Result<ChunkRow, ChunkRowPipeError<E>>;
        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if let Some(row) = self.next() {
                return Poll::Ready(Some(Ok(row)));
            }
            match futures::ready!(Pin::new(&mut self.stream).poll_next(cx)) {
                Some(Ok(chunk)) => {
                    self.refresh(chunk);
                    self.poll_next(cx)
                },
                None => Poll::Ready(None),
                Some(Err(e)) => Poll::Ready(Some(Err(ChunkRowPipeError::ChunkPipeError(e)))),
            }
        }
    }
}