from pathlib import Path
from abc import ABC, abstractmethod
from typing import List, Union
import util

class Film(ABC):

	@property
	@abstractmethod
	def subtitles(self) -> List[Path]:
		raise NotImplementedError

	@property
	@abstractmethod
	def video(self) -> Path:
		raise NotImplementedError

	@property
	@abstractmethod
	def plex_target(self) -> Path:
		raise NotImplementedError

	@property
	@abstractmethod
	def imdbid(self) -> Union[str, int]:
		raise NotImplementedError

def refresh(film: Film) -> None:
	for entry in film.plex_target.iterdir():
		if film.plex_target.name in entry.name:
			entry.unlink() # panicks if entry is a directory!
	with open(film.plex_target.with_suffix('srt'), 'w') as f:
		f.write(util.Imdbid.full(film.imdbid, 7))
	film.video.link_to(film.plex_target.with_suffix(film.video.suffix))
	for i, subtitle in enumerate(film.subtitles):
		subtitle.link_to(film.plex_target.with_suffix('{}{}'.format(i, subtitle.suffix)))