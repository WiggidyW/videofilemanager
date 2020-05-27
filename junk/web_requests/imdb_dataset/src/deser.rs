use bytes::Bytes;
use crate::model;

pub trait Deser: Sized {
	type Error: std::error::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error>;
}

impl Deser for model::NameBasics {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitleAkas {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitleBasics {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitleCrew {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitleEpisode {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitlePrincipals {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}

impl Deser for model::TitleRatings {
	type Error = std::io::Error;
	fn deser(b: Bytes) -> Result<Self, Self::Error> {
		unimplemented!()
	}
}