pub enum Dataset {
	NameBasics,
	TitleAkas,
	TitleBasics,
	TitleCrew,
	TitleEpisode,
	TitlePrincipals,
	TitleRatings,
}

impl Dataset {
	// async fn request(&self) -> Result<Response, reqwest::Error> {
	// 	Ok(Response(
	// 		reqwest::get(self.url())
	// 			.await?
	// 			.error_for_status()?,
	// 	))
	// }

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

pub struct NameBasics {
	nameID: usize,
	name: String,
	birthYear: Option<usize>,
	deathYear: Option<usize>,
	primaryProfession: Option<Vec<String>>,
	imdbIDs: Option<Vec<usize>>,
}

pub struct TitleAkas {}
pub struct TitleBasics {}
pub struct TitleCrew {}
pub struct TitleEpisode {}
pub struct TitlePrincipals {}
pub struct TitleRatings {}