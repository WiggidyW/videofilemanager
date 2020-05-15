from typing import List, IO, Union, Any
from pathlib import Path
from os import PathLike
import urllib.request
import tempfile
import zipfile
import util
import json
import io

def get_metadata(agent:str, imdbid:Union[int, str], lang:str) -> List[dict]:
	url = 'https://rest.opensubtitles.org/search/imdbid-{}/sublanguageid-{}/'.format(int(util.Imdbid.digits(imdbid, 0)), lang)
	res = urllib.request.urlopen(urllib.request.Request(url, headers={'User-Agent': agent}))
	return json.loads(res.read())

def download(metadata:dict, target:Path) -> None:
	res = urllib.request.urlopen(metadata['ZipDownloadLink'])
	extract(metadata, target, io.BytesIO(res.read()))

def extract(metadata:dict, target:Path, file:IO[bytes]) -> None:
	z = zipfile.Path(file)
	files = find_files(z)
	if not files:
		raise Exception('invalid subtitle')
	if len(files) == 1:
		write(target, files[0])
	else:
		for entry in files:
			if entry.name == metadata['SubFileName']:
				write(target, entry)
				return
		largest = (files[0], 0)
		for entry in files:
			if len(entry.read_bytes()) > largest[1]:
				largest = (entry, len(entry.read_bytes()))
		write(target, largest[0])

def write(target:'PathLike[Any]', file:Union[Path, zipfile.Path]) -> None:
	with open(target, 'wb') as f:
		f.write(file.read_bytes())

def find_files(path:Union[Path, zipfile.Path]) -> List[Union[Path, zipfile.Path]]:
	if path.is_file():
		return [path]
	files = []
	for entry in path.iterdir():
		if entry.is_file():
			files.append(entry)
		else:
			files.extend(find_files(entry))
	return files