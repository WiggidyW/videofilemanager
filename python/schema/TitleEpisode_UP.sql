CREATE TABLE ImdbDatasetTitleEpisode (
	imdbid INTEGER PRIMARY KEY,
	seriesid INTEGER NOT NULL,
	season INTEGER,
	episode INTEGER
);