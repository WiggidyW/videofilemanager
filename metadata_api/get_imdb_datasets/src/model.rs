struct TitleRatings {
	imdb_id: u32,
	average_rating: f32,
	num_votes: u32,
}

struct TitleEpisode {
	imdb_id: u32,
	series_id: u32,
	season_number: Option<u32>,
	episode_number: Option<u32>,
}

struct TitleCrew {
	imdb_id: u32,
	directors: Option<Vec<u32>>,
	writers: Option<Vec<u32>>,
}

struct TitleBasics {
	imdb_id: u32,
	title_type: String,
	primary_title: String,
	original_title: String,
	is_adult: bool,
	start_year: Option<u32>,
	end_year: Option<u32>,
	runtime_minutes: Option<u32>,
	genres: Option<Vec<String>>,
}

struct TitleAkas {
	imdb_id: u32,
	ordering: u32,
	title: String,
	region: Option<String>,
	language: Option<String>,
	types: Option<String>,
	attributes: Option<String>,
	is_original_title: Option<bool>,
}

struct NameBasics {
	person_id: u32,
	primary_name: String,
	birth_year: Option<u32>,
	death_year: Option<u32>,
	primary_profession: Option<Vec<String>>,
	known_for_titles: Option<Vec<u32>>,
}

struct TitlePrincipals {
	imdb_id: u32,
	ordering: u32,
	name_id: u32,
	category: String,
	job: Option<String>,
	characters: Option<String>,
}