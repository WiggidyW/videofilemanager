#![allow(dead_code)]
use futures_util::{
	stream::StreamExt,
	task::{Poll, Context},
};
use futures_core::{self, stream::Stream};
use std::{io, pin::Pin};
use async_compression::stream::GzipDecoder;

pub enum Dataset {
	NameBasics,
	TitleAkas,
	TitleBasics,
	TitleCrew,
	TitleEpisode,
	TitlePrincipals,
	TitleRatings,
}

struct Response {
	inner: reqwest::Response,
	kind: Dataset,
}

// struct ResponseStream {
// 	inner: GzipDecoder<impl StreamExt<Item = Result<bytes::Bytes, std::io::Error>>>,
// 	kind: Dataset,
// }

struct Row {
	inner: bytes::Bytes
}

impl Dataset {
	async fn request(self) -> Result<Response, reqwest::Error> {
		Ok(Response {
			inner: reqwest::get(self.url())
				.await?
				.error_for_status()?,
			kind: self,
		})
	}

	#[cfg(not(test))]
	fn url(&self) -> &'static str {
		match self {
			Self::NameBasics => "https://datasets.imdbws.com/name.basics.tsv.gz",
			Self::TitleAkas => "https://datasets.imdbws.com/title.akas.tsv.gz",
			Self::TitleBasics => "https://datasets.imdbws.com/title.basics.tsv.gz",
			Self::TitleCrew => "https://datasets.imdbws.com/title.crew.tsv.gz",
			Self::TitleEpisode => "https://datasets.imdbws.com/title.episode.tsv.gz",
			Self::TitlePrincipals => "https://datasets.imdbws.com/title.principals.tsv.gz",
			Self::TitleRatings => "https://datasets.imdbws.com/title.ratings.tsv.gz",
		}
	}

	// https://stackoverflow.com/a/30527289
	#[cfg(test)]
	fn url(&self) -> &'static str {
		match self {
			Self::NameBasics => "http://localhost:1234/name.basics.tsv.gz",
			Self::TitleAkas => "http://localhost:1234/title.akas.tsv.gz",
			Self::TitleBasics => "http://localhost:1234/title.basics.tsv.gz",
			Self::TitleCrew => "http://localhost:1234/title.crew.tsv.gz",
			Self::TitleEpisode => "http://localhost:1234/title.episode.tsv.gz",
			Self::TitlePrincipals => "http://localhost:1234/title.principals.tsv.gz",
			Self::TitleRatings => "http://localhost:1234/title.ratings.tsv.gz",
		}
	}
}

// Hacks
//
// https://docs.rs/crate/reqwest/0.10.4/source/src/async_impl/decoder.rs

struct ImplStream {
	inner: reqwest::Response,
}

impl Stream for ImplStream {
	type Item = Result<bytes::Bytes, std::io::Error>;
	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
		match futures_core::ready!(Pin::new(&mut self.inner.bytes_stream()).poll_next(cx)) {
			Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk))),
			None => Poll::Ready(None),
			// https://docs.rs/crate/reqwest/0.10.4/source/src/error.rs
			Some(Err(err)) => Poll::Ready(Some(Err(io::Error::new(io::ErrorKind::Other, err)))),
		}
	}
}

#[cfg(test)]
mod tests {
	use tokio::stream::StreamExt;
	use std::io::Write;

	#[tokio::test]
	async fn my_test() {
		let b = crate::Dataset::TitleRatings
			.request()
			.await
			.unwrap();
		let b = reqwest::get(crate::Dataset::TitleRatings.url())
			.await
			.unwrap()
			.text()
			.await
			.unwrap();
		std::fs::File::create("test")
			.unwrap()
			.write(&b.as_bytes())
			.unwrap();
		// let b: bytes::Bytes = crate::Dataset::TitleRatings.request().await.unwrap().next().await.unwrap().unwrap();
		// println!("{:?}", b)
	}
}