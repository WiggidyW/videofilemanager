use chrono::{DateTime, offset::Utc};
use std::collections::HashMap;

pub struct Data<'d> {
	timestamp: DateTime<Utc>,
	inner: HashMap<Parent, &'d [Child]>,
	children: [Child],
}

pub struct Parent {
	imdbid: String,
}

pub struct Child {
	imdbid: String,
	season: Option<u16>,
	episode: Option<u16>,
}