pub mod writer;

pub type Error = writer::MongoError;
pub type Transaction = writer::MongoTransaction;
pub type Writer = writer::ImplWriter<writer::MongoWriter>;
pub type Args = mongodb::Database;