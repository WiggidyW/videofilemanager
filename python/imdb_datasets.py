import sqlite3
from typing import List
from abc import ABC, abstractmethod
import tempfile
import requests
import gzip
import csv
import io

class Dataset(ABC):

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
		res = requests.get(self.url)
		res.raise_for_status()
		return res.content

	def refresh(self, cursor:sqlite3.Cursor) -> None:
		with gzip.open(io.BytesIO(self.request()), 'rt') as f:
			reader = csv.reader(f, delimiter='\t')
			next(reader) # skip header line
			for row in reader:
				self.insert(cursor, self.parse(row))

class TitleEpisode(Dataset):

	@property
	def url(self):
		return "https://datasets.imdbws.com/title.episode.tsv.gz"

	def parse(self, row:List[str]) -> list:
		return [
			int(row[0][2:])
			int(row[1][2:])
			int(row[2]) if row[2].isdigit() else None
			int(row[2]) if row[2].isdigit() else None
		]

	def insert(self, cursor:sqlite3.Cursor, parsed_row:list) -> None:
		cursor.execute('''
			INSERT OR IGNORE INTO ImdbDatasetTitleEpisode VALUES (?,?,?,?)
		''', parsed_row)