use std::{ops::Index, error::Error as StdError, convert::TryFrom, cmp::min, iter::Sum};
use scraper::{Html, Selector, element_ref::ElementRef};
use bincode::{serialize, deserialize};
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use crate::{Error, HtmlError, request::Request};

pub trait Requestor {
	type Error: StdError + 'static;
	fn request(&self, url: &str, cookie: &str) -> Result<String, Self::Error>;
}

pub trait Cache {
	type Error: StdError + 'static;
	fn get(&self, k: u32) -> Result<Option<&[u8]>, Error>;
	fn set(&self, k: u32, v: &[u8]) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct Operator<R, C, F> {
	req: R,
	cache: C,
	cookie: String,
	max_pages: usize,
	max_torrents: usize,
	sleep: F,
}

pub struct Torrent {
	pub uploader: Option<String>,
	pub torrent_title: String,
	pub comment_count: u32,
	pub free_leech: bool,
	pub label_id: u32,
	pub title: String,
	pub snatches: u32,
	pub leechers: u32,
	pub seeders: u32,
	pub age: String,
	pub id: u32,
	pub file_count: u32,
	pub size: u64,
	pub files: Vec<(String, u64)>,
}

struct Response {
	inner: Html,
}

#[derive(Debug)]
struct TorrentInfo {
	label_id: u32,
	age: String,
	uploader: Option<String>,
	free_leech: bool,
	title: String,
	id: u32,
	torrent_title: String,
	comment_count: u32,
	file_count: usize,
	snatches: u32,
	seeders: u32,
	leechers: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TorrentFile {
	title: String,
	size: u64,
}

impl<R, C, F> Operator<R, C, F>
where
	R: Requestor + Sized,
	C: Cache + Sized,
	F: Fn(),
{
	pub fn new(
		req: R,
		cache: C,
		cookie: String,
		max_pages: usize,
		max_torrents: usize,
		sleep: F) -> Self
	{
		Self {
			req: req,
			cache: cache,
			cookie: cookie,
			max_pages: max_pages,
			max_torrents: max_torrents,
			sleep: sleep,
		}
	}

	pub fn get_results(&self, req: Request) -> Result<Vec<Torrent>, Error> {
		let mut res = self.request(&req.url(1))?;
		let final_page = min(res.num_pages().unwrap_or(1), self.max_pages);
		// Get the torrents
		let mut torrents: Vec<TorrentInfo> = Vec::with_capacity(self.max_torrents);
		for i in 1..=final_page {
			if i != 1 {
				res = self.request(&req.url(i))?;
			}
			for torrent in res.torrents() {
				torrents.push(torrent?);
				if torrents.len() >= (self.max_torrents) {
					break;
				}
			}
		}
		// Get the torrent files
		let mut torrent_files: Vec<Vec<TorrentFile>> = Vec::with_capacity(torrents.len());
		for torrent in &torrents {
			match self.cache.get(torrent.id) {
				Ok(None) => (),
				Err(e) => println!("{:?}", e),
				Ok(Some(b)) => 
			match deserialize(b) {
				Err(e) => println!("{:?}", e),
				Ok(vec) => {
					torrent_files.push(vec);
					continue;
				},
			}}
			let mut vec: Vec<TorrentFile> = Vec::with_capacity(torrent.file_count);
			let res = self.request(&torrent.files_url())?;
			for file in res.torrent_files() {
				vec.push(file?);
			}
			match serialize(&vec) {
				Err(e) => println!("{:?}", e),
				Ok(b) =>
			match self.cache.set(torrent.id, &b) {
				Err(e) => println!("{:?}", e),
				Ok(()) => (),
			}}
			torrent_files.push(vec);
		}
		Ok(torrents.into_iter()
			.zip(torrent_files.into_iter())
			.map(|t| Torrent::new(t.0, t.1))
			.collect()
		)
	}

	fn request(&self, url: &str) -> Result<Response, Error> {
		let res = self.req
			.request(url, &self.cookie);
		(self.sleep)();
		match res {
			Err(e) => {
				let e = Box::new(e);
				Err(Error::RequestError(e))
			},
			Ok(res) if !res.contains("/lout.php") => {
				let cookie = (&self.cookie)
					.to_string();
				Err(Error::CookieError(cookie))
			},
			Ok(res) => {
				let res = Html::parse_document(&res);
				Ok(Response::new(res))
			}
		}
	}
}

impl Torrent {
	fn new(info: TorrentInfo, files: Vec<TorrentFile>) -> Self {
		Self {
			uploader: info.uploader,
			torrent_title: info.torrent_title,
			comment_count: info.comment_count,
			free_leech: info.free_leech,
			label_id: info.label_id,
			title: info.title,
			snatches: info.snatches,
			leechers: info.leechers,
			seeders: info.seeders,
			age: info.age,
			id: info.id,
			file_count: files.len() as u32,
			size: u64::sum(files
				.iter()
				.map(|tf| tf.size)),
			files: files
				.into_iter()
				.map(|tf| (tf.title, tf.size))
				.collect(),
		}
	}
}

impl Response {
	fn new(html: Html) -> Self {
		Self {
			inner: html,
		}
	}

	fn torrents<'a>(&'a self) -> impl Iterator<Item = Result<TorrentInfo, Error>> + 'a {
		lazy_static! {
			static ref SEL: Selector = Selector::parse(
				r#"table[id='torrents'] > tbody > tr"#)
				.unwrap();
		}
		self.inner
			.select(&SEL)
			.skip(1)
			.map(|h| TorrentInfo::try_from(h))
	}

