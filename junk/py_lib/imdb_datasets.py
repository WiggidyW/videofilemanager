from abc import ABC, abstractmethod
from constants import Constants
from typing import List
import urllib.request
import sqlite3
import gzip
import csv
import io

class Metadata(ABC):

	@property
	@abstractmethod
	def url(self) -> str:
		raise NotImplementedError

	@abstractmethod
	def parse(self, row:List[str]) -> list:
		raise NotImplementedError

	@abstractmethod
	def insert(self, cursor:sqlite3.Cursor, parsed_row:list) -> None:
		raise NotImplementedError

	def request(self) -> bytes:
		res = urllib.request.urlopen(self.url)
		return res.read()

	def refresh(self, cursor:sqlite3.Cursor) -> None:
		with gzip.open(io.BytesIO(self.request()), 'rt') as f:
			reader = csv.reader(f, delimiter='\t')
			next(reader) # skip header line
			for row in reader:
				self.insert(cursor, self.parse(row))
		Constants.CONN.commit()

class TitleEpisode(Metadata):

	@property
	def url(self):
		return "https://datasets.imdbws.com/title.episode.tsv.gz"

	def parse(self, row:List[str]) -> list:
		return [
			int(row[0][2:]),
			int(row[1][2:]),
			int(row[2]) if row[2].isdigit() else None,
			int(row[3]) if row[3].isdigit() else None,
		]

	def insert(self, cursor:sqlite3.Cursor, parsed_row:list) -> None:
		cursor.execute('''
			INSERT OR IGNORE INTO ImdbDatasetTitleEpisode VALUES (?,?,?,?)
		''', parsed_row)