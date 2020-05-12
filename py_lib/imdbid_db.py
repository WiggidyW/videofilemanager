import sqlite3
from abc import ABC, abstractmethod
from typing import Callable, Any, Tuple

class Db:
	def __init__(self, table: str, id_: Tuple[str, Any], cursor: sqlite3.Cursor, refresh:Callable[[sqlite3.Cursor], Any]):
		self.table = table
		self.id_key = id_[0]
		self.id_val = id_[1]
		self.cursor = cursor
		self._refresh = refresh
		
		if not self.exists():
			self.refresh()

	def __getitem__(self, key):
		self.cursor.execute('SELECT {} FROM {} WHERE {}=(?)'.format(
			key, self.table, self.id_key), (self.id_val,))
		return self.cursor.fetchone()[0]

	def exists(self) -> bool:
		self.cursor.execute('SELECT EXISTS(SELECT 1 FROM {} WHERE {}=(?))'.format(
			self.table, self.id_key), (self.id_val,))
		return bool(self.cursor.fetchone()[0])

	def refresh(self):
		self._refresh(self.cursor)