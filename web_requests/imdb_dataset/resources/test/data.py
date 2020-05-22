import gzip
import csv
import re

FILES = [
	"title.ratings.tsv.gz",
	"title.episode.tsv.gz",
	"title.crew.tsv.gz",
	"title.basics.tsv.gz",
	"title.akas.tsv.gz",
	'name.basics.tsv.gz',
	"title.principals.tsv.gz",
]

# with gzip.open(FILES[6], mode='rt') as f:
# 	reader = csv.reader(f, delimiter='\t')
# 	for i in range(60):
# 		print(next(reader))

with gzip.open(FILES[6], mode='rt') as f:
	reader = csv.reader(f, delimiter='\t', quoting=csv.QUOTE_NONE)
	for row in reader:
		if not all([
			re.match(r'^tt[0-9]+$', row[0]),
			re.match(r'^(10)|([1-9])|([1-9]0)+$', row[1]),
			re.match(r'^nm[0-9]+$', row[2]),
			re.match(r'^([a-z]|_)+$', row[3]),
			re.match(r'^(\\N)|(.+)$', row[4]),
			re.match(r'^(\\N)|(\[".+"\])$', row[5]),
		]):
			print(row)

# with gzip.open(FILES[2], mode='rt') as f:
# 	reader = csv.reader(f, delimiter='\t')
# 	for row in reader:
# 		if ',' in row[2]:
# 			print(row)