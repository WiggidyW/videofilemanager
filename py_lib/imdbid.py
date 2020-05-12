from typing import Union, Optional, List
from constants import Constants
from subtitle import Subtitle
from pathlib import Path
import imdb_datasets
import imdbid_db
import sqlite3
import omdb
import plex
import util

class Imdbid:

	def __init__(self, value: Union[str, int]) -> None:
		self._imdbid = int(util.Imdbid.digits(value, 0))

	## General ##

	@classmethod
	def all(cls) -> List['Imdbid']:
		return [Imdbid(entry.name) for entry in Constants.MASTER_PATH.iterdir() \
			if util.Imdbid.valid(entry.name)]

	@property
	def cursor(self) -> sqlite3.Cursor:
		try:
			return self._cursor
		except AttributeError:
			self._cursor: sqlite3.Cursor = Constants.CONN.cursor()
			return self._cursor

	@property
	def imdbid(self) -> int:
		return self._imdbid

	@property
	def path(self) -> Path:
		try:
			return self._path
		except AttributeError:
			self._path: Path = Constants.MASTER_PATH / util.Imdbid.full(self.imdbid, Constants.IMDBID_DIGITS)
			return self._path

	## Subtitles ##

	@property
	def subtitles_path(self) -> Path:
		try:
			return self._subtitles_path
		except AttributeError:
			self._subtitles_path: Path = self.path / Constants.SUBTITLES_DIR
			return self._subtitles_path

	@property
	def subtitles(self) -> List[Path]:
		try:
			return [Subtitle(subtitle) for subtitle in self.subtitles_path.iterdir()]
		except FileNotFoundError:
			return []

	## Video ##

	@property
	def video_path(self) -> Path:
		try:
			return self._video_path
		except AttributeError:
			self._video_path: Path = self.path / Constants.VIDEO_DIR
			return self._video_path

	@property
	def video(self) -> Optional[Path]:
		try:
			return next(self.video_path.iterdir())
		except FileNotFoundError:
			return None

	## OMDB ##

	@property
	def omdb(self):
		try:
			return self._omdb
		except AttributeError:
			refr = lambda refr : omdb.ByImdbid().refresh(refr, self.imdbid, Constants.OMDB_PLOT, Constants.OMDB_APIKEY)
			self._omdb = imdbid_db.Db('Omdb', ('imdbID', self.imdbid), self.cursor, refr)
			return self._omdb

	@property
	def series_omdb(self):
		try:
			return self._series_omdb
		except AttributeError:
			if self.omdb['type'] == 'episode':
				refr = lambda refr : omdb.ByImdbid().refresh(refr, self.title_episode['seriesID'], Constants.OMDB_PLOT, Constants.OMDB_APIKEY)
				self._series_omdb = imdbid_db.Db('Omdb', ('imdbID', self.title_episode['seriesID']), self.cursor, refr)
				return self._series_omdb
			else:
				return None

	## IMDB Datasets ##

	@property
	def title_episode(self):
		try:
			return self._title_episode
		except AttributeError:
			if self.omdb['type'] == 'episode':
				refr = lambda refr : imdb_datasets.TitleEpisode().refresh(refr)
				self._title_episode = imdbid_db.Db('ImdbDatasetTitleEpisode', ('imdbID', self.imdbid), self.cursor, refr)
				return self._title_episode
			else:
				return None

	## PLEX ##

	@property
	def plex_path(self) -> Optional[Path]:
		try:
			return self._plex_path
		except AttributeError:
			if self.omdb['type'] == 'movie':
				self._plex_path: Path = Constants.PLEX_PATH / Constants.PLEX_MOVIE_DIR / '{} ({})'.format(self.omdb['title'], self.omdb['year'])
			elif self.omdb['type'] == 'episode':
				if self.title_episode['season']:
					self._plex_path = Constants.PLEX_PATH / Constants.PLEX_TV_DIR / '{} ({})'.format(self.series_omdb['title'], self.series_omdb['year'][:4]) / 'Season {}'.format(self.title_episode['season'])
				else:
					self._plex_path = Constants.PLEX_PATH / Constants.PLEX_TV_DIR / '{} ({})'.format(self.series_omdb['title'], self.series_omdb['year'][:4])
			else:
				return None
			return self._plex_path

	@property
	def plex_name(self) -> Optional[str]:
		try:
			return self._plex_name
		except AttributeError:
			if self.omdb['type'] == 'movie':
				self._plex_name: str = '{} ({})'.format(self.omdb['title'], self.omdb['year'])
			elif self.omdb['type'] == 'episode':
				if self.title_episode['season'] and self.title_episode['episode']:
					self._plex_name = 'S{:02d}E{:02d}'.format(self.title_episode['season'], self.title_episode['episode'])
				else:
					self._plex_name = self.omdb['title']
			else:
				return None
			return self._plex_name

	def plex_refresh(self):
		plex.refresh(self.subtitles, self.video, self.plex_path / self.plex_name, self.imdbid)