import os
import time
import sqlite3
import unittest
import subprocess
import omdb
from lib import omdb_mock

TEST_DB = './resources/omdb.db'

with open('../../schema/Omdb_UP.sql', 'r') as f:
	SCHEMA_UP = f.read()
with open('../../schema/Omdb_DOWN.sql', 'r') as f:
	SCHEMA_DOWN = f.read()

class ByImdbidTest(omdb.ByImdbid):

	@property
	def url(self):
		return "http://localhost:{}/".format(omdb_mock.MOCK_PORT)

class TestByImdbid(unittest.TestCase):

	@classmethod
	def setUpClass(cls):
		cls.mock_process = subprocess.Popen(
			["python3", "lib/omdb_mock.py"],
			stdout=subprocess.DEVNULL,
			stderr=subprocess.DEVNULL,
		)
		try:
			os.remove(TEST_DB)
		except FileNotFoundError:
			pass
		cls.conn = sqlite3.connect(TEST_DB)
		cls.cursor = cls.conn.cursor()
		cls.t = ByImdbidTest()
		time.sleep(2) # let server get good and ready

	@classmethod
	def tearDownClass(cls):
		try:
			os.remove(TEST_DB)
		except FileNotFoundError:
			pass
		cls.mock_process.terminate()

	def sqlUp(self):
		self.cursor.execute(SCHEMA_UP)

	def sqlDown(self):
		self.cursor.execute(SCHEMA_DOWN)

	def refresh(self, imdbid):
		self.t.refresh(self.cursor, imdbid, '', '')

	def setUp(self):
		self.sqlUp()

	def tearDown(self):
		self.sqlDown()

	def test1(self):
		self.refresh(519784)
		self.cursor.execute('SELECT * FROM Omdb WHERE imdbID=(?)', (519784,))
		t = self.cursor.fetchone()
		self.assertEqual((t[0], t[1], t[3], t[4], t[12], t[13]), (519784, True, 2006, 'Scar', 15, 2))

	def test2(self):
		self.refresh(5273028)
		self.cursor.execute('SELECT * FROM Omdb WHERE imdbID=(?)', (5273028,))
		t = self.cursor.fetchone()
		self.assertEqual(t[4], 'Summer Festival Time')

	def test3(self):
		self.refresh(7049682)
		self.cursor.execute('SELECT * FROM Omdb WHERE imdbID=(?)', (7049682,))
		t = self.cursor.fetchone()
		self.assertEqual(t[4], 'Watchmen')