use std::error::Error as StdError;
use std::time::{self, SystemTime};
use std::fmt::Debug;
use serde::Serialize;
use async_trait::async_trait;
use derive_more::{Display, Error, Constructor, From};
use mongodb::bson;

#[async_trait]
pub trait DbWriter: Debug + Send + Sync + 'static {
    type Error: StdError + Send + 'static;
    type Transaction: Debug + Send + Sync + 'static;
    async fn transaction(
        &self,
        kind: &str,
    ) -> Result<Self::Transaction, Self::Error>;
    async fn insert<D: Serialize, I: IntoIterator<Item = D> + Send>(
        &self,
        transaction: &Self::Transaction,
        data: I,
    ) -> Result<(), Self::Error>;
    async fn commit(
        &self,
        transaction: Self::Transaction,
    ) -> Result<(), Self::Error>;
}

#[derive(Debug)]
pub struct MongoWriter {
    inner: mongodb::Database,
}

#[derive(Debug, Constructor)]
pub struct MongoTransaction {
    inner: mongodb::Collection,
    commit: bool,
}

#[derive(Debug, Display, Error, From)]
pub enum MongoError {
    SystemTimeError(time::SystemTimeError),
    DatabaseError(mongodb::error::Error),
    SerializeError(bson::ser::Error),
    #[display(fmt = "Invalid Document: {}", "_0.to_string()")]
    BsonAsDocumentError(
        #[error(not(source))]
        bson::Bson
    ),
}

#[async_trait]
impl DbWriter for MongoWriter {
    type Error = MongoError;
    type Transaction = MongoTransaction;
    async fn transaction(
        &self,
        kind: &str,
    ) -> Result<Self::Transaction, Self::Error>
    {
        let name = format!("{}_{}", kind, SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs()
        );
        self.inner.create_collection(&name, None).await?;
        Ok(MongoTransaction::new(self.inner.collection(&name), false))
    }
    async fn insert<D: Serialize, I: IntoIterator<Item = D> + Send>(
        &self,
        transaction: &Self::Transaction,
        data: I,
    ) -> Result<(), Self::Error>
    {
        let data: Vec<bson::Document> = data.into_iter()
            .map(|data| bson::ser::to_bson(&data)
                .map_err(|e| MongoError::from(e))
                .and_then(|data| match data {
                    bson::Bson::Document(doc) => Ok(doc),
                    _ => Err(MongoError::BsonAsDocumentError(data)),
                })
            )
            .collect::<Result<Vec<bson::Document>, Self::Error>>()?;
        transaction.inner.insert_many(data, None)
            .await?;
        Ok(())
    }
    async fn commit(
        &self,
        mut transaction: Self::Transaction,
    ) -> Result<(), Self::Error>
    {
        transaction.commit = true;
        Ok(())
    }
}

#[allow(unused_must_use)]
impl Drop for MongoTransaction {
    fn drop(&mut self) {
        if !self.commit {
            smol::block_on(self.inner.drop(None));
        }
    }
}