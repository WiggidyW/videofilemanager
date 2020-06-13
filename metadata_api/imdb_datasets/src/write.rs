use crate::Dataset;
use crate::Error;
use serde::Serialize;
use tokio::stream::Stream as AsyncStream;
use bytes::Bytes;
use std::io;

#[derive(Serialize)]
struct Row {
    kind: Dataset,
    data: Vec<String>,
}

struct Writer<'a, T> {
    stream: T,
    writer: &'a writer::MongoWriter,
}

impl<'a, T> Writer<'a, T>
where
    T: AsyncStream<Item = Result<(Dataset, Bytes), io::Error>> + Unpin,
{
    async fn write(self) -> Result<(), Error> {
        let transaction = self.writer
            .transaction("imdb_datasets")
            .await?;
        unimplemented!()
    }
    fn test() {
        ()
    }
}