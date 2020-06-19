use crate::pipe::Pipe;
use super::{DatasetKind, Rows};
use bytes::{Bytes, BytesMut};
use async_trait::async_trait;
use tokio::task::JoinHandle;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use futures::stream::StreamExt;

pub struct RowsPipe<P> {
    inner: Arc<P>,
    streams: Mutex<Vec<JoinHandle<()>>>,
}

struct Buf(BytesMut);

impl<P> RowsPipe<P>
where
    P: Pipe<DatasetKind, Bytes> + 'static,
    <P as Pipe<DatasetKind, Bytes>>::Stream: Send,
{
    pub fn new(pipe: P) -> Self {
        Self {
            inner: Arc::new(pipe),
            streams: Mutex::new(Vec::with_capacity(7)),
        }
    }
}

#[async_trait]
impl<P, T> Pipe<T, Rows> for RowsPipe<P>
where
    P: Pipe<DatasetKind, Bytes> + 'static,
    <P as Pipe<DatasetKind, Bytes>>::Stream: Send,
    T: Send + 'static,
{
    type Error = <P as Pipe<DatasetKind, Bytes>>::Error;
    type Stream = mpsc::UnboundedReceiver<Result<Rows, Self::Error>>;
    async fn get(&self, _: T) -> Result<Self::Stream, Self::Error> {
        let mut streams = self.streams.lock().unwrap();
        streams.clear(); // drops all existing streams
        let (tx, rx) = mpsc::unbounded_channel();
        *streams = vec![
            DatasetKind::TitlePrincipals,
            DatasetKind::NameBasics,
            DatasetKind::TitleAkas,
            DatasetKind::TitleBasics,
            DatasetKind::TitleCrew,
            DatasetKind::TitleEpisode,
            DatasetKind::TitleRatings,
        ]
            .into_iter()
            .map(|kind| {
                let pipe = self.inner.clone();
                let tx = tx.clone();
                tokio::spawn(async move {
                    let mut buf = Buf::new();
                    let mut stream = match pipe.get(kind).await {
                        Ok(s) => s,
                        Err(e) => return tx.send(Err(e)).unwrap(),
                    };
                    while let Some(b) = stream.next().await {
                        let b = b.map(|b| {
                            buf.extend(b);
                            buf.split_rows(kind)
                        });
                        tx.send(b).unwrap();
                    }
                })
            })
            .collect();
        Ok(rx)
    }
}

impl Buf {
    fn new() -> Self {
        Self(BytesMut::new())
    }
    fn extend(&mut self, bytes: Bytes) {
        self.0.extend_from_slice(&bytes);
    }
    fn split_rows(&mut self, kind: DatasetKind) -> Rows {
        let buf = &mut self.0;
        let bytes = match buf.iter()
            .enumerate()
            .rev()
            .find(|(_, &b)| b == b'\n')
        {
            None => buf.split().freeze(),
            Some((i, _)) => buf.split_to(i).freeze(),
        };
        Rows {
            inner: bytes,
            kind: kind,
        }
    }
}