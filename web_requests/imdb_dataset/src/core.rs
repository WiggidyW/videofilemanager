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

const DELIMITER: u8 = b'\t';
const TERMINATOR: u8 = b'\n';

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
	buf: bytes::buf::IntoIter,
}

impl Response {
	fn into_raw_stream(self) -> RawStream {
		RawStream { inner:
			Box::pin(self.inner.bytes_stream())
		}
	}
}

impl RawStream {
	fn into_decoded_stream(self) -> DecodedStream {
		DecodedStream { inner:
			GzipDecoder::new(self)
		}
	}
}

// We are using the 'RawStream' as a NewType to convert the reqwest error into an io error for this trait.
// In doing this, we become eligible for GzipDecoder.
//
// Inner is pinned, and we are pinning it again. This is due to auto-derives.
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