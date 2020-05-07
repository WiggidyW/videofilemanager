from pathlib import Path
import json
import re

DIR = Path('/mnt/c/Users/user/Programming/.config/vfr_metadata')
REXP = re.compile(r'omdb\.(series\.)?tt[0-9]+\.json')

# ### PRINT ALL POSSIBLE KEYS ###
# keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for key in data.keys():
# 			if key not in keys:
# 				keys.append(key)
# print(keys)


# ### PRINT ALL OPTIONAL KEYS ###
# keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for key in data.keys():
# 			if key not in keys:
# 				keys.append(key)
# optional_keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for key in keys:
# 			if key not in data.keys():
# 				if key not in optional_keys:
# 					optional_keys.append(key)
# print(optional_keys)


# ### PRINT ALL N/A-ABLE KEYS ###
# keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for key in data.keys():
# 			if key not in keys:
# 				keys.append(key)
# na_keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for key in keys:
# 			if data.get(key, '') == 'N/A':
# 				if key not in na_keys:
# 					na_keys.append(key)
# print(na_keys)


# ### PRINT ALL KEY TYPES ###
# types = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for value in data.values():
# 			if type(value) not in types:
# 				types.append(type(value))
# print(types)


# ### PRINT ALL LISTS ###
# keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for k, v in data.items():
# 			if type(v) == list:
# 				if k not in keys:
# 					keys.append(k)
# print(keys)

### PRINT ALL DIGIT KEYS ###
# keys = []
# for entry in DIR.iterdir():
# 	if REXP.match(entry.name):
# 		with open(entry, 'r') as f:
# 			data = json.load(f)
# 		for k, v in data.items():
# 			if isinstance(v, str):
# 				if re.match(r'^[0-9]*$', v):
# 					if k not in keys:
# 						keys.append(k)
# print(keys)

class Foo:
	def foo(self):
		return "Foo"

class Bar:
	def display(self, f):
		print(f.foo())

class FooBar(Foo, Bar):
	def p(self):
		self.display(self)

fb = FooBar()
fb.p()