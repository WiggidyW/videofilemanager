use regex::Regex;
use lazy_static::lazy_static;

pub trait Imdbid {
	type Error: std::fmt::Debug;
	fn is_valid(&self) -> Result<(), Self::Error>;
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error>;
	fn to_string(&self, pad: usize) -> Result<String, Self::Error> {
		match self.to_digits(pad) {
			Err(e) => Err(e),
			Ok(s) => Ok(format!("tt{}", s)),
		}
	}
}

#[derive(Debug)]
pub struct ImdbidOwned<T> {
	inner: T,
}

impl<T> Imdbid for ImdbidOwned<T> where
	T: Imdbid
{
	type Error = std::convert::Infallible;
	fn is_valid(&self) -> Result<(), Self::Error> {
		Ok(())
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		Ok(<T as Imdbid>::to_digits(&self.inner, pad).unwrap())
	}
}

lazy_static! {
	static ref IMDBID: Regex = Regex::new(r"^(tt)?[0-9]{0,7}[1-9]$").unwrap();
}

impl Imdbid for str {
	type Error = String;
	fn is_valid(&self) -> Result<(), Self::Error> {
		match IMDBID.is_match(self) {
			true => Ok(()),
			false => Err(format!("{} is an invalid Imdbid.", self)),
		}
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		match self.is_valid() {
			Err(e) => Err(e),
			Ok(_) => Ok(
				format!("{:0>1$}", self.split_at(
					self.find(|c| c != 't' && c != '0').unwrap()).1,
					pad)
				),
		}
	}
}

impl Imdbid for String {
	type Error = <str as Imdbid>::Error;
	fn is_valid(&self) -> Result<(), Self::Error> {
		<str as Imdbid>::is_valid(self)
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		<str as Imdbid>::to_digits(self, pad)
	}
}

impl Imdbid for std::ffi::OsStr {
	type Error = String;
	fn is_valid(&self) -> Result<(), Self::Error> {
		match self.to_str() {
			Some(s) => <str as Imdbid>::is_valid(s),
			None => Err(format!("{:?} is an invalid Imdbid. It is invalid unicode.", self)),
		}
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		match self.to_str() {
			Some(s) => <str as Imdbid>::to_digits(s, pad),
			None => Err(format!("{:?} is an invalid Imdbid. It is invalid unicode.", self)),
		}
	}
}

impl Imdbid for std::ffi::OsString {
	type Error = <std::ffi::OsStr as Imdbid>::Error;
	fn is_valid(&self) -> Result<(), Self::Error> {
		<std::ffi::OsStr as Imdbid>::is_valid(self)
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		<std::ffi::OsStr as Imdbid>::to_digits(self, pad)
	}
}

impl Imdbid for std::path::Path {
	type Error = String;
	fn is_valid(&self) -> Result<(), Self::Error> {
		match self.ancestors()
			.filter_map(|p| p.file_name())
			.filter_map(|p| p.to_str())
			.filter(|p| IMDBID.is_match(p))
			.next()
		{
			Some(_) => Ok(()),
			None => Err(format!("{} has no valid Imdbid ancestors.", self.display())),
		}
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		for ancestor in self.ancestors()
			.filter_map(|p| p.file_name())
			.filter_map(|p| p.to_str())
		{
			match <str as Imdbid>::to_digits(ancestor, pad) {
				Err(_) => (),
				Ok(s) => return Ok(s),
			}
		}
		Err(format!("{} has no valid Imdbid ancestors.", self.display()))
	}
}

impl Imdbid for std::path::PathBuf {
	type Error = <std::path::Path as Imdbid>::Error;
	fn is_valid(&self) -> Result<(), Self::Error> {
		<std::path::Path as Imdbid>::is_valid(self)
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		<std::path::Path as Imdbid>::to_digits(self, pad)
	}
}

macro_rules! impl_imdbid_try_from {
	($type:ty) => {
		impl std::convert::TryFrom<$type> for ImdbidOwned<$type> {
			type Error = <$type as Imdbid>::Error;
			fn try_from(value: $type) -> Result<Self, Self::Error> {
				match <$type as Imdbid>::is_valid(&value) {
					Ok(_) => Ok(Self{ inner: value }),
					Err(e) => Err(e),
				}
			}
		}
	}
}

macro_rules! impl_imdbid_small_num {
	($type:ty) => {
		impl Imdbid for $type {
			type Error = String;
			fn is_valid(&self) -> Result<(), Self::Error> {
				match self > &0 {
					true => Ok(()),
					false => Err(format!("{} is an invalid Imdbid as it is <= 0 or >= 100,000,000.", self)),
				}
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				match self.is_valid() {
					Err(e) => Err(e),
					Ok(_) => Ok(format!("{:0>1$}", self, pad)),
				}
			}
		}
}}

macro_rules! impl_imdbid_big_num {
	($type:ty) => {
		impl Imdbid for $type {
			type Error = String;
			fn is_valid(&self) -> Result<(), Self::Error> {
				match self > &0 && self < &100_000_000 {
					true => Ok(()),
					false => Err(format!("{} is an invalid Imdbid as it is <= 0 or >= 100,000,000.", self)),
				}
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				match self.is_valid() {
					Err(e) => Err(e),
					Ok(_) => Ok(format!("{:0>1$}", self, pad)),
				}
			}
		}
}}

impl_imdbid_small_num!(u8);
impl_imdbid_small_num!(i8);
impl_imdbid_small_num!(u16);
impl_imdbid_small_num!(i16);
impl_imdbid_big_num!(u32);
impl_imdbid_big_num!(i32);
impl_imdbid_big_num!(u64);
impl_imdbid_big_num!(i64);
impl_imdbid_big_num!(usize);
impl_imdbid_big_num!(isize);

impl_imdbid_try_from!(String);
impl_imdbid_try_from!(std::ffi::OsString);
impl_imdbid_try_from!(std::path::PathBuf);
impl_imdbid_try_from!(u8);
impl_imdbid_try_from!(u16);
impl_imdbid_try_from!(u32);
impl_imdbid_try_from!(u64);
impl_imdbid_try_from!(usize);
impl_imdbid_try_from!(i8);
impl_imdbid_try_from!(i16);
impl_imdbid_try_from!(i32);
impl_imdbid_try_from!(i64);
impl_imdbid_try_from!(isize);