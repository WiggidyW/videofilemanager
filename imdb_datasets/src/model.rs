use chrono::{DateTime, offset::Utc};
use std::collections::HashMap;
use flate2::read::GzDecoder;
use bytes::Bytes;
use reqwest;
use common;
use csv;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct TitleEpisodeData {
	timestamp: DateTime<Utc>,
	inner: HashMap<Parent, Vec<Child>>,
}

impl TitleEpisodeData {
	fn new(timestamp: DateTime<Utc>, hashmap: HashMap<Parent, Vec<Child>>) -> Self {
		Self {
			timestamp: timestamp,
			inner: hashmap,
		}
	}
}

impl std::convert::TryFrom<TitleEpisodeDataRequest> for TitleEpisodeData {
	type Error = csv::Error;
	fn try_from(value: TitleEpisodeDataRequest) -> Result<Self, Self::Error> {
		let mut hashmap: HashMap<Parent, Vec<Child>> = HashMap::new();
		for row in csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.from_reader(GzDecoder::new(<Bytes as AsRef<[u8]>>::as_ref(&value.inner)))
			.records()
		{
			let row = row?;
			let parent = Parent::new(row[1]);
			let child = Child::new(row[0], row[2], row[3]);
			match hashmap.get_mut(&parent) {
				Some(v) => v.push(child),
				None => {
					hashmap.insert(parent, vec![child]);
					()
		},};}
		Ok(Self::new(Utc::now(), hashmap))
	}
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Parent {
	imdbid: String,
}

impl Parent {
	fn new(imdbid: String) -> Self {
		Self { imdbid: imdbid }
	}
}

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Child {
	imdbid: String,
	season: Option<u16>,
	episode: Option<u16>,
}

impl Child {
	fn new(imdbid: String, season: String, episode:String) -> Self {
		Self {
			imdbid: imdbid,
			season:
				if season[0].is_numeric()
					{ Some(season.parse::<u16>().unwrap()) }
				else
					{ None },
			episode:
				if episode[0].is_numeric()
					{ Some(episode.parse::<u16>().unwrap()) }
				else
					{ None },
		}
	}
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct TitleEpisodeDataRequest {
	inner: Bytes,
}

impl TitleEpisodeDataRequest {
	fn new() -> Result<Self, reqwest::Error> {
		Ok(Self{ inner: reqwest::blocking::get
			("https://datasets.imdbws.com/title.episode.tsv.gz")?
			.error_for_status()?
			.bytes()? })
	}
}

impl std::ops::Deref for TitleEpisodeData {
	type Target = HashMap<Parent, Vec<Child>>;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl std::ops::DerefMut for TitleEpisodeData {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.inner
	}
}

// impl common::Imdbid for Parent {
// 	type Error = std::convert::Infallible;
// 	fn is_valid(&self) -> Result<(), Self::Error> {
// 		Ok(())
// 	}
// 	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
// 		Ok(<String as Imdbid>::to_digits(&self.imdbid, pad).unwrap())
// 	}
// }

// impl common::Imdbid for Child {
// 	type Error = std::convert::Infallible;
// 	fn is_valid(&self) -> Result<(), Self::Error> {
// 		Ok(())
// 	}
// 	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
// 		Ok(<String as Imdbid>::to_digits(&self.imdbid, pad).unwrap())
// 	}
// }
