use crate::error;
use bytes::Bytes;
use std::convert::TryFrom;
use error::ParamsError;

type Opt = Option::<std::convert::Infallible>;

pub struct Params {
    pub agent: String,
    pub episode: Option<u32>,
    pub imdbid: Option<u32>,
    pub moviebytesize: Option<u64>,
    pub query: Option<String>,
    pub season: Option<u32>,
    pub sublanguageid: Option<String>,
    pub tags: Option<String>,
}

impl Params {
    fn to_url(&self) -> Result<reqwest::Url, ParamsError> {
        let url = format!(
            "https://rest.opensubtitles.org/search/{}{}{}{}{}{}{}",
            self.episode_string()?,
            self.imdbid_string()?,
            self.moviebytesize_string()?,
            self.query_string()?,
            self.season_string()?,
            self.sublanguageid_string()?,
            self.tags_string()?,
        );
        if &url == "https://rest.opensubtitles.org/search/" {
            return Err(ParamsError::new("all", "None", Opt::None));
        }
        reqwest::Url::parse(&url)
            .map_err(|e| ParamsError::new("n/a", "n/a", Some(e)))
    }
    fn episode_string(&self) -> Result<String, ParamsError> {
        match self.episode {
            Some(i) => Ok(format!("episode-{}/", i)),
            None => Ok("".to_string()),
        }
    }
    fn imdbid_string(&self) -> Result<String, ParamsError> {
        match self.imdbid {
            Some(i) if i < 100_000_000 => Ok(format!("imdbid-{:0>7}/", i)),
            Some(i) => Err(ParamsError::new("imdbid", i, Opt::None)),
            None => Ok("".to_string()),
        }
    }
    fn moviebytesize_string(&self) -> Result<String, ParamsError> {
        match self.moviebytesize {
            Some(i) => Ok(format!("moviebytesize-{}/", i)),
            None => Ok("".to_string()),
        }
    }
    fn query_string(&self) -> Result<String, ParamsError> {
        match &self.query {
            Some(s) if s.find(' ') == None => Ok(format!("query-{}/", s)),
            Some(s) => Err(ParamsError::new("query", s, Opt::None)),
            None => Ok("".to_string()),
        }
    }
    fn season_string(&self) -> Result<String, ParamsError> {
        match self.season {
            Some(i) => Ok(format!("season-{}/", i)),
            None => Ok("".to_string()),
        }
    }
    fn sublanguageid_string(&self) -> Result<String, ParamsError> {
        match &self.sublanguageid {
            Some(s) if s.split(",").all(|s| s.len() == 3) => Ok(
                format!("sublanguageid-{}/", s)
            ),
            Some(s) => Err(ParamsError::new("sublanguageid", s, Opt::None)),
            None => Ok("".to_string()),
        }
    }
    fn tags_string(&self) -> Result<String, ParamsError> {
        match &self.tags {
            Some(s) if s.find(' ') == None => Ok(format!("tags-{}/", s)),
            Some(s) => Err(ParamsError::new("tags", s, Opt::None)),
            None => Ok("".to_string()),
        }
    }
}

impl crate::Params for Params {
    const KIND: &'static str = "opensubtitles";
    fn parse(&self) -> Result<reqwest::Request, ParamsError> {
        let mut request = reqwest::Request::new(
            reqwest::Method::GET,
            self.to_url()?
        );
        let agent = http::header::HeaderValue::try_from(&self.agent)
            .map_err(|e| ParamsError::new(
                "agent",
                self.agent.to_string(),
                Some(e),
            ))?;
        request.headers_mut()
            .insert("User-Agent", agent);
        Ok(request)
    }
    fn validate(&self, data: Bytes) -> Result<(), error::ResponseError> {
        if data.first() != Some(&b'[') {
            let s = String::from_utf8_lossy(&data);
            let kind = match s.find("agent") {
                Some(_) => error::ResponseErrorKind::Authentication,
                _ => error::ResponseErrorKind::Other,
            };
            Err(error::ResponseError::new(s, kind, Opt::None))
        }
        else {
            Ok(())
        }
    }
}