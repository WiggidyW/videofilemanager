use serde::Deserialize;
use derive_more::From;

#[derive(Debug, From)]
pub enum ImdbDataset {
	TitleRatings(TitleRatings),
	TitleEpisode(TitleEpisode),
	TitleCrew(TitleCrew),
	TitleBasics(TitleBasics),
	TitleAkas(TitleAkas),
	NameBasics(NameBasics),
	TitlePrincipals(TitlePrincipals),
}

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