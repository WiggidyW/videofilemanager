DROP TABLE IF EXISTS title_principals;
DROP TABLE IF EXISTS name_basics;
DROP TABLE IF EXISTS title_akas;
DROP TABLE IF EXISTS title_basics;
DROP TABLE IF EXISTS title_crew;
DROP TABLE IF EXISTS title_episode;
DROP TABLE IF EXISTS title_ratings;
CREATE TABLE title_principals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    ordering INTEGER NOT NULL,
    name_id INTEGER NOT NULL,
    category TEXT NOT NULL,
    job TEXT,
    characters TEXT
);
CREATE TABLE name_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    birth_year INTEGER,
    death_year INTEGER,
    primary_profession TEXT,
    imdb_ids TEXT
);
CREATE TABLE title_akas (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    ordering INTEGER NOT NULL,
    title TEXT,
    region TEXT,
    language TEXT,
    types TEXT,
    attributes TEXT,
    is_original_title BOOL
);
CREATE TABLE title_basics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    title_type TEXT NOT NULL,
    primary_title TEXT,
    original_title TEXT,
    is_adult BOOL NOT NULL,
    start_year INTEGER,
    end_year INTEGER,
    runtime_minutes INTEGER,
    genres TEXT
);
CREATE TABLE title_crew (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    directors TEXT,
    writers TEXT
);
CREATE TABLE title_episode (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    series_id INTEGER NOT NULL,
    season_number INTEGER,
    episode_number INTEGER
);
CREATE TABLE title_ratings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    imdb_id INTEGER NOT NULL,
    average_rating FLOAT NOT NULL,
    num_votes INTEGER NOT NULL
);