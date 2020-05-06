from pathlib import Path
from typing import Union
import re

class Imdbid:

	@staticmethod
	def valid(arg: Union[str, int, Path]) -> bool:
		if isinstance(arg, str):
			return bool(re.fullmatch(r'^(tt)?0*[1-9][0-9]{0,7}$', arg))
		elif isinstance(arg, int):
			return 0 < arg < 100000000
		elif isinstance(arg, Path):
			for part in arg.parts:
				if re.fullmatch(r'^tt0*[1-9][0-9]{0,7}$', part):
					return True
		return False

	@staticmethod
	def digits(arg: Union[str, int, Path], pad: int) -> str:
		s = "0"
		if isinstance(arg, Path):
			for part in reversed(arg.parts):
				if re.fullmatch(r'^tt0*[1-9][0-9]{0,7}$', part):
					s = re.search(r'[1-9][0-9]{0,7}$', part).group()
					break
		elif Imdbid.valid(arg):
			if isinstance(arg, str):
				s = re.search(r'[1-9][0-9]{0,7}$', arg).group()
			elif isinstance(arg, int):
				s = arg
		return str(int(s)).zfill(pad)

	@staticmethod
	def full(arg: Union[str, int, Path], pad: int) -> str:
		return 'tt{}'.format(Imdbid.digits(arg, pad))