use serde::de::DeserializeOwned;
use crate::error::Error;

pub struct Response<T> {
	inner: reqwest::blocking::Response,
	kind: std::marker::PhantomData<T>,
}

pub trait ImdbDataset: Sized {
	fn url() -> &'static str;
	fn into(self) -> crate::model::ImdbDataset;
	fn request() -> Result<Response<Self>, Error> {
		let res = reqwest::blocking::get(Self::url())?
			.error_for_status()?;
		Ok(Response {
			inner: res,
			kind: std::marker::PhantomData::default(),
		})
	}
}

impl<T> Response<T>
where
	T: ImdbDataset,
	T: DeserializeOwned,
{
	pub fn into_iter(self) -> impl Iterator<Item = Result<crate::model::ImdbDataset, Error>> {
		let decoder = flate2::read::GzDecoder::new(self.inner);
		csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.quoting(false)
			.from_reader(decoder)
			.into_deserialize()
			.map(|d_res: Result<T, csv::Error>| d_res
				.map(|d| d.into())
				.map_err(|e| Error::from(e))
			)
	}
}

macro_rules! imdb_dataset {
	($t:ty, $url: literal) => {
		impl ImdbDataset for $t {
			fn url() -> &'static str {
				$url
			}
			fn into(self) -> crate::model::ImdbDataset {
				crate::model::ImdbDataset::from(self)
			}
		}
	}
}

#[cfg(not(test))]
const _: () = {
	imdb_dataset!(crate::model::TitleRatings, "https://datasets.imdbws.com/title.ratings.tsv.gz");
	imdb_dataset!(crate::model::TitleEpisode, "https://datasets.imdbws.com/title.episode.tsv.gz");
	imdb_dataset!(crate::model::TitleCrew, "https://datasets.imdbws.com/title.crew.tsv.gz");
	imdb_dataset!(crate::model::TitleBasics, "https://datasets.imdbws.com/title.basics.tsv.gz");
	imdb_dataset!(crate::model::TitleAkas, "https://datasets.imdbws.com/title.akas.tsv.gz");
	imdb_dataset!(crate::model::NameBasics, "https://datasets.imdbws.com/name.basics.tsv.gz");
	imdb_dataset!(crate::model::TitlePrincipals, "https://datasets.imdbws.com/title.principals.tsv.gz");
};

#[cfg(test)]
const _: () = {
	imdb_dataset!(crate::model::TitleRatings, "http://localhost:12794/imdb-datasets/title.ratings.tsv.gz");
	imdb_dataset!(crate::model::TitleEpisode, "http://localhost:12794/imdb-datasets/title.episode.tsv.gz");
	imdb_dataset!(crate::model::TitleCrew, "http://localhost:12794/imdb-datasets/title.crew.tsv.gz");
	imdb_dataset!(crate::model::TitleBasics, "http://localhost:12794/imdb-datasets/title.basics.tsv.gz");
	imdb_dataset!(crate::model::TitleAkas, "http://localhost:12794/imdb-datasets/title.akas.tsv.gz");
	imdb_dataset!(crate::model::NameBasics, "http://localhost:12794/imdb-datasets/name.basics.tsv.gz");
	imdb_dataset!(crate::model::TitlePrincipals, "http://localhost:12794/imdb-datasets/title.principals.tsv.gz");
};