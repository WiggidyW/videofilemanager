use crate::error::Error;
use std::time::Duration;
use lazy_static::lazy_static;
use regex::Regex;

pub struct Requestor {
	inner: ureq::Agent,
	max_pages: u32,
	interval: Option<Duration>,
}

struct Response {
	inner: String,
}

struct TorrentElement {

}

struct TorrentFilesElement {

}

pub struct Torrent {
	Name: String,
	Torrent: String,
	Id: u32,
	Type: String,
	FileCount: u32,
	Snatches: u32,
	Leechers: u32,
	Seeders: u32,
	Size: u64, // in bytes
	Files: Vec<(String, u64)>, // (file name, byte size)
}

impl Requestor {
	pub fn new(cookie: &str, max_pages: u32, interval: Option<Duration>) -> Self {
		let agent = ureq::agent()
			.set("Cookie", cookie)
			.build();
		Self {
			inner: agent,
			max_pages: max_pages,
			interval: interval,
		}
	}

	fn get(&self, url: &str) -> Result<Response, Error>	{
		let res = self.inner
			.get(url)
			.call();
		match res.status() {
			i if i >= 200 && i <= 299 => (),
			i if i >= 300 && i <= 399 => return Err(Error::RedirectError(i)),
			i if i >= 400 && i <= 499 => return Err(Error::ClientError(i)),
			i if i >= 500 && i <= 599 => return Err(Error::ServerError(i)),
			_ => unreachable!(),
		}
		let text = res.into_string()
			.map_err(|e| Error::ParseError(e))?;
		Response::new(text)
	}
}

impl Response {
	fn new(html: String) -> Result<Self, Error> {
		match html.contains("/lout.php") {
			true => Ok(Self{ inner: html }),
			false => Err(Error::CookieError),
		}
	}

	fn page_count(&self) -> u32 {
		lazy_static! {
			static ref RE1: Regex = Regex::new("<div class=\"single\"><a>Page <b>[0-9]+</b> of <b>[0-9]+</b></a></div>")
				.unwrap();
			static ref RE2: Regex = Regex::new("[0-9]+")
				.unwrap();
		}
		let match_str = RE1.find(&self.inner)
			.map(|m| m.as_str());
		match match_str {
			None => 1, // if the element is not present, then there's 1 page
			Some(s) => RE2
				.find_iter(s)
				.nth(1)
				.unwrap() // infallible, validated by RE1
				.as_str()
				.parse()
				.unwrap(), // infallible, validated by RE2
		}
	}
}