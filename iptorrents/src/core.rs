use crate::error::Error;
use std::{ops::Deref, time::Duration};
use lazy_static::lazy_static;
use scraper::{Html, Selector};
use regex::Regex;

pub struct Requestor {
	inner: ureq::Agent,
	max_pages: u32,
	interval: Option<Duration>,
}

enum Response {
	Torrents(Html),
	TorrentFiles(Html),
}

struct TorrentElement {

}

struct TorrentFilesElement {

}

pub struct Torrent {
	name: String,
	torrent: String,
	id: u32,
	category: String,
	file_count: u32,
	snatches: u32,
	leechers: u32,
	seeders: u32,
	size: u64, // in bytes
	files: Vec<(String, u64)>, // (file name, byte size)
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

	fn get(&self, url: &str) -> Result<String, Error>	{
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
		match text.contains("/lout.php") {
			false => Err(Error::CookieError),
			true => Ok(text),
		}
	}
}

impl Deref for Response {
	type Target = Html;
	fn deref(&self) -> &Self::Target {
		match self {
			Self::Torrents(html) => &html,
			Self::TorrentFiles(html) => &html,
		}
	}
}

impl Response {

}

	// fn into_torrent_doc(&self) -> Self {
	// 	// https://github.com/Jackett/Jackett/blob/b98dbd70fa985255fa96b14d0718ea59f6d1b308/src/Jackett.Common/Indexers/IPTorrents.cs#L196
	// 	let select_table = Selector::parse(r#"table[id='torrents'] > tbody > tr"#)
	// 		.unwrap();
	// 	for row in self.inner
	// 		.select(select_table)
	// }

	// fn page_count(&self) -> u32 {
	// 	lazy_static! {
	// 		static ref RE1: Regex = Regex::new("<div class=\"single\"><a>Page <b>[0-9]+</b> of <b>[0-9]+</b></a></div>")
	// 			.unwrap();
	// 		static ref RE2: Regex = Regex::new("[0-9]+")
	// 			.unwrap();
	// 	}
	// 	let match_str = RE1.find(&self.inner)
	// 		.map(|m| m.as_str());
	// 	match match_str {
	// 		None => 1, // if the element is not present, then there's 1 page
	// 		Some(s) => RE2
	// 			.find_iter(s)
	// 			.nth(1)
	// 			.unwrap() // infallible, validated by RE1
	// 			.as_str()
	// 			.parse()
	// 			.unwrap(), // infallible, validated by RE2
	// 	}
	// }