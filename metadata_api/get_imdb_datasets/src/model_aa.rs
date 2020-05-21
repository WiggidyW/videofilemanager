use futures_core::{
	stream::Stream,
	task::{Poll, Context},
};
use bytes::Bytes;
use std::{
	marker::{Unpin, PhantomData},
	pin::Pin,
	ops::{Deref, DerefMut},
};

pub async fn request<D: Dataset>() -> Result<Response<D>, reqwest::Error> {
	Ok(Response::new(
		reqwest::get(D::url())
			.await?
			.error_for_status()?
	))
}

pub trait Dataset {
	type Url: reqwest::IntoUrl;
	fn url() -> Self::Url;
}

pub struct Response<D> {
	inner: reqwest::Response,
	kind: PhantomData<D>,
}

#[derive(Deref, DerefMut)]
pub struct RawByteStream {
	#[deref]
	#[deref_mut]
	inner: Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>>>>,
}

impl<D> Response<D> {
	fn new(inner: reqwest::Response) -> Self {
		Self {
			inner: inner,
			kind: PhantomData::<D>::default(),
		}
	}

	// fn into_raw_byte_stream(self) -> RawByteStream<D> {
	// 	RawByteStream::new(self.inner.bytes_stream())
	// }
}

// impl<D> RawByteStream<D> {
// 	fn new<S>(inner: S) -> Self where
// 		S: Stream<Item = Result<Bytes, reqwest::Error>>,
// 		S: 'static,
// 	{
// 		Self {
// 			inner: Box::pin(inner),
// 			kind: PhantomData::<D>::default(),
// 		}
// 	}
// }

impl Stream for RawByteStream {
	type Item = Result<Bytes, std::io::Error>;
	fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Stream::poll_next(Pin::new(&mut self.inner), cx)) {
			None => Poll::Ready(None),
			Some(Err(e)) =>
				Poll::Ready(Some(Err(
					std::io::Error::new(crate::error::ERR_REQWEST_TO_IO, e)
				))),
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
		}
	}
}