use crate::pipe::imdb_datasets::ByteRowError;
use std::sync::Arc;
use derive_more::{Display, Error};

#[derive(Debug, Clone)]
pub struct ByteRowPipe<P> {
    chunk_pipe: Arc<P>,
}

#[derive(Debug, Display, Error)]
pub enum ByteRowPipeError<E> {
    ChunkPipeError(E),
    ByteRowError(ByteRowError),
}

mod rows_pipe {
    use super::{ByteRowPipe, ByteRowPipeError};
    use crate::pipe::imdb_datasets::{DatasetKind, Chunk, ByteRow, ByteRowError};
    use crate::pipe::Pipe;
    use async_trait::async_trait;
    use std::sync::Arc;
    use bytes::{Bytes, BytesMut};
    use futures::stream::Stream;
    use std::pin::Pin;
    use std::task::{Context, Poll};
    use std::collections::VecDeque;

    impl<P: Pipe<DatasetKind, Chunk>> ByteRowPipe<P> {
        pub fn new(p: Arc<P>) -> Self {
            Self { chunk_pipe: p }
        }
    }

    #[async_trait]
    impl<P: Pipe<DatasetKind, Chunk>> Pipe<DatasetKind, ByteRow> for ByteRowPipe<P> {
        type Error = ByteRowPipeError<P::Error>;
        type Stream = impl Stream<Item = Result<ByteRow, Self::Error>> + Send + Unpin;
        async fn get(self: &Arc<Self>, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
            Ok(ByteRowStream {
                stream: self.chunk_pipe.get(token)
                    .await
                    .map_err(|e| ByteRowPipeError::ChunkPipeError(e))?,
                buf: Bytes::new(),
                rows: VecDeque::new(),
                kind: token,
            })
        }
    }

    struct ByteRowStream<S> {
        stream: S,
        buf: Bytes,
        rows: VecDeque<Bytes>,
        kind: DatasetKind,
    }

    const SPLIT: u8 = b'\n';

    impl<S> ByteRowStream<S> {
        fn push(&mut self, chunk: Bytes) {
            let mut indexes = chunk
                .iter()
                .enumerate()
                .filter_map(|(i, b)| match b == &SPLIT {
                    true => Some(i),
                    false => None
                })
                .collect::<Vec<usize>>()
                .into_iter()
                .peekable();
            match indexes.peek() {
                Some(0) if self.buf.is_empty() => (),
                Some(0) => self.rows.push_front(self.buf.split_off(0)),
                Some(i) if self.buf.is_empty() => self.rows.push_back(chunk.slice(0..*i)),
                Some(i) => {
                    let mut row = BytesMut::new();
                    row.extend_from_slice(&self.buf.split_off(0));
                    row.extend_from_slice(&chunk.slice(0..*i));
                    self.rows.push_back(row.freeze());
                },
                None if self.buf.is_empty() => self.buf = chunk,
                None => {
                    let mut buf = BytesMut::new();
                    buf.extend_from_slice(&self.buf.split_off(0));
                    buf.extend_from_slice(&chunk);
                    self.buf = buf.freeze();
                },
            }
            for index in indexes {
                match indexes.peek() {
                    Some(i) => self.rows.push_back(chunk.slice(index + 1..*i)),
                    None if index == chunk.len() - 1 => self.rows.push_back(
                        chunk.slice(index + 1..chunk.len())
                    ),
                    None => self.buf = chunk.slice(index + 1..chunk.len()),
                }
            }
        }
        fn next(&mut self) -> Option<Result<ByteRow, ByteRowError>> {
            self.rows
                .pop_front()
                .map(|b| ByteRow::new(b, self.kind))
        }
    }

    impl<E, S: Stream<Item = Result<Chunk, E>> + Unpin> Stream for ByteRowStream<S> {
        type Item = Result<ByteRow, ByteRowPipeError<E>>;
        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            if let Some(row) = self.next() {
                return Poll::Ready(Some(row.map_err(|e| ByteRowPipeError::ByteRowError(e))));
            }
            match futures::ready!(Pin::new(&mut self.stream).poll_next(cx)) {
                Some(Ok(chunk)) => {
                    self.push(chunk.bytes);
                    self.poll_next(cx)
                },
                None => Poll::Ready(None),
                Some(Err(e)) => Poll::Ready(Some(Err(ByteRowPipeError::ChunkPipeError(e)))),
            }
        }
    }
}