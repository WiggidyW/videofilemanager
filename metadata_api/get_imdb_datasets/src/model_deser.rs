use serde::{self, de::{self, Visitor}, Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Deserialize)]
struct TitleRatings {
	imdb_id: String,
	average_rating: f32,
	num_votes: u32,
}

#[derive(Debug, Deserialize)]
struct TitleEpisode {
	imdb_id: String,
	series_id: String,
	season_number: Option<u32>,
	episode_number: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct TitleCrew {
	imdb_id: String,
	directors: Option<Vec<String>>,/**/
	writers: Option<Vec<String>>,/**/
}

#[derive(Debug, Deserialize)]
struct TitleBasics {
	imdb_id: String,
	title_type: String,
	primary_title: String,
	original_title: String,
	is_adult: Bool,/**/
	start_year: Option<u32>,
	end_year: Option<u32>,
	runtime_minutes: Option<u32>,
	genres: Option<Vec<String>>,/**/
}

#[derive(Debug, Deserialize)]
struct TitleAkas {
	imdb_id: String,
	ordering: u32,
	title: String, // Sometimes is only whitespace
	region: Option<String>,/**/
	language: Option<String>,/**/
	types: Option<String>,/**/
	attributes: Option<String>,/**/
	is_original_title: Option<Bool>,/**/
}

#[derive(Debug, Deserialize)]
struct NameBasics {
	person_id: String,
	primary_name: String,
	birth_year: Option<u32>,
	death_year: Option<u32>,
	primary_profession: Option<Vec<String>>, // Sometimes is only empty
	known_for_titles: Option<Vec<String>>,/**/
}

#[derive(Debug, Deserialize)]
struct TitlePrincipals {
	imdb_id: String,
	ordering: u32,
	name_id: String,
	category: String,
	job: Option<String>,/**/
	characters: Option<String>,/**/
}

// impl<'de> Deserialize<'de> for String {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 	where
// 		D: Deserializer<'de>,
// 	{
// 		let s: &str = Deserialize::deserialize(deserializer)?;
// 		match s.starts_with("tt") {
// 			true => Ok(Self(s
// 				.split_at(2)
// 				.1
// 				.parse()
// 				.map_err(de::Error::custom)?)),
// 			false => Ok(Self(s
// 				.parse()
// 				.map_err(de::Error::custom)?)),
// 		}
// 	}
// }

// impl<'de> Deserialize<'de> for String {
// 	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
// 	where
// 		D: Deserializer<'de>,
// 	{
// 		let s: &str = Deserialize::deserialize(deserializer)?;
// 		match s.starts_with("nm") {
// 			true => Ok(Self(s
// 				.split_at(2)
// 				.1
// 				.parse()
// 				.map_err(de::Error::custom)?)),
// 			false => Ok(Self(s
// 				.parse()
// 				.map_err(de::Error::custom)?)),
// 		}
// 	}
// }

// fn optional_str<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let s: String = Deserialize::deserialize(deserializer)?;
// 	match s.as_str() {
// 		"\\N" => Ok(None),
// 		"" => Ok(None),
// 		_ => Ok(Some(s)),
// 	}
// }

// fn optional_num<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
// where
// 	D: Deserializer<'de>,
// {
// 	let s: &str = Deserialize::deserialize(deserializer)?;
// 	match s {
// 		"\\N" => Ok(None),
// 		"" => Ok(None),
// 		_ => Ok(Some(s
// 			.parse()
// 			.map_err(de::Error::custom)?)),
// 	}
// }

// // fn optional<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
// // where
// // 	D: Deserializer<'de>,
// // 	T: Deserialize<'de>,
// // {
// // 	let b: &[u8] = Deserialize::deserialize(deserializer)?;
// // 	match b {
// // 		&[b'\\', b'N'] => Ok(None),
// // 		&[] => Ok(None),
// // 		_ => Ok(Some(T::deserialize(deserializer)?)),
// // 	}
// // }