import gzip
import csv
import sys

csv.field_size_limit(sys.maxsize)

FILES = [
	"name.basics.tsv.gz",
	"title.akas.tsv.gz",
	"title.basics.tsv.gz",
	"title.crew.tsv.gz",
	"title.episode.tsv.gz",
	"title.principals.tsv.gz",
	"title.ratings.tsv.gz",
]

nullable = []
for file in FILES:
	with gzip.open(file, mode='rt') as f:
		reader = csv.reader(f, delimiter='\t')
		column_names = next(reader)
		for row in reader:
			for i, col in enumerate(row):
				if col in ('\\N', ''):
					if '{} - {}'.format(file, column_names[i]) not in nullable:
						nullable.append('{} - {}'.format(file, column_names[i]))

print(nullable)