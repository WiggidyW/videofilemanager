pub struct ImdbDataset<T> {
	timestamp: crate::DateTime,
	inner: T,
}

macro_rules! imdb_dataset_res {
	($type:ty, $url:literal) => {
		impl crate::MetadataRequest for $type {
			type Error = reqwest::Error;
   			fn request() -> Result<Self, Self::Error> {
   				Ok(Self{
   					inner: reqwest::blocking::get($url)?
   						.error_for_status()?
   						.bytes()?
})}}}}

pub struct TitleEpisode {
	inner: Vec<TitleEpisodeEntry>,
}

pub struct TitleEpisodeRes {
	inner: bytes::Bytes,
}

imdb_dataset_res!(TitleEpisodeRes, "https://datasets.imdbws.com/title.episode.tsv.gz");

pub struct TitleEpisodeEntry {
	imdbid: String,
	seriesid: String,
	season: Option<u16>,
	episode: Option<u16>,
}