use crate::Dataset;
use crate::Error;
use serde::Serialize;
use tokio::stream::Stream as AsyncStream;
use tokio::stream::StreamExt;
use bytes::Bytes;
use std::sync::Arc;
use db_writer::DbWriter;

#[derive(Serialize)]
struct Row<'a> {
    kind: Dataset,
    data: Vec<&'a str>,
}

pub struct Writer<W>(Arc<W>);

impl<'a> Row<'a> {
    fn try_from_many(
        kind: Dataset,
        bytes: &'a Bytes,
    ) -> Result<Vec<Self>, Error>
    {
        let rows = std::str::from_utf8(bytes)?
            .split('\n')
            .map(|row| row.split('\t'))
            .map(|row| Row {
                kind: kind,
                data: row.collect()
            })
            .collect();
        Ok(rows)
    }
}

impl<W: DbWriter> Writer<W> {
    pub fn new(writer: Arc<W>) -> Self {
        Self(writer)
    }
    pub async fn write<S>(self, stream: S) -> Result<(), Error>
    where
        S: AsyncStream<Item = Result<(Dataset, Bytes), Error>> + Unpin,
    {
        let transaction = Arc::new(
            self.0.transaction("imdb_datasets")
                .await
                .map_err(|e| Error::writer(e))?
        );
        let writer = self.0.clone();
        let tasks = stream.map(|res| {
            let (w, tx) = (writer.clone(), transaction.clone());
            tokio::spawn(async move {
                let (kind, bytes) = res?;
                let rows = Row::try_from_many(kind, &bytes)?;
                w.insert(&tx, rows)
                    .await
                    .map_err(|e| Error::writer(e))
            })
        })
            .collect::<Vec<_>>()
            .await;
        for task in tasks {
            task.await??;
        }
        self.0.commit(Arc::try_unwrap(transaction).unwrap())
            .await
            .map_err(|e| Error::writer(e))?;
        Ok(())
    }
}