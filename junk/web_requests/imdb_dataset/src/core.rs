use async_compression::stream::GzipDecoder;
use futures_core::{
	stream::Stream,
	task::{Poll, Context},
};
use std::{
	pin::Pin,
	marker::PhantomData,
};
use bytes::Bytes;
use crate::error::Error;

async fn request<T>(url: T) -> Result<Response, Error> where
	T: reqwest::IntoUrl,
{
	Ok(Response {
		inner: reqwest::get(url)
			.await
			.map_err(|e| Error::request_error(e))?
			.error_for_status()
			.map_err(|e| Error::request_error(e))?
	})
}

struct Response {
	inner: reqwest::Response,
}

struct RawStream {
	inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>>,
}

struct PublRawStream {
	inner: RawStream,
}

struct DecodedStream {
	inner: GzipDecoder<RawStream>,
}

struct ByteRecordStream {
	inner: DecodedStream,
	row_buf: Vec<u8>,
	chunk_buf: bytes::buf::IntoIter<Bytes>,
}

struct DeserRecordStream<T> {
	inner: ByteRecordStream,
	kind: PhantomData<T>,
}

struct TextRecordStream {
	inner: ByteRecordStream,
}

struct SplitTextRecordStream {
	inner: TextRecordStream,
}

impl Response {
	fn into_raw_stream(self) -> RawStream {
		RawStream {
			inner: Box::pin(self.inner.bytes_stream()),
		}
	}
}

impl RawStream {
	fn into_decoded_stream(self) -> DecodedStream {
		DecodedStream {
			inner: GzipDecoder::new(self),
		}
	}

	fn into_publ_raw_stream(self) -> PublRawStream {
		PublRawStream {
			inner: self,
		}
	}
}

impl DecodedStream {
	fn into_byte_record_stream(self) -> ByteRecordStream {
		ByteRecordStream::new(self)
	}
}

impl ByteRecordStream {
	fn into_deser_record_stream<T>(self) -> DeserRecordStream<T> {
		DeserRecordStream {
			inner: self,
			kind: PhantomData::<T>::default(),
		}
	}

	fn into_text_record_stream(self) -> TextRecordStream {
		TextRecordStream {
			inner: self,
		}
	}

	fn new(stream: DecodedStream) -> Self {
		Self {
			inner: stream,
			row_buf: Vec::new(),
			chunk_buf: bytes::buf::IntoIter::new(Bytes::new()),
		}
	}

	fn take(&mut self) -> Bytes {
		(&mut self.row_buf).drain(..)
			.collect()
	}

	fn next(&mut self) -> Option<Bytes> {
		while let Some(b) = (&mut self.chunk_buf).next() {
			(&mut self.row_buf).push(b);
			if b == crate::CSV_TERMINATOR {
				return Some(self.take());
			}
		}
		None
	}
}

impl TextRecordStream {
	fn into_split_text_record_stream(self) -> SplitTextRecordStream {
		SplitTextRecordStream {
			inner: self,
		}
	}
}

// We are using the 'RawStream' as a NewType to convert the reqwest error into an io error for this trait.
// In doing this, we become eligible for GzipDecoder.
//
// Inner is pinned, and we are pinning it again. This is due to auto-derives on the Stream trait.
// An alternative to the double pinning would be nice.
//
// https://docs.rs/async-compression/0.3.4/async_compression/stream/struct.GzipDecoder.html
impl Stream for RawStream {
	type Item = Result<Bytes, std::io::Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			// https://docs.rs/crate/reqwest/0.10.4/source/src/error.rs
			Some(Err(err)) => Poll::Ready(Some(Err(Error::reqwest_to_io(err)))),
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
		}
	}
}

// We have to use io::Error for the previous struct so that DecodedStream's inner gzip decoder
// will auto-derive Stream.
//
// This is the public version which correctly wraps the io::error into our own Error type.
impl Stream for PublRawStream {
	type Item = Result<Bytes, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(Error::decoder_error(err)))),
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
		}
	}
}

impl Stream for DecodedStream {
	type Item = Result<Bytes, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(Error::decoder_error(err)))),
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
		}
	}
}

impl Stream for ByteRecordStream {
	type Item = Result<Bytes, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match self.next() {
			Some(b) => Poll::Ready(Some(Ok(b))),
			None =>
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(err))),
			Some(Ok(chunk)) =>
		{
			self.chunk_buf = bytes::buf::IntoIter::new(chunk);
			match self.next() {
				Some(b) => Poll::Ready(Some(Ok(b))),
				None => Poll::Ready(None),
			}
		},},}
	}
}

// impl<T> Stream for DeserRecordStream<T> where
// 	T: crate::deser::Deser,
// 	T: std::marker::Unpin,
// {
// 	type Item = Result<T, std::io::Error>;
// 	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
// 		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
// 			None => Poll::Ready(None),
// 			Some(Err(err)) => Poll::Ready(Some(Err(err))),
// 			Some(Ok(byte_record)) => Poll::Ready(Some(Ok(crate::deser::Deser::deser(byte_record)))),
// 		}
// 	}
// }

impl Stream for TextRecordStream {
	type Item = Result<String, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(err))),
			Some(Ok(byte_record)) => 
		match String::from_utf8(byte_record.to_vec()) {
			Ok(text) => Poll::Ready(Some(Ok(text))),
			Err(err) => Poll::Ready(Some(Err(Error::utf8_error(err)))),
		},}
	}
}

// Splits the text records by the delimiter. Additionally, removes the termination character.
impl Stream for SplitTextRecordStream {
	type Item = Result<Vec<String>, Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(err))),
			Some(Ok(mut text_record)) =>
		{
			let _ = text_record.pop();
			Poll::Ready(Some(Ok(
				text_record.split(char::from(crate::CSV_DELIMITER))
					.map(|s| s.to_string())
					.collect()
			)))
		},}
	}
}