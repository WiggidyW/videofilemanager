import sqlite3

conn = sqlite3.connect("test/imdb_datasets.db")
c = conn.cursor()

def exec(path):
	with open(path, 'r') as f:
		c.executescript(f.read())

exec("imdb_datasets_temp_DOWN.sql")
exec("imdb_datasets_DOWN.sql")
exec("imdb_datasets_UP.sql")
exec("imdb_datasets_temp_UP.sql")

conn.commit()