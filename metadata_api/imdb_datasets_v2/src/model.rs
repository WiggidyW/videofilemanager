use serde::{Deserialize, de::DeserializeOwned};

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleRatings {
	tconst: String,
	averageRating: String,
	numVotes: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleEpisode {
	tconst: String,
	parentTconst: String,
	seasonNumber: String,
	episodeNumber: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleCrew {
	tconst: String,
	directors: String,
	writers: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleBasics {
	tconst: String,
	titleType: String,
	primaryTitle: String,
	originalTitle: String,
	isAdult: String,
	startYear: String,
	endYear: String,
	runtimeMinutes: String,
	genres: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitleAkas {
	titleId: String,
	ordering: String,
	title: String,
	region: String,
	language: String,
	types: String,
	attributes: String,
	isOriginalTitle: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct NameBasics {
	nconst: String,
	primaryName: String,
	birthYear: String,
	deathYear: String,
	primaryProfession: String,
	knownForTitles: String,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct TitlePrincipals {
	tconst: String,
	ordering: String,
	nconst: String,
	category: String,
	job: String,
	characters: String,
}

pub trait ImdbDataset: Sized {
	fn url() -> &'static str;
	fn request() -> Result<Response<Self>, reqwest::Error> {
		let res = reqwest::blocking::get(Self::url())?
			.error_for_status()?;
		Ok(Response {
			inner: res,
			kind: std::marker::PhantomData::default(),
		})
	}
}

macro_rules! url {
	($t:ty, $url: literal) => {
		impl ImdbDataset for $t {
			fn url() -> &'static str {
				$url
			}
		}
	}
}

pub struct Response<T> {
	inner: reqwest::blocking::Response,
	kind: std::marker::PhantomData<T>,
}

impl<T> Response<T>
where
	T: ImdbDataset,
	T: DeserializeOwned,
{
	pub fn into_iter(self) -> impl Iterator<Item = Result<T, csv::Error>> {
		let decoder = flate2::read::GzDecoder::new(self.inner);
		csv::ReaderBuilder::new()
			.delimiter(b'\t')
			.quoting(false)
			.from_reader(decoder)
			.into_deserialize()
	}
}

#[cfg(not(test))]
const _: () = {
	url!(TitleRatings, "https://datasets.imdbws.com/title.ratings.tsv.gz");
	url!(TitleEpisode, "https://datasets.imdbws.com/title.episode.tsv.gz");
	url!(TitleCrew, "https://datasets.imdbws.com/title.crew.tsv.gz");
	url!(TitleBasics, "https://datasets.imdbws.com/title.basics.tsv.gz");
	url!(TitleAkas, "https://datasets.imdbws.com/title.akas.tsv.gz");
	url!(NameBasics, "https://datasets.imdbws.com/name.basics.tsv.gz");
	url!(TitlePrincipals, "https://datasets.imdbws.com/title.principals.tsv.gz");
};

#[cfg(test)]
const _: () = {
	url!(TitleRatings, "http://localhost:12794/imdb-datasets/title.ratings.tsv.gz");
	url!(TitleEpisode, "http://localhost:12794/imdb-datasets/title.episode.tsv.gz");
	url!(TitleCrew, "http://localhost:12794/imdb-datasets/title.crew.tsv.gz");
	url!(TitleBasics, "http://localhost:12794/imdb-datasets/title.basics.tsv.gz");
	url!(TitleAkas, "http://localhost:12794/imdb-datasets/title.akas.tsv.gz");
	url!(NameBasics, "http://localhost:12794/imdb-datasets/name.basics.tsv.gz");
	url!(TitlePrincipals, "http://localhost:12794/imdb-datasets/title.principals.tsv.gz");
};