mod stream;
mod kind;
mod write;
mod error;

pub use error::Error;
pub(crate) use kind::Dataset;
pub(crate) use stream::request_stream;

pub async fn refresh(writer: writer::Writer) -> Result<(), Error> {
    let stream = request_stream().await?;
    let writer = write::Writer::new(writer);
    writer.write(stream).await?;
    Ok(())
}