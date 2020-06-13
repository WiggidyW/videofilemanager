use std::io;
use std::pin::Pin;
use std::task::Poll;
use std::task::Context;

use tokio::stream::Stream as AsyncStream;
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::stream::StreamExt;

use async_compression::stream::GzipDecoder;

use bytes::Bytes;
use bytes::BytesMut;

use futures::future::try_join_all;

use crate::Dataset;
use crate::Error;

pub async fn request_stream() -> Result<
    impl AsyncStream<Item = Result<(Dataset, Bytes), Error>> + Unpin,
    Error,
> {
    let responses = try_join_all(Dataset::iter()
        .map(|d| async move {
            reqwest::get(reqwest::Url::from(d))
                .await?
                .error_for_status()
                .map(|res| Response {
                    inner: res,
                    kind: d,
                })
                .map_err(|e| Error::from(e))
        })
    )
        .await?;
    Ok(Stream::new(responses.into_iter()))
}

struct Stream {
    inner: mpsc::UnboundedReceiver<Result<(Dataset, Bytes), Error>>,
    tasks: Vec<JoinHandle<()>>,
}

struct PartialStream<T> {
    inner: GzipDecoder<T>,
    kind: Dataset,
    buf: BytesMut,
}

struct ByteStreamWrapper<T>(T);

struct Response {
    inner: reqwest::Response,
    kind: Dataset,
}

impl Stream {
    fn new(streams: impl Iterator<Item = Response>) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let tasks = streams
            .map(|res| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    let mut stream = PartialStream::new(
                        res.kind,
                        ByteStreamWrapper(res.inner.bytes_stream()),
                    );
                    while let Some(data) = stream.next().await {
                        tx.send(data).unwrap();
                    }
                })
            })
            .collect();
        Self {
            inner: rx,
            tasks: tasks,
        }
    }
}

impl AsyncStream for Stream {
    type Item = Result<(Dataset, Bytes), Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>>
    {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}

impl<T> PartialStream<T>
where
    T: AsyncStream<Item = Result<Bytes, io::Error>> + Unpin,
{
    fn new(kind: Dataset, stream: T) -> Self {
        Self {
            inner: GzipDecoder::new(stream),
            kind: kind,
            buf: BytesMut::new(),
        }
    }
    fn buffer_it(&mut self, chunk: Bytes) -> Bytes {
        let buf = &mut self.buf;
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

impl<T> AsyncStream for PartialStream<T>
where
    T: AsyncStream<Item = Result<Bytes, io::Error>> + Unpin,
{
    type Item = Result<(Dataset, Bytes), Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>>
    {
        match futures::ready!(
            Pin::new(&mut self.inner).poll_next(cx)
        ) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(
                (self.kind, self.buffer_it(chunk))
            ))),
            None => Poll::Ready(None),
            Some(Err(e)) => Poll::Ready(Some(Err(Error::from(e)))),
        }
    }
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