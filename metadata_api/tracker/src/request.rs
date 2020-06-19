use serde::{Serialize, Deserialize, de::DeserializeOwned};
use async_trait::async_trait;
use futures::stream::{Stream, StreamExt};
use std::future::Future;
use serde_json::Value as Json;
use mongodb::bson::{self, Bson, Document};
use mongodb::Collection;

pub enum ClientError {
    DatabaseError(mongodb::error::Error),
    ClientError(Box<dyn std::error::Error>),
    SerializeError(serde_json::error::Error),
}

struct ClientWrapper<C, P> {
    inner: C,
    conn: Collection,
    p: std::marker::PhantomData<P>,
}

impl<C, P> ClientWrapper<C, P>
where
    C: Client<P>,
    P: Send + Sync + Clone + Into<Bson> + 'static,
{
    async fn get(&self, p: P) -> Result<Json, ClientError> {
        enum DataError {
            DatabaseError(mongodb::error::Error),
            DeserializeError(bson::de::Error),
            InvalidTimestamp(bson::document::ValueAccessError),
            MissingData,
        }

        let get_stored = |conn: Collection, query: Document| async move {
            Result::<_, ClientError>::Ok(
                conn.find(
                    query,
                    None,
                )
                .await // Result<Cursor, Error>
                .map_err(|e| ClientError::DatabaseError(e))? // Cursor
                .map(|document| {
                    let mut document: Document = document
                        .map_err(|e| DataError::DatabaseError(e))?;
                    let timestamp: Timestamp = document.get_i64("timestamp")
                        .map_err(|e| DataError::InvalidTimestamp(e))?;
                    let bson_data: Bson = document.remove("data")
                        .ok_or(DataError::MissingData)?;
                    let data = bson::de::from_bson(bson_data)
                        .map_err(|e| DataError::DeserializeError(e))?;
                    Result::<_, DataError>::Ok((data, timestamp))
                }) // Stream<Output = Result<(Data, Timestamp), DataError>>
            ) // Result<Stream<...>, ClientError>
        };

        let query: Document = bson::doc! {
            "id": p.clone().into(),
            "kind": C::KIND,
        };

        match self.inner.merge(
            get_stored(self.conn.clone(), query.clone()).await?
            ).await
            .map_err(|e| ClientError::ClientError(Box::new(e)))?
        {
            (output, false) => serde_json::to_value(output)
                .map_err(|e| ClientError::SerializeError(e)),
            (_, true) => serde_json::to_value(
                self.inner
                    .merge(
                        get_stored(self.conn.clone(), query.clone()).await?
                    ).await
                    .map_err(|e| ClientError::ClientError(Box::new(e)))?
                    .0
                ).map_err(|e| ClientError::SerializeError(e)),
        }
    }
}

struct Item<T> {
    id: T,
    kind: &'static str,
    timestamp: Timestamp,
    data: Bson,
}