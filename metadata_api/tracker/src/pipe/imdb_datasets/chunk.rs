use super::DatasetKind;
use derive_more::{Display, Error, From};
use std::io;
use std::collections::HashMap;

pub struct HttpPipe {
    client: reqwest::Client,
}

#[derive(Debug, Display, Error, From)]
pub enum HttpPipeError {
    RequestError(reqwest::Error),
    #[from(ignore)]
    NetworkStreamError(Box<reqwest::Error>),
    #[from(ignore)]
    IoError(io::Error),
}

pub struct LocalFilePipe<P> {
    file_map: HashMap<DatasetKind, P>,
}

#[derive(Debug, Display, Error, From)]
pub enum LocalFilePipeError {
    IoError(io::Error),
}

mod http_pipe {
    use super::{HttpPipe, HttpPipeError};
    use crate::pipe::imdb_datasets::{DatasetKind, Chunk};
    use crate::pipe::Pipe;
    use async_compression::stream::GzipDecoder;
    use futures::stream::{Stream, StreamExt};
    use async_trait::async_trait;
    use std::sync::Arc;
    use std::io;

    #[tokio::test]
    async fn test_error() {
        let err = reqwest::get("invalid url").await.unwrap_err();
        let err = io::Error::new(io::ErrorKind::Other, err);
        let err = HttpPipeError::from(err);
        println!("{:?}", err);
    }

    impl From<io::Error> for HttpPipeError {
        fn from(v: io::Error) -> Self {
            match v.get_ref().map(|e| e.is::<reqwest::Error>()) {
                Some(true) => Self::NetworkStreamError(
                    v.into_inner()
                        .unwrap()
                        .downcast::<reqwest::Error>()
                        .unwrap()
                ),
                _ => Self::IoError(v),
            }
        }
    }

    #[async_trait]
    impl Pipe<DatasetKind, Chunk> for HttpPipe {
        type Error = HttpPipeError;
        type Stream = impl Stream<Item = Result<Chunk, Self::Error>> + Send + Unpin;
        async fn get(self: &Arc<Self>, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
            let url = match token {
                DatasetKind::TitlePrincipals => "https://datasets.imdbws.com/title.principals.tsv.gz",
                DatasetKind::NameBasics => "https://datasets.imdbws.com/name.basics.tsv.gz",
                DatasetKind::TitleAkas => "https://datasets.imdbws.com/title.akas.tsv.gz",
                DatasetKind::TitleBasics => "https://datasets.imdbws.com/title.basics.tsv.gz",
                DatasetKind::TitleCrew => "https://datasets.imdbws.com/title.crew.tsv.gz",
                DatasetKind::TitleEpisode => "https://datasets.imdbws.com/title.episode.tsv.gz",
                DatasetKind::TitleRatings => "https://datasets.imdbws.com/title.ratings.tsv.gz",
            };
            let stream = GzipDecoder::new(
                self.client
                    .get(url)
                    .send()
                    .await?
                    .error_for_status()?
                    .bytes_stream()
                    .map(|result| result.map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
                )
                .zip(futures::stream::repeat(token)) // allows us to use the token in the stream
                .map(|(result, token)| match result {
                    Ok(b) => Ok(Chunk::from((b, token))),
                    Err(e) => Err(HttpPipeError::from(e)),
                });
            Ok(stream)
        }
    }
}

mod local_file_pipe {
    use super::{LocalFilePipe, LocalFilePipeError};
    use crate::pipe::imdb_datasets::{DatasetKind, Chunk};
    use crate::pipe::Pipe;
    use async_compression::stream::GzipDecoder;
    use futures::stream::{Stream, StreamExt};
    use async_trait::async_trait;
    use std::path::Path;
    use std::sync::Arc;
    use std::collections::HashMap;

    impl<P: AsRef<Path> + Send + Sync> LocalFilePipe<P> {
        pub fn new(file_map: HashMap<DatasetKind, P>) -> Self {
            Self { file_map: file_map }
        }
    }

    #[async_trait]
    impl<P: AsRef<Path> + Send + Sync> Pipe<DatasetKind, Chunk> for LocalFilePipe<P> {
        type Error = LocalFilePipeError;
        type Stream = impl Stream<Item = Result<Chunk, Self::Error>> + Send + Unpin;
        async fn get(self: &Arc<Self>, token: DatasetKind) -> Result<Self::Stream, Self::Error> {
            let stream = GzipDecoder::new(
                tokio_util::codec::FramedRead::new(
                    tokio::fs::File::open(self.file_map[&token].as_ref()).await?,
                    tokio_util::codec::BytesCodec::new(),
                    )
                    .map(|result| result.map(|b| b.freeze()))
                )
                .zip(futures::stream::repeat(token)) // allows us to use the token in the stream
                .map(|(result, token)| match result {
                    Ok(b) => Ok(Chunk::from((b, token))),
                    Err(e) => Err(LocalFilePipeError::from(e)),
                });
            Ok(stream)
        }
    }
}