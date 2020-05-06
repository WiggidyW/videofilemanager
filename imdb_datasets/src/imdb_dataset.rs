use serde::{Serialize, Deserialize, de::DeserializeOwned};
use std::convert::TryFrom;
use std::ops::Deref;

const TITLE_EPISODE: &'static str = "https://datasets.imdbws.com/title.episode.tsv.gz";
pub type TitleEpisode = ImdbDataset<Vec<TitleEpisodeEntry>, TITLE_EPISODE>;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ImdbDataset<T, const URL: &'static str> {
	inner: T,
}

impl<T, const URL: &'static str> Deref for ImdbDataset<T, URL> {
	type Target = T;
	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct ImdbDatasetResponse {
	inner: bytes::Bytes,
}

impl ImdbDatasetResponse {
	fn bytes(&self) -> &[u8] {
		<bytes::Bytes as AsRef<[u8]>>::as_ref(&self.inner)
	}
	fn new(url: &str) -> Result<Self, reqwest::Error> {
		Ok(Self{ inner: reqwest::blocking::get(url)?
			.error_for_status()?
			.bytes()?
		})
	}
	fn deserialize<T>(self) -> Result<Vec<T>, csv::Error> where
		T: DeserializeOwned,
	{
		csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.from_reader(flate2::read::GzDecoder::new(self.bytes()))
			.deserialize()
			.collect::<Result<Vec<T>, csv::Error>>()
	}
}

impl<T, const URL: &'static str> crate::Request for ImdbDataset<T, URL> {
	type Error = reqwest::Error;
	type Response = ImdbDatasetResponse;
	fn request() -> Result<Self::Response, Self::Error> {
		ImdbDatasetResponse::new(URL)
	}
}

impl<T, const URL: &'static str> TryFrom<ImdbDatasetResponse> for ImdbDataset<T, URL> where
	T: TryFrom<ImdbDatasetResponse>,
{
	type Error = <T as TryFrom<ImdbDatasetResponse>>::Error;
	fn try_from(value: ImdbDatasetResponse) -> Result<Self, Self::Error> {
		Ok(Self {
			inner: <T as TryFrom<ImdbDatasetResponse>>::try_from(value)?
		})
	}
}

impl<T> TryFrom<ImdbDatasetResponse> for Vec<T> where
	T: Row,
	T: Serialize,
	T: DeserializeOwned,
{
	type Error = csv::Error;
	fn try_from(value: ImdbDatasetResponse) -> Result<Self, Self::Error> {
		value.deserialize()
	}
}

pub trait Row {}

// https://docs.rs/csv/1.0.0-beta.5/csv/tutorial/index.html#handling-invalid-data-with-serde
#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TitleEpisodeEntry {
	#[serde(rename = "tconst")]
	pub imdbid: String,
	#[serde(rename = "parentTconst")]
	pub seriesid: String,
	#[serde(rename = "seasonNumber")]
	#[serde(deserialize_with = "csv::invalid_option")]
	pub season: Option<u32>,
	#[serde(rename = "episodeNumber")] 
	#[serde(deserialize_with = "csv::invalid_option")]
	pub episode: Option<u32>,
}
impl Row for TitleEpisodeEntry {}