CREATE TABLE ImdbDatasetTitleEpisode (
	imdbID INTEGER PRIMARY KEY,
	seriesID INTEGER NOT NULL,
	season INTEGER,
	episode INTEGER
);