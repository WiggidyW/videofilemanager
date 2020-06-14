mod stream;
mod kind;
mod writer;
mod error;

pub use error::Error;
pub(crate) use kind::Dataset;
pub(crate) use stream::request_stream;

pub async fn refresh<W: db_writer::DbWriter>(writer: W) -> Result<(), Error> {
    let stream = request_stream().await?;
    let writer = writer::Writer::new(writer);
    writer.write(stream).await?;
    Ok(())
}