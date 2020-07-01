CREATE TABLE title_info (
    imdb_id INTEGER PRIMARY KEY,
    title_type TEXT,
    primary_title TEXT,
    original_title TEXT,
    is_adult BOOL,
    start_year INTEGER,
    end_year INTEGER,
    runtime_minutes INTEGER,
    genres TEXT[],
    average_rating REAL,
    num_votes INTEGER,
    series_id INTEGER,
    season_number INTEGER,
    episode_number INTEGER,
    writers INTEGER[],
    directors INTEGER[]
);

CREATE TYPE Principal AS (
    ordering INTEGER,
    category TEXT,
    job TEXT,
    characters TEXT
);

CREATE TABLE title_person (
    imdb_id INTEGER,
    name_id INTEGER,
    PRIMARY KEY (imdb_id, name_id),
    principals Principal[]
);

CREATE TABLE name_basics (
    name_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    birth_year INTEGER,
    death_year INTEGER,
    primary_profession TEXT[],
    imdb_ids INTEGER[]
);

CREATE TABLE title_akas (
    imdb_id INTEGER NOT NULL,
    ordering INTEGER NOT NULL,
    PRIMARY KEY (imdb_id, ordering),
    title TEXT,
    region TEXT,
    language TEXT,
    types TEXT,
    attributes TEXT,
    is_original_title BOOL
);

CREATE FUNCTION title_crew(INTEGER, INTEGER[], INTEGER[])
    RETURNS VOID AS $$
    DECLARE
        writer INTEGER;
        director INTEGER;
    BEGIN
        FOR writer IN 1 .. array_upper($2, 1) LOOP
            INSERT INTO title_person ( imdb_id, name_id, principals )
            VALUES ( $1, $2[writer], (0, 'writer', NULL, NULL) )
            ON CONFLICT ( imdb_id )
            DO UPDATE SET
                principals = array_append(EXCLUDED.principals, (0, 'writer', NULL, NULL));
        END LOOP;
        FOR director IN 1 .. array_upper($3, 1) LOOP
            INSERT INTO title_person ( imdb_id, name_id, principals )
            VALUES ( $1, $3[director], (0, 'director', NULL, NULL) )
            ON CONFLICT ( imdb_id )
            DO UPDATE SET
                principals = array_append(EXCLUDED.principals, (0, 'director', NULL, NULL));
        END LOOP;
    END;
    $$ LANGUAGE plpgsql;

CREATE FUNCTION title_principals(INTEGER, INTEGER, INTEGER, TEXT, TEXT, TEXT)
	RETURNS VOID AS $$
	BEGIN
		INSERT INTO title_person ( imdb_id, name_id, principals )
		VALUES ( $1, $3, ($2, $4, $5, $6) )
		ON CONFLICT ( imdb_id )
		DO UPDATE SET
			principals = array_append(EXCLUDED.principals, ($2, $4, $5, $6));
	END;
	$$ LANGUAGE plpgsql;