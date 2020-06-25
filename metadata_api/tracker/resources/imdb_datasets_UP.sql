CREATE TABLE IF NOT EXISTS title_principals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    ordering INTEGER NOT NULL,
    name_id INTEGER NOT NULL,
    category TEXT NOT NULL,
    job TEXT,
    characters TEXT,
    UNIQUE (imdb_id, ordering)
);

CREATE TABLE IF NOT EXISTS name_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_id INTEGER UNIQUE NOT NULL,
    name TEXT NOT NULL,
    birth_year INTEGER,
    death_year INTEGER,
    primary_profession TEXT,
    imdb_ids TEXT
);

CREATE TABLE IF NOT EXISTS title_akas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    ordering INTEGER NOT NULL,
    title TEXT,
    region TEXT,
    language TEXT,
    types TEXT,
    attributes TEXT,
    is_original_title BOOL,
    UNIQUE (imdb_id, ordering)
);

CREATE TABLE IF NOT EXISTS title_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER UNIQUE NOT NULL,
    title_type TEXT NOT NULL,
    primary_title TEXT,
    original_title TEXT,
    is_adult BOOL NOT NULL,
    start_year INTEGER,
    end_year INTEGER,
    runtime_minutes INTEGER,
    genres TEXT
);

CREATE TABLE IF NOT EXISTS title_crew (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER UNIQUE NOT NULL,
    directors TEXT,
    writers TEXT
);

CREATE TABLE IF NOT EXISTS title_episode (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER UNIQUE NOT NULL,
    series_id INTEGER NOT NULL,
    season_number INTEGER,
    episode_number INTEGER
);

CREATE TABLE IF NOT EXISTS title_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER UNIQUE NOT NULL,
    average_rating FLOAT NOT NULL,
    num_votes INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS title_info (
    imdb_id INTEGER PRIMARY KEY,
    title_type TEXT,
    primary_title TEXT,
    original_title TEXT,
    is_adult BOOL,
    start_year INTEGER,
    end_year INTEGER,
    runtime_minutes INTEGER,
    genres TEXT,
    average_rating FLOAT,
    num_votes INTEGER,
    series_id INTEGER,
    season_number INTEGER,
    episode_number INTEGER,
    directors TEXT,
    writers TEXT
);