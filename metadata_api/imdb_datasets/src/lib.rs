mod stream;
mod kind;
mod writer;
mod error;

pub use error::Error;
pub use writer::DbWriter;
pub(crate) use kind::Dataset;
pub(crate) use stream::request_stream;

use std::sync::Arc;

pub async fn refresh<W: DbWriter>(writer: Arc<W>) -> Result<(), Error> {
    let stream = request_stream().await?;
    let writer = writer::Writer::new(writer);
    writer.write(stream).await?;
    Ok(())
}