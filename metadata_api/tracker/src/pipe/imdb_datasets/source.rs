use super::DatasetKind;
use crate::pipe::Pipe;
use bytes::{Bytes, BytesMut};
use derive_more::{Display, Error, From};
use async_compression::stream::GzipDecoder;
use async_trait::async_trait;
use futures::stream::Stream;
use tokio_util::codec;
use std::pin::Pin;
use std::task::{Poll, Context};
use std::io;
use std::path::Path;

pub struct HttpPipe(reqwest::Client);

pub struct LocalFilePipe<P>(P);

#[derive(Debug, Display, Error, From)]
pub enum HttpError {
    RequestError(reqwest::Error),
    IoError(io::Error),
}

struct ReqwestStreamWrapper<S>(S);
struct GzipStreamWrapper<S>(S);
struct FileStreamWrapper<S>(S);

impl HttpPipe {
    pub fn new(client: reqwest::Client) -> Self {
        Self(client)
    }
}

impl<P: AsRef<Path>> LocalFilePipe<P> {
    pub fn new(path: P) -> Self {
        Self(path)
    }
}

#[async_trait]
impl Pipe<DatasetKind, Bytes> for HttpPipe {
    type Error = HttpError;
    type Stream = impl Stream<Item = Result<Bytes, HttpError>> + Send + Unpin;
    async fn get(&self, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
        let url = match token {
            DatasetKind::TitlePrincipals => "https://datasets.imdbws.com/title.principals.tsv.gz",
            DatasetKind::NameBasics => "https://datasets.imdbws.com/name.basics.tsv.gz",
            DatasetKind::TitleAkas => "https://datasets.imdbws.com/title.akas.tsv.gz",
            DatasetKind::TitleBasics => "https://datasets.imdbws.com/title.basics.tsv.gz",
            DatasetKind::TitleCrew => "https://datasets.imdbws.com/title.crew.tsv.gz",
            DatasetKind::TitleEpisode => "https://datasets.imdbws.com/title.episode.tsv.gz",
            DatasetKind::TitleRatings => "https://datasets.imdbws.com/title.ratings.tsv.gz",
        };
        Ok(GzipStreamWrapper(GzipDecoder::new(ReqwestStreamWrapper(
            self.0
                .get(url)
                .send()
                .await?
                .error_for_status()?
                .bytes_stream()
        ))))
    }
}

#[async_trait]
impl<P: AsRef<Path> + Send + Sync> Pipe<DatasetKind, Bytes> for LocalFilePipe<P> {
    type Error = io::Error;
    type Stream = impl Stream<Item = Result<Bytes, io::Error>> + Send + Unpin;
    async fn get(&self, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
        let path = match token {
            DatasetKind::TitlePrincipals => self.0.as_ref().join("title.principals.tsv.gz"),
            DatasetKind::NameBasics => self.0.as_ref().join("name.basics.tsv.gz"),
            DatasetKind::TitleAkas => self.0.as_ref().join("title.akas.tsv.gz"),
            DatasetKind::TitleBasics => self.0.as_ref().join("title.basics.tsv.gz"),
            DatasetKind::TitleCrew => self.0.as_ref().join("title.crew.tsv.gz"),
            DatasetKind::TitleEpisode => self.0.as_ref().join("title.episode.tsv.gz"),
            DatasetKind::TitleRatings => self.0.as_ref().join("title.ratings.tsv.gz"),
        };
        Ok(GzipDecoder::new(FileStreamWrapper(codec::FramedRead::new(
            tokio::fs::File::open(path).await?,
            codec::BytesCodec::new(),
        ))))
    }
}

impl<S> Stream for ReqwestStreamWrapper<S>
where
    S: Stream<Item = Result<Bytes, reqwest::Error>> + Unpin + Send,
{
    type Item = Result<Bytes, io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match futures::ready!(Pin::new(&mut self.0).poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
            None => Poll::Ready(None),
            Some(Err(e)) => Poll::Ready(Some(Err(io::Error::new(io::ErrorKind::Other, e)))),
        }
    }
}

impl<S> Stream for GzipStreamWrapper<S>
where
    S: Stream<Item = Result<Bytes, io::Error>> + Unpin + Send,
{
    type Item = Result<Bytes, HttpError>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match futures::ready!(Pin::new(&mut self.0).poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
            None => Poll::Ready(None),
            Some(Err(e)) => Poll::Ready(Some(Err(HttpError::from(e)))),
        }
    }
}

impl<S> Stream for FileStreamWrapper<S>
where
    S: Stream<Item = Result<BytesMut, io::Error>> + Unpin + Send,
{
    type Item = Result<Bytes, io::Error>;
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match futures::ready!(Pin::new(&mut self.0).poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk.freeze()))),
            None => Poll::Ready(None),
            Some(Err(e)) => Poll::Ready(Some(Err(e))),
        }
    }
}