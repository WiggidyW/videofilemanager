use crate::pipe::Pipe;
use super::{DatasetKind, Rows};
use bytes::{Bytes, BytesMut};
use async_trait::async_trait;
use tokio::task::JoinHandle;
use tokio::sync::mpsc;
use futures::stream::{Stream, StreamExt};
use futures::future;
use std::sync::Mutex;

pub struct RowsPipe<P> {
    inner: P,
    tasks: Mutex<Vec<JoinHandle<()>>>,
}

impl<P> RowsPipe<P>
where
    P: Pipe<DatasetKind, Bytes> + 'static,
    <P as Pipe<DatasetKind, Bytes>>::Stream: Send,
{
    pub fn new(pipe: P) -> Self {
        Self {
            inner: pipe,
            tasks: Mutex::new(Vec::with_capacity(7)),
        }
    }
}

#[async_trait]
impl<P, T> Pipe<T, Rows> for RowsPipe<P>
where
    P: Pipe<DatasetKind, Bytes> + 'static,
    T: Send + 'static,
{
    type Error = <P as Pipe<DatasetKind, Bytes>>::Error;
    type Stream = mpsc::Receiver<Result<Rows, Self::Error>>;
    async fn get(&self, _: T) -> Result<Self::Stream, Self::Error> {
        self.tasks.lock().unwrap().clear(); // drops all existing streams
        let streams = future::try_join_all(
            DatasetKind::iter()
                .map(|kind| async move {
                    match self.inner.get(kind).await {
                        Ok(stream) => Ok((stream, kind)),
                        Err(e) => Err(e),
                    }
                })
            )
            .await?;
        let (tx, rx) = mpsc::channel(100);
        *self.tasks.lock().unwrap() = streams
            .into_iter()
            .map(|(stream, kind)| {
                let tx = tx.clone();
                tokio::spawn(async move {
                    RowTransceiver::new(stream, kind, tx).run().await;
                })
            })
            .collect();
        Ok(rx)
    }
}

struct RowTransceiver<S, E> {
    buf: BytesMut,
    stream: S,
    kind: DatasetKind,
    tx: mpsc::Sender<Result<Rows, E>>,
}

impl<S, E> RowTransceiver<S, E>
where
    S: Stream<Item = Result<Bytes, E>> + Unpin,
    E: std::fmt::Debug,
{
    fn new(stream: S, kind: DatasetKind, tx: mpsc::Sender<Result<Rows, E>>) -> Self {
        Self {
            buf: BytesMut::new(),
            stream: stream,
            kind: kind,
            tx: tx,
        }
    }
    fn extend_buf(&mut self, bytes: Bytes) {
        self.buf.extend_from_slice(&bytes);
    }
    fn split_rows(&mut self) -> Rows {
        let buf = &mut self.buf;
        let bytes = match buf
            .iter()
            .enumerate()
            .rev()
            .find(|(_, &b)| b == b'\n')
        {
            None => buf.split().freeze(),
            Some((i, _)) => buf.split_to(i + 1).freeze(),
        };
        Rows::new(bytes, self.kind)
    }
    fn discard_first_row(&mut self) {
        let buf = &mut self.buf;
        if let Some((i, _)) = buf
            .iter()
            .enumerate()
            .find(|(_, &b)| b == b'\n')
        {
            let _ = buf.split_to(i + 1);
        }
    }
    async fn run(mut self) {
        if let Some(b) = self.stream.next().await { // discard the header row
            let b = b.map(|b| {
                self.extend_buf(b);
                self.discard_first_row();
                self.split_rows()
            });
            self.tx.send(b).await.unwrap();
        }
        while let Some(b) = self.stream.next().await { // then continue until empty
            let b = b.map(|b| {
                self.extend_buf(b);
                self.split_rows()
            });
            self.tx.send(b).await.unwrap();
        }
    }
}