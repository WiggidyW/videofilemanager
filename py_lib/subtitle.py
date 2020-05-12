from constants import Constants
from pathlib import Path
from imdbid_db import Db
from typing import Union, Callable
import hashlib
import sqlite3
import shutil
import json
import util
import time
import ost

# https://stackoverflow.com/questions/29850801/
class Subtitle(type(Path())): # type: ignore

	@classmethod
	def _new(cls, path:Path) -> 'Subtitle':
		self = Subtitle(path)
		self._init_metadata()
		return self

	def _init_metadata(self):
		SubtitleMetadata.insert(self.cursor, self.hash, self.imdbid)
		Constants.CONN.commit()

	@classmethod
	def new(cls, sub:Path, imdbid:Union[int, str]):
		target = Constants.MASTER_PATH / util.Imdbid.full(imdbid, Constants.IMDBID_DIGITS) / Constants.SUBTITLES_DIR
		if target.exists():
			new = cls.recursive_rename(target, sub)
			if sub != new:
				sub.rename(new)
				sub = new
		else:
			target.mkdir(parents=True)
		shutil.move(str(sub), target / sub.name)
		return cls._new(target / sub.name)

	@classmethod
	def _ost_new(cls, path:Path, data:dict):
		self = cls._new(path)
		self._init_ost_metadata(int(data['IDSubtitleFile']), data)
		return self

	def _init_ost_metadata(self, subid:int, data:dict):
		OstMetadata.insert(self.cursor, self.hash, subid, data)
		Constants.CONN.commit()

	@classmethod
	def ost_new(cls, imdbid:Union[int, str], lang:str=Constants.OST_LANG, filtr:Callable[[dict], bool]=lambda filtr : True):
		data = ost.get_metadata(Constants.OST_AGENT, imdbid, lang)
		target = Constants.MASTER_PATH / util.Imdbid.full(imdbid, Constants.IMDBID_DIGITS) / Constants.SUBTITLES_DIR
		for entry in data:
			if entry['SubLanguageID'] == lang:
				c = Constants.CONN.cursor()
				c.execute('SELECT EXISTS(SELECT 1 FROM Ost WHERE subID=(?))', (int(entry['IDSubtitleFile']),))
				if not bool(c.fetchone()[0]):
					if filtr(entry):
						if target.exists():
							target = cls.recursive_rename(target, target / entry['SubFileName'])
						else:
							target.mkdir(parents=True)
							target = target / entry['SubFileName']
						ost.download(entry, target)
						return cls._ost_new(target, entry)

	@staticmethod
	def recursive_rename(directory:Path, file:Path) -> Path:
		for entry in directory.iterdir():
			if entry.name == file.name:
				new = file.with_name(file.stem + '_' + file.suffix)
				return Subtitle.recursive_rename(directory, new)
		return file

	@property
	def imdbid(self) -> int:
		try:
			return self._imdbid
		except AttributeError:
			self._imdbid: int = int(util.Imdbid.digits(self, 0))
			return self._imdbid

	@property
	def hash(self) -> str:
		h = hashlib.sha256()
		with open(self, 'rb') as f:
			h.update(f.read())
		return h.hexdigest()

	@property
	def cursor(self) -> sqlite3.Cursor:
		try:
			return self._cursor
		except AttributeError:
			self._cursor: sqlite3.Cursor = Constants.CONN.cursor()
			return self._cursor

	@property
	def metadata(self):
		try:
			return self._metadata
		except AttributeError:
			refr = lambda refr : SubtitleMetadata.insert(refr, self.hash, self.imdbid)
			self._metadata = Db('Subtitles', ('hash', self.hash), self.cursor, refr)
			return self._metadata

	@property
	def ost(self):
		try:
			return self._ost
		except AttributeError:
			refr = lambda refr : ()
			db = Db('Ost', ('hash', self.hash), self.cursor, refr)
			if db.exists():
				self._ost = db
				return self._ost
			else:
				return None

class SubtitleMetadata:

	@staticmethod
	def insert(cursor:sqlite3.Cursor, hash_:str, imdbid:int) -> None:
		cursor.execute('''
			INSERT OR REPLACE INTO Subtitles VALUES (?,?,?)
		''', (hash_, imdbid, time.time(),))

class OstMetadata:

	@staticmethod
	def insert(cursor:sqlite3.Cursor, hash_:str, subid:int, data:dict) -> None:
		cursor.execute('''
			INSERT OR REPLACE INTO Ost VALUES(?,?,?,?)
		''', (hash_, subid, time.time(), json.dumps(data, separators=(',', ':')),))