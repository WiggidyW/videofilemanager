import sqlite3

with open("imdb_datasets_DOWN.sql", 'r') as f:
    cmd = f.read()

conn = sqlite3.connect("test/imdb_datasets.db")

c = conn.cursor()
c.executescript(cmd)

conn.commit()