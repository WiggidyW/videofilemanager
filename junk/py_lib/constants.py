from pathlib import Path
import sqlite3
import os

class classproperty(object):

    def __init__(self, fget):
        self.fget = fget

    def __get__(self, owner_self, owner_cls):
        return self.fget(owner_cls)

class Constants:

	_CONN = None
	_MASTER_PATH = None
	_PLEX_PATH = None
	_SUBTITLES_DIR = None
	_VIDEO_DIR = None
	_PLEX_MOVIE_DIR = None
	_PLEX_TV_DIR = None
	_OMDB_PLOT = None
	_OMDB_APIKEY = None
	_IMDBID_DIGITS = None
	_WATCHER_LOG = None
	_OST_AGENT = None
	_OST_LANG = None
	_SERVER_IP = None
	_SERVER_PORT = None

	@classproperty
	def CONN(cls):
		if cls._CONN:
			return cls._CONN
		else:
			cls._CONN = sqlite3.connect(os.environ['DB_PATH'])
			return cls._CONN

	@classproperty
	def MASTER_PATH(cls):
		if cls._MASTER_PATH:
			return cls._MASTER_PATH
		else:
			cls._MASTER_PATH = Path(os.environ['MASTER_PATH'])
			return cls._MASTER_PATH

	@classproperty
	def SUBTITLES_DIR(cls):
		if cls._SUBTITLES_DIR:
			return cls._SUBTITLES_DIR
		else:
			cls._SUBTITLES_DIR = os.environ['SUBTITLES_DIR']
			return cls._SUBTITLES_DIR

	@classproperty
	def VIDEO_DIR(cls):
		if cls._VIDEO_DIR:
			return cls._VIDEO_DIR
		else:
			cls._VIDEO_DIR = os.environ['VIDEO_DIR']
			return cls._VIDEO_DIR

	@classproperty
	def PLEX_PATH(cls):
		if cls._PLEX_PATH:
			return cls._PLEX_PATH
		else:
			cls._PLEX_PATH = Path(os.environ['PLEX_PATH'])
			return cls._PLEX_PATH

	@classproperty
	def PLEX_MOVIE_DIR(cls):
		if cls._PLEX_MOVIE_DIR:
			return cls._PLEX_MOVIE_DIR
		else:
			cls._PLEX_MOVIE_DIR = os.environ['PLEX_MOVIE_DIR']
			return cls._PLEX_MOVIE_DIR

	@classproperty
	def PLEX_TV_DIR(cls):
		if cls._PLEX_TV_DIR:
			return cls._PLEX_TV_DIR
		else:
			cls._PLEX_TV_DIR = os.environ['PLEX_TV_DIR']
			return cls._PLEX_TV_DIR

	@classproperty
	def OMDB_PLOT(cls):
		if cls._OMDB_PLOT:
			return cls._OMDB_PLOT
		else:
			cls._OMDB_PLOT = os.environ['OMDB_PLOT']
			return cls._OMDB_PLOT

	@classproperty
	def OMDB_APIKEY(cls):
		if cls._OMDB_APIKEY:
			return cls._OMDB_APIKEY
		else:
			cls._OMDB_APIKEY = os.environ['OMDB_APIKEY']
			return cls._OMDB_APIKEY

	@classproperty
	def IMDBID_DIGITS(cls):
		if cls._IMDBID_DIGITS:
			return cls._IMDBID_DIGITS
		else:
			cls._IMDBID_DIGITS = int(os.environ['IMDBID_DIGITS'])
			return cls._IMDBID_DIGITS

	@classproperty
	def WATCHER_LOG(cls):
		if cls._WATCHER_LOG:
			return cls._WATCHER_LOG
		else:
			cls._WATCHER_LOG = Path(os.environ['WATCHER_LOG'])
			return cls._WATCHER_LOG

	@classproperty
	def OST_AGENT(cls):
		if cls._OST_AGENT:
			return cls._OST_AGENT
		else:
			cls._OST_AGENT = os.environ['OST_AGENT']
			return cls._OST_AGENT

	@classproperty
	def OST_LANG(cls):
		if cls._OST_LANG:
			return cls._OST_LANG
		else:
			cls._OST_LANG = os.environ['OST_LANG']
			return cls._OST_LANG

	@classproperty
	def SERVER_PORT(cls):
		if cls._SERVER_PORT:
			return cls._SERVER_PORT
		else:
			cls._SERVER_PORT = int(os.environ['SERVER_PORT'])
			return cls._SERVER_PORT

	@classproperty
	def SERVER_IP(cls):
		if cls._SERVER_IP:
			return cls._SERVER_IP
		else:
			cls._SERVER_IP = os.environ['SERVER_IP']
			return cls._SERVER_IP