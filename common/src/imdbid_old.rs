use regex::{self, Regex};
use lazy_static::lazy_static;

pub trait Imdbid {
	type Error;
	fn is_valid(&self) -> bool;
	fn to_string(&self, pad: usize) -> Result<String, Self::Error>;
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error>;
}

pub struct ImdbidOwned<T: Imdbid> {
	inner: T,
}

macro_rules! impl_uint_imdbid {
	($type:ident) => {
		impl Imdbid for $type {
			type Error = std::convert::Infallible;
			fn is_valid(&self) -> bool {
				true
			}
			fn to_string(&self, pad: usize) -> Result<String, Self::Error> {
				Ok(format!("tt{:0>1$}", self, pad))
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				Ok(format!("{:0>1$}", self, pad))
			}
		}

		impl Imdbid for ImdbidOwned<$type> {
			type Error = std::convert::Infallible;
			fn is_valid(&self) -> bool {
				true
			}
			fn to_string(&self, pad:usize) -> Result<String, Self::Error> {
				Ok(format!("tt{:0>1$}", &self.inner, pad))
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				Ok(format!("{:0>1$}", &self.inner, pad))
			}
		}

		impl std::convert::TryFrom<$type> for ImdbidOwned<$type> {
			type Error = std::convert::Infallible;
			fn try_from(value: $type) -> Result<Self, Self::Error> {
				Ok(Self{ inner: value })
			}
		}
	}
}

macro_rules! impl_int_imdbid {
	($type:ident) => {
		impl Imdbid for $type {
			type Error = &'static str;
			fn is_valid(&self) -> bool {
				self > &0
			}
			fn to_string(&self, pad: usize) -> Result<String, Self::Error> {
				match self.is_valid() {
					false => Err("Imdbids cannot be negative."),
					true => Ok(format!("tt{:0>1$}", self, pad)),
				}
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				match self.is_valid() {
					false => Err("Imdbids cannot be negative."),
					true => Ok(format!("{:0>1$}", self, pad)),
				}
			}
		}

		impl Imdbid for ImdbidOwned<$type> {
			type Error = std::convert::Infallible;
			fn is_valid(&self) -> bool {
				true
			}
			fn to_string(&self, pad:usize) -> Result<String, Self::Error> {
				Ok(format!("tt{:0>1$}", &self.inner, pad))
			}
			fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
				Ok(format!("{:0>1$}", &self.inner, pad))
			}
		}

		impl std::convert::TryFrom<$type> for ImdbidOwned<$type> {
			type Error = &'static str;
			fn try_from(value: $type) -> Result<Self, Self::Error> {
				match &value.is_valid() {
					false => Err("Imdbids cannot be negative."),
					true => Ok(Self{ inner: value })
				}
			}
		}
	}
}

impl_uint_imdbid!(u8);
impl_uint_imdbid!(u16);
impl_uint_imdbid!(u32);
impl_uint_imdbid!(u64);
impl_uint_imdbid!(usize);

impl_int_imdbid!(i8);
impl_int_imdbid!(i16);
impl_int_imdbid!(i32);
impl_int_imdbid!(i64);
impl_int_imdbid!(isize);

lazy_static! {
	static ref IMDBID_FULL: Regex = Regex::new(r"^tt[0-9]+$").unwrap();
	static ref IMDBID_DIGITS: Regex = Regex::new(r"^[0-9]+$").unwrap();
	static ref DIGITS: Regex = Regex::new(r"[1-9]+$").unwrap();
}

impl Imdbid for str {
	type Error = &'static str;
	fn is_valid(&self) -> bool {
		IMDBID_FULL.is_match(self) || IMDBID_DIGITS.is_match(self)
	}
	fn to_string(&self, pad:usize) -> Result<String, Self::Error> {
		match self.is_valid() {
			false => Err("Invalid Imdbid."),
			true => Ok(format!("tt{:0>1$}", DIGITS.find(self).unwrap().as_str(), pad)),
		}
	}
	fn to_digits(&self, pad:usize) -> Result<String, Self::Error> {
		match self.is_valid() {
			false => Err("Invalid Imdbid."),
			true => Ok(format!("{:0>1$}", DIGITS.find(self).unwrap().as_str(), pad)),
		}
	}
}

impl Imdbid for String {
	type Error = <str as Imdbid>::Error;
	fn is_valid(&self) -> bool {
		<str as Imdbid>::is_valid(self)
	}
	fn to_string(&self, pad:usize) -> Result<String, Self::Error> {
		<str as Imdbid>::to_string(self, pad)
	}
	fn to_digits(&self, pad:usize) -> Result<String, Self::Error> {
		<str as Imdbid>::to_digits(self, pad)
	}
}

impl Imdbid for ImdbidOwned<String> {
	type Error = std::convert::Infallible;
	fn is_valid(&self) -> bool {
		true
	}
	fn to_string(&self, pad:usize) -> Result<String, Self::Error> {
		Ok(format!("tt{:0>1$}", DIGITS.find(&self.inner).unwrap().as_str(), pad))
	}
	fn to_digits(&self, pad: usize) -> Result<String, Self::Error> {
		Ok(format!("{:0>1$}", DIGITS.find(&self.inner).unwrap().as_str(), pad))
	}
}

impl std::convert::TryFrom<String> for ImdbidOwned<String> {
	type Error = &'static str;
	fn try_from(value: String) -> Result<Self, Self::Error> {
		match &value.is_valid() {
			false => Err("Invalid Imdbid."),
			true => Ok(Self{ inner: value })
		}
	}
}