	fn torrent_files<'a>(&'a self) -> impl Iterator<Item = Result<TorrentFile, Error>> + 'a {
		lazy_static! {
			static ref SEL: Selector = Selector::parse(
				r#"table[id='body'] > tbody > tr > td > table[class='t1'] tr"#)
				.unwrap();
		}
		self.inner
			.select(&SEL)
			.skip(1)
			.map(|h| TorrentFile::try_from(h))
	}

	fn num_pages(&self) -> Option<usize> {
		lazy_static! {
			static ref SEL: Selector = Selector::parse(
				r#"div[class="single"] > a > b ~ b"#)
				.unwrap();
		}
		self.inner
			.select(&SEL)
			.filter_map(|s| s
				.inner_html()
				.parse()
				.ok()
			)
			.next()
	}
}

impl TorrentInfo {
	fn files_url(&self) -> String {
		unimplemented!()
	}
}

impl TryFrom<ElementRef<'_>> for TorrentInfo {
	type Error = Error;
	fn try_from(value: ElementRef) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref SEL_TD: Selector = Selector::parse("td")
				.unwrap();
			static ref SEL_A: Selector = Selector::parse("a")
				.unwrap();
			static ref SEL_SPAN: Selector = Selector::parse("span")
				.unwrap();
			static ref SEL_DIV: Selector = Selector::parse("div")
				.unwrap();
		}
		let rows: Vec<ElementRef> = value
			.select(&SEL_TD)
			.collect();
		if rows.len() != 10 {
			Err(HtmlError::InvalidLineCount(value.html()))?;
		}
			
		let label_id = {
			rows[0].select(&SEL_A)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 1))?
				.value()
				.attr("href")
				.ok_or(HtmlError::AttributeNotFound(value.html(), "attribute 'href' of element 'a'", 1))?
				.index(1..)
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of attribute 'href' of element 'a'", 1))?
		};
		let (age, uploader) = {
			let split = rows[1]
				.select(&SEL_DIV)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 2))?
				.inner_html();
			let mut split = split
				.split(" | ")
				.nth(1)
				.ok_or(HtmlError::InvalidValue(value.html(), "2nd 'text' of split(' | ') of 'value' of element 'a'", 2))?
				.split(" by ");
			let age = split
				.next()
				// the following error should be practically impossible
				.ok_or(HtmlError::InvalidValue(value.html(), "2nd 'text' of split(' | ') of 'value' of element 'a'", 2))?
				.to_string();
			let uploader = split
				.next()
				.map(|s| s.to_string());
			(age, uploader)
		};
		let free_leech = {
			rows[1].select(&SEL_SPAN)
				.next()
				.is_some()
		};
		let title = {
			rows[1].select(&SEL_A)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 2))?
				.inner_html()
		};
		let (id, torrent_title) = {
			let mut split = rows[3]
				.select(&SEL_A)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 4))?
				.value()
				.attr("href")
				.ok_or(HtmlError::AttributeNotFound(value.html(), "attribute 'href' of element 'a'", 4))?
				.split('/');
			let id = split
				.nth(2)
				.ok_or(HtmlError::InvalidValue(value.html(), "3rd 'text' of split('/') of 'value' of attribute 'href' of element 'a'", 4))?
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "3rd 'num' of split('/') of 'value' of attribute 'href' of element 'a'", 4))?;
			let torrent_title = split
				.next()
				.ok_or(HtmlError::InvalidValue(value.html(), "4th 'text' of split('/') of 'value' of attribute 'href' of element 'a'", 4))?
				.to_string();
			(id, torrent_title)
		};
		let comment_count = {
			rows[4].select(&SEL_A)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 5))?
				.inner_html()
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'a'", 5))?
		};
		let file_count = {
			rows[6].select(&SEL_A)
				.next()
				.ok_or(HtmlError::AttributeNotFound(value.html(), "element 'a'", 7))?
				.inner_html()
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'a'", 7))?
		};
		let snatches = {
			rows[7].inner_html()
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'td'", 8))?
		};
		let seeders = {
			rows[8].inner_html()
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'td'", 9))?
		};
		let leechers = {
			rows[9].inner_html()
				.parse()
				.map_err(|_| HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'td'", 10))?
		};

		Ok(Self {
			label_id: label_id,
			age: age,
			uploader: uploader,
			free_leech: free_leech,
			title: title,
			id: id,
			torrent_title: torrent_title,
			comment_count: comment_count,
			file_count: file_count,
			snatches: snatches,
			seeders: seeders,
			leechers: leechers,
		})
	}
}
impl TryFrom<ElementRef<'_>> for TorrentFile {
	type Error = Error;
	fn try_from(value: ElementRef) -> Result<Self, Self::Error> {
		lazy_static! {
			static ref SEL_TD: Selector = Selector::parse("td")
				.unwrap();
		}
		let rows: Vec<ElementRef> = value
			.select(&SEL_TD)
			.collect();
		if rows.len() != 2 {
			Err(HtmlError::InvalidLineCount(value.html()))?;
		}
		let title = {
			rows[0].inner_html()
		};
		let size = {
			let s = rows[1].inner_html();
			str_to_byte_count(&s)
				.ok_or(HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'td'", 2))?
		};
		Ok(Self {
			title: title,
			size: size,
		})
	}
}

fn str_to_byte_count(string: &str) -> Option<u64> {
	if !string.is_empty() {
		if let Some(s) = string
			.get((string.len() - 2)..string.len())
		{
			let mult: f64;
			match s {
				" B" => mult = 1.0,
				"KB" => mult = 1_024.0,
				"MB" => mult = 1_048_576.0,
				"GB" => mult = 1_073_741_824.0,
				"TB" => mult = 1_099_511_627_776.0,
				_ => return None,
			}
			if let Ok(f) = string
				.trim_end_matches(|c| !char::is_numeric(c))
				.parse::<f64>()
			{
				return Some((f * mult) as u64);
			}
		}
	}
	None
}