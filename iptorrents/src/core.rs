use std::{ops::Index, error::Error as StdError, convert::TryFrom, cmp::min, iter::Sum, io::Read};
use scraper::{Html, Selector, element_ref::ElementRef};
use lazy_static::lazy_static;
use crate::Error;
use crate::request::{SearchRequest, TorrentRequest, FileInfoRequest};
use crate::response::{SearchResponse, TorrentResponse, FileInfoResponse};

pub trait Requestor {
	type Error: StdError + 'static;
	type Reader: Read;
	fn request(&self, url: &str, cookie: &str) -> Result<Self::Reader, Self::Error>;
}

#[derive(Debug)]
pub struct Operator<R> {
	req: R,
}

impl<R> Operator<R>
where
	R: Requestor + Sized,
{
	pub fn new(req: R) -> Self {
		Self {
			req: req,
		}
	}

	pub fn get_search(
		&self,
		req: SearchRequest,
		cookie: &str,
		size_hint: Option<usize>,
		) -> Result<SearchResponse, Error>
	{
		let mut buf: String = String::new();
		self.req
			.request(&req.url(), cookie)
			.map_err(|e| Error::RequestError(Box::new(e)))?
			.read_to_string(&mut buf)
			.map_err(|e| Error::FileStreamError(e))?;
		Ok(SearchResponse::new(buf)?)
	}


	pub fn get_file_info(
		&self,
		req: FileInfoRequest,
		cookie: &str,
		size_hint: Option<usize>,
		) -> Result<FileInfoResponse, Error>
	{
		let mut buf: String = String::new();
		self.req
			.request(&req.url(), cookie)
			.map_err(|e| Error::RequestError(Box::new(e)))?
			.read_to_string(&mut buf)
			.map_err(|e| Error::FileStreamError(e))?;
		Ok(FileInfoResponse::new(buf)?)
	}

	pub fn get_torrent(
		&self,
		req: TorrentRequest,
		cookie: &str,
		) -> Result<TorrentResponse, Error>
	{
		let mut buf: Vec<u8> = Vec::new();
		self.req
			.request(&req.url(), cookie)
			.map_err(|e| Error::RequestError(Box::new(e)))?
			.read_to_end(&mut buf)
			.map_err(|e| Error::FileStreamError(e))?;
		Ok(TorrentResponse::new(buf))
	}
}

// pub struct Torrent {
// 	pub uploader: Option<String>,
// 	pub torrent_title: String,
// 	pub comment_count: u32,
// 	pub free_leech: bool,
// 	pub label_id: u32,
// 	pub title: String,
// 	pub snatches: u32,
// 	pub leechers: u32,
// 	pub seeders: u32,
// 	pub age: String,
// 	pub id: u32,
// 	pub file_count: u32,
// 	pub size: u64,
// 	pub files: Vec<(String, u64)>,
// }

// struct Response {
// 	inner: Html,
// }

// impl<R, C, F> Operator<R, C, F>
// where
// 	R: Requestor + Sized,
// 	C: Cache + Sized,
// 	F: Fn(),
// {
// 	pub fn new(
// 		req: R,
// 		cache: C,
// 		cookie: String,
// 		max_pages: usize,
// 		max_torrents: usize,
// 		sleep: F) -> Self
// 	{
// 		Self {
// 			req: req,
// 			cache: cache,
// 			cookie: cookie,
// 			max_pages: max_pages,
// 			max_torrents: max_torrents,
// 			sleep: sleep,
// 		}
// 	}

// 	pub fn get_torrent(&self, req: &Torrent) -> Result<Vec<u8>, Error> {
// 		self.request_bytes(&req.url())
// 	}

