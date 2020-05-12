import load_config
from imdbid import Imdbid
import subtitle
import json
from constants import Constants

if __name__ == '__main__':
	import load_config
	load_config.env_load()

	print(Constants.OST_LANG)

	subtitle.Subtitle.ost_new('tt00519777')

	all = Imdbid.all()

	for imdbid in all:
		if imdbid.subtitles:
			for sub in imdbid.subtitles:
				print(sub.metadata['imdbID'])