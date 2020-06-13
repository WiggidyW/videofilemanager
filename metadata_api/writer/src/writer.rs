use std::error::Error as StdError;
use std::time::{self, SystemTime};

use serde::Serialize;

use async_trait::async_trait;

use derive_more::{Display, Error, Constructor, From};

use mongodb::bson;

pub struct ImplWriter<T>(T);

#[async_trait]
pub trait Writer: Sized {
    type Error: StdError + 'static;
    type Args;
    type Transaction;
    fn new(args: Self::Args) -> Result<Self, Self::Error>;
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

impl<T: Writer> ImplWriter<T> {
    pub fn new(
        args: <T as Writer>::Args,
    ) -> Result<Self, <T as Writer>::Error>
    {
        T::new(args).map(|t| Self(t))
    }
    pub async fn transaction(
        &self,
        kind: &str,
    ) -> Result<<T as Writer>::Transaction, <T as Writer>::Error>
    {
        self.0.transaction(kind).await
    }
    pub async fn insert<D: Serialize, I: IntoIterator<Item = D> + Send>(
        &self,
        transaction: &<T as Writer>::Transaction,
        data: I,
    ) -> Result<(), <T as Writer>::Error>
    {
        self.0.insert(transaction, data).await
    }
    pub async fn commit(
        &self,
        transaction: <T as Writer>::Transaction,
    ) -> Result<(), <T as Writer>::Error>
    {
        self.0.commit(transaction).await
    }
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
impl Writer for MongoWriter {
    type Error = MongoError;
    type Args = mongodb::Database;
    type Transaction = MongoTransaction;
    fn new(args: Self::Args) -> Result<Self, Self::Error> {
        Ok(Self { inner: args })
    }
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