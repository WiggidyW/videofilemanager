use crate::Dataset;
use crate::Error;
use serde::Serialize;
use tokio::stream::Stream as AsyncStream;
use tokio::stream::StreamExt;
use bytes::Bytes;
use std::sync::Arc;

#[derive(Serialize)]
struct Row<'a> {
    kind: Dataset,
    data: Vec<&'a str>,
}

pub struct Writer(Arc<writer::Writer>);

impl<'a> Row<'a> {
    fn try_from_many(
        kind: Dataset,
        bytes: &'a Bytes,
    ) -> Result<Vec<Self>, Error>
    {
        unimplemented!()
    }
}

impl Writer {
    pub fn new(writer: writer::Writer) -> Self {
        Self(Arc::new(writer))
    }
    pub async fn write<S>(self, stream: S) -> Result<(), Error>
    where
        S: AsyncStream<Item = Result<(Dataset, Bytes), Error>> + Unpin,
    {
        let transaction = Arc::new(
            self.0.transaction("imdb_datasets").await?
        );
        let writer = self.0.clone();
        let tasks = stream.map(|res| {
            let (w, tx) = (writer.clone(), transaction.clone());
            tokio::spawn(async move {
                let (kind, bytes) = res?;
                let rows = Row::try_from_many(kind, &bytes)?;
                w.insert(&tx, rows)
                    .await
                    .map_err(|e| Error::from(e))
            })
        })
            .collect::<Vec<_>>()
            .await;
        for task in tasks {
            task.await??;
        }
        self.0.commit(Arc::try_unwrap(transaction).unwrap()).await?;
        Ok(())
    }
}