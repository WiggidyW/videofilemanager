use crate::error;
use bytes::Bytes;
use std::borrow::Cow;
use error::ParamsError;

type Opt = Option::<std::convert::Infallible>;
type CowStr<'a> = Cow<'a, str>;

pub struct Params {
    pub apikey: String,
    pub imdbid: Option<u32>,
    pub title: Option<String>,
    pub kind: Option<String>,
    pub year: Option<u32>,
    pub season: Option<u32>,
    pub episode: Option<u32>,
}

impl Params {
    fn to_url(&self) -> Result<reqwest::Url, ParamsError> {
        if self.imdbid == None && self.title == None {
            return Err(
                ParamsError::new("imdbid / title", "both None", Opt::None)
            );
        }
        reqwest::Url::parse_with_params("http://www.omdbapi.com/", [
            self.apikey_tuple()?,
            self.imdbid_tuple()?,
            self.title_tuple()?,
            self.kind_tuple()?,
            self.year_tuple()?,
            self.season_tuple()?,
            self.episode_tuple()?,
        ]
            .iter()
            .filter_map(|o| o.clone())
        )
            .map_err(|e| ParamsError::new("n/a", "n/a", Some(e)))
    }
    fn apikey_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        Ok(Some(("apikey", Cow::<str>::from(&self.apikey))))
    }
    fn imdbid_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match self.imdbid {
            Some(i) if i < 100_000_000 => Ok(
                Some(("i", Cow::<str>::from(i.to_string())))
            ),
            Some(i) => Err(ParamsError::new("imdbid", i, Opt::None)),
            None => Ok(None),
        }
    }
    fn title_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match &self.title {
            Some(t) => Ok(Some(("t", CowStr::from(t)))),
            None => Ok(None),
        }
    }
    fn kind_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match &self.kind {
            Some(k) if {
                k == "movie" ||
                k == "episode" ||
                k == "series"
            } => Ok(Some(("type", CowStr::from(k)))),
            Some(k) => Err(ParamsError::new("kind", k, Opt::None)),
            None => Ok(None),
        }
    }
    fn year_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match self.year {
            Some(i) => Ok(Some(("year", CowStr::from(i.to_string())))),
            None => Ok(None),
        }
    }
    fn season_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match self.season {
            Some(i) => Ok(Some(("Season", CowStr::from(i.to_string())))),
            None => Ok(None),
        }
    }
    fn episode_tuple(&self) -> Result<
        Option<(&'static str, Cow<str>)>,
        ParamsError,
    > {
        match self.episode {
            Some(i) => Ok(Some(("Episode", CowStr::from(i.to_string())))),
            None => Ok(None),
        }
    }
}

impl crate::Params for Params {
    const KIND: &'static str = "omdb";
    fn parse(&self) -> Result<reqwest::Request, ParamsError> {
        self.to_url()
            .map(|url| reqwest::Request::new(reqwest::Method::GET, url))
    }
    fn validate(&self, _: Bytes) -> Result<(), error::ResponseError> {
        Ok(())
    }
}