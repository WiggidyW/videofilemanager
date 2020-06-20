import gzip
import csv

files = [
    "title.principals.tsv.gz",
    "name.basics.tsv.gz",
    "title.akas.tsv.gz",
    "title.basics.tsv.gz",
    "title.crew.tsv.gz",
    "title.episode.tsv.gz",
    "title.ratings.tsv.gz",
]

# counter = 0

# for file in files:
# 	with gzip.open(file, 'rt') as f:
# 		reader = csv.reader(f, delimiter='\t', quoting=csv.QUOTE_NONE)
# 		print("---")
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print(next(reader))
# 		print("---")

with gzip.open(files[3], 'rt') as f:
	reader = csv.reader(f, delimiter='\t', quoting=csv.QUOTE_NONE)
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])
	print(next(reader)[1])