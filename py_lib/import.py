from pathlib import Path
from typing import List
import mimetypes
import tempfile

def prepare(path: Path) -> List[Path]:
	guess = mimetypes.guess_type(path.name)
	if 'video' in guess[0]:
		return path