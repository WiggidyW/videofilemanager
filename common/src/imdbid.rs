use regex::Regex;
use lazy_static::lazy_static;

pub trait AsImdbid {
	fn digits(&self, pad: usize) -> String;
	fn full(&self, pad: usize) -> String {
		format!("tt{}", self.digits(pad))
	}
}

macro_rules! format_digits {
	($item:tt, $pad:ident) => {
		format!("{:0>1$}", $item, $pad)
	}
}

macro_rules! impl_as_imdbid_str {
	($type:ty) => { impl AsImdbid for $type {
		fn digits(&self, pad: usize) -> String {
			match &self[0..2] {
				"tt" => format_digits!((self.split_at(2).1), pad),
				_ => format_digits!(self, pad),
		}}
}}}

macro_rules! impl_as_imdbid_osstr {
	($type:ty) => { impl AsImdbid for $type {
		fn digits(&self, pad: usize) -> String {
			let s = self.to_str().unwrap();
			match &s[0..2] {
				"tt" => format_digits!((s.split_at(2).1), pad),
				_ => format_digits!(s, pad),
		}}
}}}

macro_rules! impl_as_imdbid_num {
	($type:ty) => { impl AsImdbid for $type {
		fn digits(&self, pad: usize) -> String {
			format_digits!(self, pad)
		}
}}}

lazy_static! {
	static ref IMDBID_PATH: Regex = Regex::new(r"^tt[0-9]{0,7}[1-9]$").unwrap();
}
macro_rules! impl_as_imdbid_path {
	($type:ty) => { impl AsImdbid for $type {
		fn digits(&self, pad: usize) -> String {
			format_digits!(
				(self.ancestors()
					.filter_map(|p| p.file_name())
					.filter_map(|p| p.to_str())
					.filter(|p| IMDBID_PATH.is_match(p))
					.next()
					.unwrap()
					.split_at(2).1),
				pad
		)}
}}}

impl_as_imdbid_str!(str);
impl_as_imdbid_str!(String);

impl_as_imdbid_osstr!(std::ffi::OsStr);
impl_as_imdbid_osstr!(std::ffi::OsString);

impl_as_imdbid_num!(u8);
impl_as_imdbid_num!(i8);
impl_as_imdbid_num!(u16);
impl_as_imdbid_num!(i16);
impl_as_imdbid_num!(u32);
impl_as_imdbid_num!(i32);
impl_as_imdbid_num!(u64);
impl_as_imdbid_num!(i64);
impl_as_imdbid_num!(usize);
impl_as_imdbid_num!(isize);

impl_as_imdbid_path!(std::path::Path);
impl_as_imdbid_path!(std::path::PathBuf);

pub enum Error {
	NumberZero,
	NumberNegative,
	NumberTooLarge,
	TextInvalid,
	TextInvalidUnicode,
	NoValidAncestors,
}

pub trait TryAsImdbid {
	fn valid(&self) -> Result<(), Error>;
	fn digits(&self, pad: usize) -> Result<String, Error>;
	fn full(&self, pad: usize) -> Result<String, Error> {
		Ok(format!("tt{}", self.digits(pad)?))
	}
}

lazy_static! {
	static ref IMDBID: Regex = Regex::new(r"^(tt)?[0-9]{0,7}[1-9]$").unwrap();
}

macro_rules! impl_try_as_imdbid_str {
	($type:ty) => { impl TryAsImdbid for $type {
		fn valid(&self) -> Result<(), Error> {
			match IMDBID.is_match(self) {
				true => Ok(()),
				false => Err(Error::TextInvalid),
		}}
		fn digits(&self, pad: usize) -> Result<String, Error> {
			self.valid()
				.map(|_| match &self[0..2] {
					"tt" => format_digits!((self.split_at(2).1), pad),
					_ => format_digits!(self, pad),
		})}
}}}

macro_rules! impl_try_as_imdbid_osstr {
	($type:ty) => { impl TryAsImdbid for $type {
		fn valid(&self) -> Result<(), Error> {
			match self.to_str() {
				None => Err(Error::TextInvalidUnicode),
				Some(s) => match IMDBID.is_match(s) {
					true => Ok(()),
					false => Err(Error::TextInvalid),
		}}}
		fn digits(&self, pad: usize) -> Result<String, Error> {
			self.valid()
				.map(|_| self.to_str().unwrap())
				.map(|s| match &s[0..2] {
					"tt" => format_digits!((s.split_at(2).1), pad),
					_ => format_digits!(s, pad),
		})}
}}}

macro_rules! impl_try_as_imdbid_num {
	($type:ty) => { impl TryAsImdbid for $type {
		fn valid(&self) -> Result<(), Error> {
			match self {
				&0 => Err(Error::NumberZero),
				_ if self < &0 => Err(Error::NumberNegative),
				_ if self > &99_999_999 => Err(Error::NumberTooLarge),
				_ => Ok(()),
		}}
		fn digits(&self, pad: usize) -> Result<String, Error> {
			self.valid()
				.map(|_| format_digits!(self, pad))
		}
}}}

macro_rules! impl_try_as_imdbid_smallnum {
	($type:ty) => { impl TryAsImdbid for $type {
		fn valid(&self) -> Result<(), Error> {
			match self {
				&0 => Err(Error::NumberZero),
				_ if self < &0 => Err(Error::NumberNegative),
				_ => Ok(()),
		}}
		fn digits(&self, pad: usize) -> Result<String, Error> {
			self.valid()
				.map(|_| format_digits!(self, pad))
		}
}}}

impl_try_as_imdbid_str!(str);
impl_try_as_imdbid_str!(String);

impl_try_as_imdbid_osstr!(std::ffi::OsStr);
impl_try_as_imdbid_osstr!(std::ffi::OsString);

impl_try_as_imdbid_smallnum!(u8);
impl_try_as_imdbid_smallnum!(i8);
impl_try_as_imdbid_smallnum!(u16);
impl_try_as_imdbid_smallnum!(i16);

impl_try_as_imdbid_num!(u32);
impl_try_as_imdbid_num!(i32);
impl_try_as_imdbid_num!(u64);
impl_try_as_imdbid_num!(i64);
impl_try_as_imdbid_num!(usize);
impl_try_as_imdbid_num!(isize);