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

CREATE TRIGGER IF NOT EXISTS update_from_title_basics
    AFTER INSERT ON title_basics
BEGIN
    INSERT OR IGNORE INTO title_info (imdb_id) VALUES (NEW.imdb_id);
    UPDATE title_info
    SET
        title_type = NEW.title_type,
        primary_title = NEW.primary_title,
        original_title = NEW.original_title,
        is_adult = NEW.is_adult,
        start_year = NEW.start_year,
        end_year = NEW.end_year,
        runtime_minutes = NEW.runtime_minutes,
        genres = NEW.genres
    WHERE
        imdb_id = NEW.imdb_id;
END;

CREATE TRIGGER IF NOT EXISTS update_from_title_crew
    AFTER INSERT ON title_crew
BEGIN
    INSERT OR IGNORE INTO title_info (imdb_id) VALUES (NEW.imdb_id);
    UPDATE title_info
    SET
        directors = NEW.directors,
        writers = NEW.writers
    WHERE
        imdb_id = NEW.imdb_id;
END;

CREATE TRIGGER IF NOT EXISTS update_from_title_episode
    AFTER INSERT ON title_episode
BEGIN
    INSERT OR IGNORE INTO title_info (imdb_id) VALUES (NEW.imdb_id);
    UPDATE title_info
    SET
        series_id = NEW.series_id,
        season_number = NEW.season_number,
        episode_number = NEW.episode_number
    WHERE
        imdb_id = NEW.imdb_id;
END;

CREATE TRIGGER IF NOT EXISTS update_from_title_ratings
    AFTER INSERT ON title_ratings
BEGIN
    INSERT OR IGNORE INTO title_info (imdb_id) VALUES (NEW.imdb_id);
    UPDATE title_info
    SET
        average_rating = NEW.average_rating,
        num_votes = NEW.num_votes
    WHERE
        imdb_id = NEW.imdb_id;
END;