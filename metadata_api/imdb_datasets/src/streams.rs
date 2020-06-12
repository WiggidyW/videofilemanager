use async_compression::stream::GzipDecoder;
use futures_core::{stream::Stream as AsyncStream, task::{Poll, Context}};
use bytes::Bytes;
use std::{io, pin::Pin};

struct ReqwestByteStreamWrapper<T>(T);

struct Stream<T>(GzipDecoder<T>);

impl<T> AsyncStream for ReqwestByteStreamWrapper<T>
where
    T: AsyncStream<Item = Result<Bytes, reqwest::Error>> + Unpin,
{
    type Item = Result<Bytes, io::Error>;
    fn poll_next(
        mut self: Pin<&mut Self>,
        cx: &mut Context,
    ) -> Poll<Option<Self::Item>>
    {
        match futures_core::ready!(
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