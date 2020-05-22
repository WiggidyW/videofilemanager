use futures_core::{stream::Stream, task::{Poll, Context}};
use std::{marker::{Unpin, PhantomData}, pin::Pin, iter::Iterator};
use async_compression::stream::GzipDecoder;
use bytes::{Bytes, BytesMut, buf::BufMut};
use crate::error::Error;
use derive_more::{Deref, DerefMut};

// Visible
pub trait Dataset: Sized + Unpin {
	type Url: reqwest::IntoUrl;
	fn url() -> Self::Url;
	type Error: std::error::Error + 'static;
	fn deserialize(b: Bytes) -> Result<Vec<Self>, Self::Error>;
}

pub async fn request<D: Dataset>() -> Result<Response<D>, Error> {
	Ok(Response::new(
		reqwest::get(D::url())
			.await
			.map_err(|e| Error::request_error(e))?
			.error_for_status()
			.map_err(|e| Error::request_error(e))?
	))
}

pub struct Response<D> {
	inner: reqwest::Response,
	kind: PhantomData<D>,
}

impl<D: Dataset + Sized> Response<D> {
	pub fn stream(self) -> impl Stream<Item = Result<Vec<D>, Error>> {
		RecordStream::new(self.into_raw_byte_stream())
	}
}
// End Visible

// Private
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

#[derive(Deref, DerefMut)]
struct RawByteStream {
	inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>>,
}

#[derive(Deref, DerefMut)]
struct RecordStream<D> {
	#[deref]
	#[deref_mut]
	inner: GzipDecoder<RawByteStream>,
	kind: PhantomData<D>,
	buf: BytesMut,
	init: bool,
}

impl RawByteStream {
	fn new<S>(inner: S) -> Self where
		S: Stream<Item = Result<Bytes, reqwest::Error>>,
		S: 'static,
	{
		Self {
			inner: Box::pin(inner),
		}
	}
}

impl Stream for RawByteStream {
	type Item = Result<Bytes, std::io::Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		// Through sheer magic, (&mut **self).as_mut() is a Pin<&mut dyn Stream>. No box.
		match futures_core::ready!((&mut **self).as_mut().poll_next(cx)) {
			None => Poll::Ready(None),
			Some(Err(e)) =>
				Poll::Ready(Some(Err(
					std::io::Error::new(std::io::ErrorKind::Other, e)
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

	// - Mutates the buffer, splitting off the first row.
	fn split_first_terminator(&mut self) {
		let iter = (&self.buf)
			.into_iter()
			.enumerate();
		for (i, b) in iter {
			if b == crate::CSV_TERM {
				let _ = self.buf.split_to(i + 1);
				break;
			}
		}
	}

	// - Mutates the buffer, splitting off all but the final row.
	// - Returns those bytes.
	fn split_last_terminator(&mut self) -> BytesMut {
		let iter = (&self.buf)
			.into_iter()
			.enumerate()
			.rev();
		for (i, b) in iter {
			if b == crate::CSV_TERM {
				return self.buf.split_to(i + 1);
			}
		}
		self.buf.split()
	}

	fn parse(&mut self, b: Bytes) -> Bytes {
		self.buf.put(b);
		if !self.init {
			self.split_first_terminator();
			self.init = true;
		}
		self.split_last_terminator()
			.freeze()
	}
}

impl<D: Dataset> Stream for RecordStream<D> {
	type Item = Result<Vec<D>, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		// **self resolves to inner GzipDecoder
		match futures_core::ready!(Pin::new(&mut **self).poll_next(cx)) {
			None => Poll::Ready(None),
			Some(Err(e)) => Poll::Ready(Some(Err(Error::stream_error(e)))),
			Some(Ok(chunk)) =>
		match D::deserialize(self.parse(chunk)) {
			Ok(rows) => Poll::Ready(Some(Ok(rows))),
			Err(e) => Poll::Ready(Some(Err(Error::deser_error(e)))),
		},}
	}
}
// End Private