from abc import ABC, abstractmethod
from typing import Union
import time
import util

class Params(ABC):

	@property
	@abstractmethod
	def imdbid(self) -> Union[str, int]
		raise NotImplementedError

	@property
	@abstractmethod
	def plot(self) -> str:
		raise NotImplementedError

	@property
	@abstractmethod
	def apikey(self) -> str:
		raise NotImplementedError

class Omdb(ABC):

	@property
	@abstractmethod 
	def url(self) -> str:
		raise NotImplementedError

	@abstractmethod 
	def parse(self, data:dict) -> list:
		raise NotImplementedError

	@abstractmethod
	def insert(self, cursor:sqlite3.Cursor, parsed_data:list) -> None:
		raise NotImplementedError

	def request(self, params:Params) -> bytes:
		res = requests.get(self.url, params={
			'apikey': params.apikey,
			'plot': params.plot,
			'r': 'json',
			'i': util.Imdbid.full(params.imdbid, 8),
		})
		res.raise_for_status()
		return res.content

	def refresh(self, cursor:sqlite3.Cursor, params:Params) -> None:
		data = json.loads(self.request(params))
		assert(data.get('Response', 'False') != 'False')
		self.insert(cursor, self.parse(data))

class ByImdbid(Omdb):

	@property
	def url(self):
		return "https://www.omdbapi.com/"

	def parse(self, data:dict) -> list:
		return [
			int(data['imdbID'][2:]),
			True if data['Response'] == 'True' else False,
			time.time(),
			int(data['Year']),
			data['Title'],
			data['Type'],
			str(data['Ratings']) if data.get('Ratings') else None,
			float(data['imdbRating']) if data.get('imdbRating', 'N/A') != 'N/A' else None,
			int(data['totalSeasons']) if data.get('totalSeasons', 'N/A') != 'N/A' else None,
			int(data['imdbVotes'].replace(',', '')) if data.get('imdbVotes', 'N/A') != 'N/A' else None,
			int(data['Metascore']) if data.get('Metascore', 'N/A') != 'N/A' else None,
			int(data['seriesID'][2:]) if data.get('seriesID', 'N/A') != 'N/A' else None,
			int(data['Episode']) if data.get('Episode', 'N/A') != 'N/A' else None,
			int(data['Season']) if data.get('Season', 'N/A') != 'N/A' else None,
			data['Production'] if data.get('Production', 'N/A') != 'N/A' else None,
			data['BoxOffice'] if data.get('BoxOffice', 'N/A') != 'N/A' else None,
			data['Language'] if data.get('Language', 'N/A') != 'N/A' else None,
			data['Released'] if data.get('Released', 'N/A') != 'N/A' else None,
			data['Director'] if data.get('Director', 'N/A') != 'N/A' else None,
			data['Runtime'] if data.get('Runtime', 'N/A') != 'N/A' else None,
			data['Country'] if data.get('Country', 'N/A') != 'N/A' else None,
			data['Website'] if data.get('Website', 'N/A') != 'N/A' else None,
			data['Writer'] if data.get('Writer', 'N/A') != 'N/A' else None,
			data['Actors'] if data.get('Actors', 'N/A') != 'N/A' else None,
			data['Awards'] if data.get('Awards', 'N/A') != 'N/A' else None,
			data['Poster'] if data.get('Poster', 'N/A') != 'N/A' else None,
			data['Rated'] if data.get('Rated', 'N/A') != 'N/A' else None,
			data['Genre'] if data.get('Genre', 'N/A') != 'N/A' else None,
			data['Plot'] if data.get('Plot', 'N/A') != 'N/A' else None,
			data['DVD'] if data.get('DVD', 'N/A') != 'N/A' else None,
		]

	def insert(self, cursor:sqlite3.Cursor, parsed_data:list) -> None:
		cursor.execute('''
			INSERT OR REPLACE INTO Omdb VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
		''', parsed_data)