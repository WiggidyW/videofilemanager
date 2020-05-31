use std::{ops::Index, convert::TryFrom};
use scraper::{Selector, element_ref::ElementRef};
use lazy_static::lazy_static;
use crate::{Error, HtmlError};

#[derive(Debug)]
pub struct SearchResponse {
	pub page: usize,
	pub final_page: usize,
	pub torrents: Vec<TorrentInfo>,
}

#[derive(Debug)]
pub struct FileInfoResponse {
	pub files: Vec<TorrentFile>,
}

#[derive(Debug)]
pub struct TorrentResponse {
	pub file: Vec<u8>,
}

#[derive(Debug)]
pub struct TorrentInfo {
	pub label_id: u32,
	pub age: String,
	pub uploader: Option<String>,
	pub free_leech: bool,
	pub title: String,
	pub id: u32,
	pub torrent_title: String,
	pub comment_count: u32,
	pub size: u64,
	pub file_count: u32,
	pub snatches: u32,
	pub seeders: u32,
	pub leechers: u32,
}

#[derive(Debug)]
pub struct TorrentFile {
	pub title: String,
	pub size: u64,
}

impl SearchResponse {
	pub(crate) fn new(s: String) -> Result<Self, Error> {
		unimplemented!()
	}
}

impl TorrentResponse {
	pub(crate) fn new(b: Vec<u8>) -> Self {
		Self {
			file: b,
		}
	}
}

impl FileInfoResponse {
	pub(crate) fn new(s: String) -> Result<Self, Error> {
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
		let size = {
			str_to_byte_count(&rows[5].inner_html())
				.ok_or(HtmlError::InvalidValue(value.html(), "'num' of 'value' of element 'td'", 6))?
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
			size: size,
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