import tempfile
import requests
import time
import gzip
import json
import csv

URL = 'https://datasets.imdbws.com/title.episode.tsv.gz'

def refresh(path):
	data = {'timestamp': time.time()}
	tmp = tempfile.NamedTemporaryFile()
	res = requests.get(URL)
	res.raise_for_status()
	tmp.write(res.content)
	tmp.seek(0)
	with gzip.open(tmp, 'rt') as f:
		tsv = csv.reader(f, delimiter='\t')
		for row in tsv:
			season = row[2] if row[2].isdigit() else None
			episode = row[3] if row[3].isdigit() else None
			data.update({row[0]: [row[1], season, episode]})
	with open(path, 'w') as f:
		json.dump(data, f, separators=(',', ':'))

refresh('data.json')