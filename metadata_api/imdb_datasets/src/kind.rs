use reqwest::Url;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
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
    pub fn iter() -> impl Iterator<Item = Self> {
        vec![
            Dataset::NameBasics,
            Dataset::TitleAkas,
            Dataset::TitleBasics,
            Dataset::TitleCrew,
            Dataset::TitleEpisode,
            Dataset::TitlePrincipals,
            Dataset::TitleRatings,
        ].into_iter()
    }
}

impl From<Dataset> for reqwest::Url {
    fn from(value: Dataset) -> Self {
        match value {
            Dataset::NameBasics => Url::parse(
                "https://datasets.imdbws.com/name.basics.tsv.gz"
            ).unwrap(),
            Dataset::TitleAkas => Url::parse(
                "https://datasets.imdbws.com/title.akas.tsv.gz"
            ).unwrap(),
            Dataset::TitleBasics => Url::parse(
                "https://datasets.imdbws.com/title.basics.tsv.gz"
            ).unwrap(),
            Dataset::TitleCrew => Url::parse(
                "https://datasets.imdbws.com/title.crew.tsv.gz"
            ).unwrap(),
            Dataset::TitleEpisode => Url::parse(
                "https://datasets.imdbws.com/title.episode.tsv.gz"
            ).unwrap(),
            Dataset::TitlePrincipals => Url::parse(
                "https://datasets.imdbws.com/title.principals.tsv.gz"
            ).unwrap(),
            Dataset::TitleRatings => Url::parse(
                "https://datasets.imdbws.com/title.ratings.tsv.gz"
            ).unwrap(),
        }
    }
}