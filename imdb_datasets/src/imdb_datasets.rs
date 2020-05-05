use chrono::{offset::Utc, DateTime};
use serde::{Serialize, Deserialize};
use flate2::read::GzDecoder;
use common::AsImdbid;
use bytes::Bytes;
use reqwest;
use bincode;
use csv;

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct EpisodeData {
    timestamp: DateTime<Utc>,
    inner: Vec<Episode>,
}

impl std::ops::Deref for EpisodeData {
    type Target = Vec<Episode>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for EpisodeData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl std::convert::TryFrom<EpisodeDataReq> for EpisodeData {
    type Error = csv::Error;
    fn try_from(value: EpisodeDataReq) -> Result<Self, Self::Error> {
        let mut vec: Vec<Episode> = Vec::new();
        for row in csv::ReaderBuilder::new()
            .delimiter(b'\t')
            .from_reader(GzDecoder::new(<Bytes as AsRef<[u8]>>::as_ref(&value.inner)))
            .records()
        {
            let row = row?;
            vec.push(Episode::from_row(&row[0], &row[1], &row[2], &row[3]));
        }
        Ok(Self {
            timestamp: Utc::now(),
            inner: vec,
        })
    }
}

impl EpisodeData {
    pub fn new() -> Result<Self, Error> {
        Ok(<Self as std::convert::TryFrom<EpisodeDataReq>>::try_from(
            EpisodeDataReq::new()?,
        )?)
    }

    pub fn deserialize_from<R: std::io::Read>(reader: R) -> bincode::Result<Self> {
    	bincode::deserialize_from(reader)
    }

    pub fn serialize_into<W: std::io::Write>(&self, writer: W) -> bincode::Result<()> {
    	bincode::serialize_into(writer, self)
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Episode {
	imdbid: String,
	seriesid: String,
	season: Option<u16>,
	episode: Option<u16>,
}

impl Episode {
    fn from_row(imdbid: &str, seriesid: &str, season: &str, episode: &str) -> Self {
        Self {
            imdbid: imdbid.to_string(),
            seriesid: seriesid.to_string(),
            season: {
                match season.parse::<u16>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            },
            episode: {
                match episode.parse::<u16>() {
                    Ok(i) => Some(i),
                    Err(_) => None,
                }
            },
        }
    }

    pub fn seriesid<'a>(&'a self) -> &'a str {
    	&self.seriesid
    }
}

impl AsImdbid for Episode {
    fn digits(&self, pad: usize) -> String {
        <String as AsImdbid>::digits(&self.imdbid, pad)
    }
}

#[derive(Debug)]
pub enum Error {
    RequestError(reqwest::Error),
    ReadError(csv::Error),
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<csv::Error> for Error {
    fn from(value: csv::Error) -> Self {
        Self::ReadError(value)
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct EpisodeDataReq {
    inner: Bytes,
}

impl EpisodeDataReq {
    fn new() -> Result<Self, reqwest::Error> {
        Ok(Self {
            inner: reqwest::blocking::get("https://datasets.imdbws.com/title.episode.tsv.gz")?
                .error_for_status()?
                .bytes()?,
        })
    }
}