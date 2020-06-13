use tokio::{stream::{StreamMap, Stream as AsyncStream}};
use std::{io, pin::Pin, task::{Poll, Context}};
use async_compression::stream::GzipDecoder;
use futures::future::try_join_all;
use bytes::{Bytes, BytesMut};
use crate::Dataset;

pub async fn request_stream() -> Result<
    impl AsyncStream<Item = Result<(Dataset, Bytes), io::Error>> + Unpin,
    reqwest::Error,
> {
    try_join_all(
        Dataset::iter().map(|d| async move {
            reqwest::get(reqwest::Url::from(d))
                .await?
                .error_for_status()
                .map(|resp| (d, GzipDecoder::new(
                    ByteStreamWrapper(resp.bytes_stream())
                )))
        })
    )
        .await
        .map(|strms| Stream::new(strms))
}

struct ByteStreamWrapper<T>(T);

struct Stream<T> {
    inner: StreamMap<Dataset, GzipDecoder<T>>,
    buf: [BytesMut; 7],
}

impl<T> AsyncStream for ByteStreamWrapper<T>
where
    T: AsyncStream<Item = Result<Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<Bytes, io::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>>
    {
        match futures::ready!(
            Pin::new(&mut self.0).poll_next(cx)
        ) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
            None => Poll::Ready(None),
            Some(Err(e)) => Poll::Ready(Some(Err(
                io::Error::new(io::ErrorKind::Other, e)
            ))),
        }
    }
}

impl<T> Stream<T>
where
    T: AsyncStream<Item = Result<Bytes, io::Error>> + Unpin,
{
    fn new(streams: Vec<(Dataset, GzipDecoder<T>)>) -> Self {
        Self {
            inner: {
                let mut inner = StreamMap::new();
                streams.into_iter()
                    .for_each(|(d, s)| { inner.insert(d, s); });
                inner
            },
            buf: <[BytesMut; 7]>::default(),
        }
    }

    fn buffer_it(&mut self, kind: Dataset, chunk: Bytes) -> Bytes {
        let buf = &mut self.buf[*kind.as_ref()];
        buf.extend_from_slice(&chunk);
        match buf.iter()
            .enumerate()
            .rev()
            .find(|(_, &b)| b == b'\n')
        {
            None => buf.split().freeze(),
            Some((i, _)) => buf.split_to(i).freeze(),
        }
    }
}

impl<T> AsyncStream for Stream<T>
where
    T: AsyncStream<Item = Result<Bytes, io::Error>> + Unpin,
{
    type Item = Result<(Dataset, Bytes), io::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>>
    {
        match futures::ready!(
            Pin::new(&mut self.inner).poll_next(cx)
        ) {
            Some((kind, Ok(chunk))) => Poll::Ready(Some(Ok(
                (kind, self.buffer_it(kind, chunk))
            ))),
            None => Poll::Ready(None),
            Some((_, Err(e))) => Poll::Ready(Some(Err(e))),
        }
    }
}