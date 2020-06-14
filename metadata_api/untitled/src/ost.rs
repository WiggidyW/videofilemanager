use crate::error;
use bytes::Bytes;
use serde_json::Value as Json;

pub struct Params<'a, Ag, Qu, Tg, Sl> {
    pub agent: Ag,
    pub episode: Option<u32>,
    pub imdbid: Option<u32>,
    pub moviebytesize: Option<u64>,
    pub query: Option<Qu>,
    pub season: Option<u32>,
    pub sublanguageid: Option<&'a [Sl]>,
    pub tags: Option<Tg>,
}

impl<Ag, Qu, Tg, Sl> Params<'_, Ag, Qu, Tg, Sl>
where
    Ag: AsRef<str>,
    Qu: AsRef<str>,
    Tg: AsRef<str>,
    Sl: AsRef<str>,
{
    fn to_url(&self) -> Result<reqwest::Url, error::ParamsError> {
        let url = format!("https://rest.opensubtitles.org/search/{}{}{}{}{}{}{}",
            self.episode.map(|i| format!("episode-{}/", i)).unwrap_or(""),
            "",
            "",
            "",
            "",
            "",
            "",
        );
        Ok(reqwest::Url::parse(&url).unwrap())
    }
    fn episode(&self)
}

impl<Ag, Qu, Tg, Sl> crate::Params for Params<'_, Ag, Qu, Tg, Sl>
where
    Ag: AsRef<str>,
    Qu: AsRef<str>,
    Tg: AsRef<str>,
    Sl: AsRef<str>,
{
    const KIND: &'static str = "opensubtitles";
    fn parse(&self) -> Result<reqwest::Request, error::ParamsError> {
        let url = format!("https://rest.opensubtitles.org/search/{}{}{}{}{}{}{}",

        );
        unimplemented!()
    }
    fn pre_validate(&self, data: Bytes) -> Result<(), error::ResponseError> {
        if data.first() != Some(&b'[') {
            let s = String::from_utf8_lossy(&data);
            Err(error::ResponseError::new(
                &s,
                match s.find("agent") {
                    Some(_) => error::ResponseErrorKind::Authentication,
                    _ => error::ResponseErrorKind::Other,
                },
                Option::<std::convert::Infallible>::None,
            ))
        }
        else {
            Ok(())
        }
    }
    fn post_validate(&self, _: &Json) -> Result<(), error::ResponseError> {
        Ok(())
    }
}