// 	pub fn get_results(&self, req: Request) -> Result<Vec<Torrent>, Error> {
// 		let mut res = self.request(&req.url(1))?;
// 		let final_page = min(res.num_pages().unwrap_or(1), self.max_pages);
// 		// Get the torrents
// 		let mut torrents: Vec<TorrentInfo> = Vec::with_capacity(self.max_torrents);
// 		for i in 1..=final_page {
// 			if i != 1 {
// 				res = self.request(&req.url(i))?;
// 			}
// 			for torrent in res.torrents() {
// 				torrents.push(torrent?);
// 				if torrents.len() >= (self.max_torrents) {
// 					break;
// 				}
// 			}
// 		}
// 		// Get the torrent files
// 		let mut torrent_files: Vec<Vec<TorrentFile>> = Vec::with_capacity(torrents.len());
// 		for torrent in &torrents {
// 			match self.cache.get(torrent.id) {
// 				Ok(None) => (),
// 				Err(e) => println!("{:?}", e),
// 				Ok(Some(b)) => 
// 			match deserialize(b) {
// 				Err(e) => println!("{:?}", e),
// 				Ok(vec) => {
// 					torrent_files.push(vec);
// 					continue;
// 				},
// 			}}
// 			let mut vec: Vec<TorrentFile> = Vec::with_capacity(torrent.file_count);
// 			let res = self.request(&torrent.files_url())?;
// 			for file in res.torrent_files() {
// 				vec.push(file?);
// 			}
// 			match serialize(&vec) {
// 				Err(e) => println!("{:?}", e),
// 				Ok(b) =>
// 			match self.cache.set(torrent.id, &b) {
// 				Err(e) => println!("{:?}", e),
// 				Ok(()) => (),
// 			}}
// 			torrent_files.push(vec);
// 		}
// 		Ok(torrents.into_iter()
// 			.zip(torrent_files.into_iter())
// 			.map(|t| Torrent::new(t.0, t.1))
// 			.collect()
// 		)
// 	}

// 	fn request(&self, url: &str) -> Result<Response, Error> {
// 		let res = self.req
// 			.request(url, &self.cookie);
// 		(self.sleep)();
// 		match res {
// 			Err(e) => {
// 				let e = Box::new(e);
// 				Err(Error::RequestError(e))
// 			},
// 			Ok(res) if !res.contains("/lout.php") => {
// 				let cookie = (&self.cookie)
// 					.to_string();
// 				Err(Error::CookieError(cookie))
// 			},
// 			Ok(res) => {
// 				let res = Html::parse_document(&res);
// 				Ok(Response::new(res))
// 			}
// 		}
// 	}

// 	fn request_bytes(&self, url: &str) -> Result<Vec<u8>, Error> {
// 		self.req
// 			.request_bytes(url, &self.cookie)
// 			.map_err(|e| Error::RequestError(Box::new(e)))
// 	}
// }

// impl Torrent {
// 	fn new(info: TorrentInfo, files: Vec<TorrentFile>) -> Self {
// 		Self {
// 			uploader: info.uploader,
// 			torrent_title: info.torrent_title,
// 			comment_count: info.comment_count,
// 			free_leech: info.free_leech,
// 			label_id: info.label_id,
// 			title: info.title,
// 			snatches: info.snatches,
// 			leechers: info.leechers,
// 			seeders: info.seeders,
// 			age: info.age,
// 			id: info.id,
// 			file_count: files.len() as u32,
// 			size: u64::sum(files
// 				.iter()
// 				.map(|tf| tf.size)),
// 			files: files
// 				.into_iter()
// 				.map(|tf| (tf.title, tf.size))
// 				.collect(),
// 		}
// 	}

// 	fn url(&self) -> String {
// 		format!("https://www.iptorrents.com/download.php/{}/{}", self.id, self.torrent_title)
// 	}
// }

// impl Response {
// 	fn new(html: Html) -> Self {
// 		Self {
// 			inner: html,
// 		}
// 	}

// 	fn torrents<'a>(&'a self) -> impl Iterator<Item = Result<TorrentInfo, Error>> + 'a {
// 		lazy_static! {
// 			static ref SEL: Selector = Selector::parse(
// 				r#"table[id='torrents'] > tbody > tr"#)
// 				.unwrap();
// 		}
// 		self.inner
// 			.select(&SEL)
// 			.skip(1)
// 			.map(|h| TorrentInfo::try_from(h))
// 	}

// 	fn torrent_files<'a>(&'a self) -> impl Iterator<Item = Result<TorrentFile, Error>> + 'a {
// 		lazy_static! {
// 			static ref SEL: Selector = Selector::parse(
// 				r#"table[id='body'] > tbody > tr > td > table[class='t1'] tr"#)
// 				.unwrap();
// 		}
// 		self.inner
// 			.select(&SEL)
// 			.skip(1)
// 			.map(|h| TorrentFile::try_from(h))
// 	}

// 	fn num_pages(&self) -> Option<usize> {
// 		lazy_static! {
// 			static ref SEL: Selector = Selector::parse(
// 				r#"div[class="single"] > a > b ~ b"#)
// 				.unwrap();
// 		}
// 		self.inner
// 			.select(&SEL)
// 			.filter_map(|s| s
// 				.inner_html()
// 				.parse()
// 				.ok()
// 			)
// 			.next()
// 	}
// }

// impl TorrentInfo {
// 	fn files_url(&self) -> String {
// 		format!("https://www.iptorrents.com/t/{}/files", self.id)
// 	}
// }
