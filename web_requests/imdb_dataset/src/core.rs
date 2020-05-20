use async_compression::stream::GzipDecoder;
use futures_core::{
	stream::Stream,
	task::{Poll, Context},
};
use std::{
	pin::Pin,
	io,
};
use bytes::Bytes;

const CSV_DELIMITER: u8 = b'\t';
const CSV_TERMINATOR: u8 = b'\n';

struct Response {
	inner: reqwest::Response,
}

struct RawStream {
	inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>>,
}

struct DecodedStream {
	inner: GzipDecoder<RawStream>,
}

struct CsvRowStream {
	inner: DecodedStream,
	row_buf: Vec<u8>,
	chunk_buf: bytes::buf::IntoIter<Bytes>,
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
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
			None => Poll::Ready(None),
			// https://docs.rs/crate/reqwest/0.10.4/source/src/error.rs
			Some(Err(err)) => Poll::Ready(Some(Err(io::Error::new(io::ErrorKind::Other, err)))),
		}
	}
}

impl DecodedStream {
	fn into_csv_row_stream(self) -> CsvRowStream {
		CsvRowStream::new(self)
	}
}

impl CsvRowStream {
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
			if b == CSV_TERMINATOR {
				return Some(self.take());
			}
		}
		None
	}
}

impl Stream for CsvRowStream {
	type Item = Result<Bytes, std::io::Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match self.next() {
			Some(b) => Poll::Ready(Some(Ok(b))),
			None =>
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut (&mut self.inner).inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(err)) => Poll::Ready(Some(Err(err))),
			Some(Ok(chunk)) => {
		self.chunk_buf = bytes::buf::IntoIter::new(chunk);
		match self.next() {
			Some(b) => Poll::Ready(Some(Ok(b))),
			None => Poll::Ready(None),
		}},},}
	}
}