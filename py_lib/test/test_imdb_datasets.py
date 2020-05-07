import os
import time
import sqlite3
import unittest
import subprocess
import imdb_datasets
import test_imdb_datasets_mock

TEST_DB = './resources/imdb_datasets.db'

class TitleEpisodeTest(imdb_datasets.TitleEpisode):

	@property
	def url(self):
		return "http://localhost:{}/".format(test_imdb_datasets_mock.MOCK_PORT)

class TestImdbDatasets(unittest.TestCase):

	@classmethod
	def setUpClass(cls):
		cls.mock_process = subprocess.Popen(
			["python3", "test_imdb_datasets_mock.py"],
			stdout=subprocess.DEVNULL,
			stderr=subprocess.DEVNULL,
		)
		try:
			os.remove(TEST_DB)
		except FileNotFoundError:
			pass
		cls.conn = sqlite3.connect(TEST_DB)
		cls.cursor = cls.conn.cursor()
		cls.t = TitleEpisodeTest()
		time.sleep(2) # let server get good and ready

	@classmethod
	def tearDownClass(cls):
		try:
			os.remove(TEST_DB)
		except FileNotFoundError:
			pass
		cls.mock_process.terminate()

	def sqlUp(self):
		self.cursor.execute('''
			CREATE TABLE ImdbDatasetTitleEpisode (
				imdbid INTEGER PRIMARY KEY,
				seriesid INTEGER NOT NULL,
				season INTEGER,
				episode INTEGER
			);
		''')

	def sqlDown(self):
		self.cursor.execute('''
			DROP TABLE ImdbDatasetTitleEpisode;
		''')

	def refresh(self):
		self.t.refresh(self.cursor)

	def setUp(self):
		self.sqlUp()

	def tearDown(self):
		self.sqlDown()

	def test1(self):
		self.refresh()
		self.cursor.execute('SELECT imdbid, seriesid, season, episode FROM ImdbDatasetTitleEpisode WHERE imdbid=(?)', (54319,))
		t = self.cursor.fetchone()
		self.assertEqual(t, (54319, 52511, 1, 33))

	def test2(self):
		self.refresh()
		self.cursor.execute('SELECT imdbid, seriesid, season, episode FROM ImdbDatasetTitleEpisode WHERE imdbid=(?)', (5720052,))
		t = self.cursor.fetchone()
		self.assertEqual(t, (5720052, 1490123, None, None))