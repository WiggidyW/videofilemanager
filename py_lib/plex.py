from pathlib import Path
from typing import List, Union, Optional
import util
import os

def refresh(subtitles: List[Path], video: Optional[Path], target: Path, imdbid: Union[str, int]):
	if target.parent.exists():
		for entry in target.parent.iterdir():
			if target.name in entry.name:
				entry.unlink()
	else:
		target.parent.mkdir(parents=True)
	with open(target.with_name(target.name + '.nfo'), 'w') as f:
		f.write(util.Imdbid.full(imdbid, 7))
	if video:
		os.link(video, target.with_name(target.name + video.suffix))
	for i, subtitle in enumerate(subtitles):
		os.link(subtitle, target.with_name(target.name + '.{}{}'.format(i, subtitle.suffix)))