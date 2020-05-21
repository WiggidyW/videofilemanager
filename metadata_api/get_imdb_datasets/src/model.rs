use async_compression::stream::GzipDecoder;
use bytes::{Bytes, BytesMut, buf::BufMut};
use futures_core::{stream::Stream, task::{Poll, Context}};
use std::{marker::{Unpin, PhantomData}, pin::Pin, ops::{Deref, DerefMut}};
use derive_more::*;

pub async fn request<D: Dataset>() -> Result<Response<D>, reqwest::Error> {
	Ok(Response::new(
		reqwest::get(D::url())
			.await?
			.error_for_status()?
	))
}

pub trait Dataset: Sized {
	type Error;
	type Url: reqwest::IntoUrl;
	fn url() -> Self::Url;
	fn deserialize(b: Bytes) -> Result<Vec<Self>, Self::Error>;
}

pub struct Response<D> {
	inner: reqwest::Response,
	kind: PhantomData<D>,
}

pub struct RawByteStream {
	inner: Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>,
}

#[derive(Deref)]
pub struct RecordStream<D> {
	#[deref]
	inner: GzipDecoder<RawByteStream>,
	kind: PhantomData<D>,
	buf: BytesMut,
	init: bool,
}

impl<D> Response<D> {
	fn new(inner: reqwest::Response) -> Self {
		Self {
			inner: inner,
			kind: PhantomData::<D>::default(),
		}
	}

	fn into_raw_byte_stream(self) -> RawByteStream {
		RawByteStream::new(self.inner.bytes_stream())
	}
}

impl RawByteStream {
	fn new<S>(inner: S) -> Self where
		S: Stream<Item = Result<Bytes, reqwest::Error>>,
		S: 'static,
	{
		Self {
			inner: Box::new(inner),
		}
	}
}

impl Stream for RawByteStream {
	type Item = Result<Bytes, std::io::Error>;
	#[allow(unconditional_recursion)] // This warning is due to a bug.
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(self.as_mut().poll_next(cx)) {
			None => Poll::Ready(None),
			Some(Err(e)) =>
				Poll::Ready(Some(Err(
					std::io::Error::new(crate::error::ERR_REQWEST_TO_IO, e)
				))),
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
		}
	}
}

impl<D> RecordStream<D> {
	fn new(inner: RawByteStream) -> Self {
		Self {
			inner: GzipDecoder::new(inner),
			kind: PhantomData::<D>::default(),
			buf: BytesMut::new(),
			init: false,
		}
	}

	// - Returns Bytes following the first terminator in the buffer.
	// - Clears the buffer.
	// - Sets init to true.
	fn split_first_terminator(&mut self) -> BytesMut {
		unimplemented!()
	}

	// - Returns Bytes preceding the final terminator in the buffer.
	// - Splits the buffer.
	fn split_last_terminator(&mut self) -> BytesMut {
		unimplemented!()
	}

	fn parse(&mut self, b: Bytes) -> Bytes {
		self.buf.put(b);
		match self.init {
			false => self.split_first_terminator()
				.freeze(),
			true => self.split_last_terminator()
				.freeze(),
		}
	}
}

// impl<D: Dataset> Stream for RecordStream<D> {
// 	type Item = Result<Vec<D>, std::io::Error>;
// 	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
// 		match futures_core::ready!(self.as_mut().poll_next(cx)) {
// 			None => Poll::Ready(None),
// 			Some(Err(e)) => unimplemented!(),
// 			Some(Ok(chunk)) =>
// 		match 
// 		}
// 		unimplemented!()
// 	}
// }