use std::convert::TryFrom;
use serde::{Serialize, Deserialize};

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ImdbDataset<T> {
	timestamp: crate::DateTime,
	inner: T,
}

impl<T, U> TryFrom<crate::MetadataRes<U>> for ImdbDataset<T> where
	U: crate::MetadataRequest,
	T: TryFrom<U>,
{
	type Error = <T as TryFrom<U>>::Error;
	fn try_from(value: crate::MetadataRes<U>) -> Result<Self, Self::Error> {
		Ok(Self{
			timestamp: value.timestamp,
			inner: <T as TryFrom<U>>::try_from(value.data)?,
		})
	}
}

pub struct ImdbDatasetRes<
	const URL: &'static str,
> {
	inner: bytes::Bytes,
}

impl<const URL: &'static str> crate::MetadataRequest for ImdbDatasetRes<URL> {
	type Error = reqwest::Error;
	fn request() -> Result<Self, Self::Error> {
		Ok(Self{
			inner: reqwest::blocking::get(URL)?
				.error_for_status()?
				.bytes()?
		})
	}
}

impl<const URL: &'static str> ImdbDatasetRes<URL> {
	fn bytes(&self) -> &[u8] {
		<bytes::Bytes as AsRef<[u8]>>::as_ref(&self.inner)
	}
}

macro_rules! read_res_string_records {
	($res:ident) => {
		csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.from_reader(flate2::read::GzDecoder::new($res.bytes()))
			.records()
	}
}

const TITLE_EPISODE: &'static str = "https://datasets.imdbws.com/title.episode.tsv.gz";
pub type TitleEpisode = Vec<TitleEpisodeEntry>;

#[derive(Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct TitleEpisodeEntry {
	pub imdbid: String,
	pub seriesid: String,
	pub season: Option<u16>,
	pub episode: Option<u16>,
}

impl From<csv::StringRecord> for TitleEpisodeEntry {
	fn from(value: csv::StringRecord) -> Self {
		Self {
            imdbid: value[0].to_string(),
            seriesid: value[1].to_string(),
            season: {
                match value[2].parse::<u16>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
            }},
            episode: {
                match value[3].parse::<u16>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
            }},
        }
	}
}

impl TryFrom<ImdbDatasetRes<TITLE_EPISODE>> for TitleEpisode {
	type Error = csv::Error;
	fn try_from(value: ImdbDatasetRes<TITLE_EPISODE>) -> Result<Self, Self::Error> {
		let mut vec: Vec<TitleEpisodeEntry> = Vec::new();
		for row in read_res_string_records!(value) {
			vec.push(TitleEpisodeEntry::from(row?));
		}
		Ok(vec)
	}
}

impl crate::Metadata for ImdbDataset<TitleEpisode> {
	type Request = ImdbDatasetRes<TITLE_EPISODE>;
	type Data = TitleEpisode;
	fn timestamp(&self) -> &crate::DateTime {
		&self.timestamp
	}
	fn data(&self) -> &Self::Data {
		&self.inner
	}
}