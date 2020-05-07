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

	def refresh(self) -> None:
		for entry in self.plex_target.iterdir():
			if self.plex_target.name in entry.name:
				entry.unlink() # panicks if entry is a directory!
		with open(self.plex_target.with_suffix('srt'), 'w') as f:
			f.write(util.Imdbid.full(self.imdbid, 7))
		self.video.link_to(self.plex_target.with_suffix(self.video.suffix))
		for i, subtitle in enumerate(self.subtitles):
			subtitle.link_to(self.plex_target.with_suffix('{}{}'.format(i, subtitle.suffix)))