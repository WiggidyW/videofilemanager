from pathlib import Path
import unittest
import util

validate = util.Imdbid.valid
digits = util.Imdbid.digits
full = util.Imdbid.full

class TestImdbid(unittest.TestCase):

	def setUp(self):
		self.before = ""
		self.after = ""

	def prepare(self, before, after):
		self.before = before
		self.after = after

	def activate(self, func, *args, **kwargs):
		self.assertEqual(self.after, func(self.before, *args, **kwargs))

	def test_Full_Str_01(self):
		self.prepare("tt12345678", "tt12345678")
		self.activate(full, 0)

	def test_Full_Str_02(self):
		self.prepare("tt00008", "tt8")
		self.activate(full, 0)

	def test_Full_Str_03(self):
		self.prepare("tt1", "tt0001")
		self.activate(full, 4)

	def test_Full_Str_04(self):
		self.prepare("tt01", "tt0001")
		self.activate(full, 4)

	def test_Full_Str_05(self):
		self.prepare("012345", "tt12345")
		self.activate(full, 5)

	def testSad_Full_Str_01(self):
		self.prepare("FooBar", "tt0")
		self.activate(full, 0)

	def testSad_Full_Str_02(self):
		self.prepare("FooBar", "tt0")
		self.activate(full, 1)

	def testSad_Full_Str_03(self):
		self.prepare("FooBar", "tt00000000")
		self.activate(full, 8)

	def test_Full_Int_01(self):
		self.prepare(1, "tt0001")
		self.activate(full, 4)

	def test_Full_Int_02(self):
		self.prepare(99900, "tt00099900")
		self.activate(full, 8)

	def testSad_Full_Int_01(self):
		self.prepare(-150, "tt0000")
		self.activate(full, 4)

	def testSad_Full_Int_02(self):
		self.prepare(100000000, "tt0")
		self.activate(full, 1)

	def test_Full_Path_01(self):
		self.prepare(Path('/foo/bar/tt01950/foobar'), "tt01950")
		self.activate(full, 5)

	def test_Full_Path_02(self):
		self.prepare(Path('/foo/bar/tt01950/foobar'), "tt1950")
		self.activate(full, 4)

	def test_Full_Path_03(self):
		self.prepare(Path('/foo/bar/tt01950/foobar'), "tt1950")
		self.activate(full, 3)

	def test_Full_Path_04(self):
		self.prepare(Path('/foo/bar/foobar/tt01950'), "tt1950")
		self.activate(full, 0)

	def test_Full_Path_05(self):
		self.prepare(Path('/tt01950/foo/bar/foobar'), "tt00001950")
		self.activate(full, 8)

	def test_Full_Path_06(self):
		self.prepare(Path('tt33'), "tt0000033")
		self.activate(full, 7)

	# It should go off of the last one!
	def test_Full_Path_07(self):
		self.prepare(Path('/tt33/tt9090'), "tt0009090")
		self.activate(full, 7)

	# It should go off of the last one!
	def test_Full_Path_08(self):
		self.prepare(Path('/tt9999999/tt33/tt1337'), "tt1337")
		self.activate(full, 1)

	def testSad_Full_Path_01(self):
		self.prepare(Path('/'), "tt00")
		self.activate(full, 2)

	def testSad_Full_Path_02(self):
		self.prepare(Path('/9999999/33/1337'), "tt00")
		self.activate(full, 2)

	def testSad_Full_Path_03(self):
		self.prepare(Path('Foo/Bar'), "tt00")
		self.activate(full, 2)