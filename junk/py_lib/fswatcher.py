from watchdog import events, observers #type:ignore
from constants import Constants
from imdbid import Imdbid
from pathlib import Path
import util

class Watcher(events.FileSystemEventHandler):

	def log(self, s):
		with open(Constants.WATCHER_LOG, 'a') as f:
			f.write(s + '\n')

	def plex_refresh(self, path):
		imdbid = Imdbid(util.Imdbid.digits(Path(path), 0))
		try:
			imdbid.plex_refresh()
		except Exception as e:
			self.log('Plex link error for [{}]: [{}]'.format(imdbid.imdbid, e))

	def on_moved(self, event):
		if not event.is_directory:
			if '/{}/'.format(Constants.SUBTITLES_DIR) in event.src_path or '/{}/'.format(Constants.VIDEO_DIR) in event.src_path:
				self.plex_refresh(event.src_path)
			if '/{}/'.format(Constants.SUBTITLES_DIR) in event.dest_path or '/{}/'.format(Constants.VIDEO_DIR) in event.dest_path:
				self.plex_refresh(event.dest_path)

	def on_created(self, event):
		if not event.is_directory:
			if '/{}/'.format(Constants.SUBTITLES_DIR) in event.src_path or '/{}/'.format(Constants.VIDEO_DIR) in event.src_path:
				self.plex_refresh(event.src_path)

	def on_deleted(self, event):
		if not event.is_directory:
			if '/{}/'.format(Constants.SUBTITLES_DIR) in event.src_path or '/{}/'.format(Constants.VIDEO_DIR) in event.src_path:
				self.plex_refresh(event.src_path)

def watch():
	observer = observers.Observer()
	observer.schedule(Watcher(), str(Constants.MASTER_PATH), recursive=True)
	observer.start()
	observer.join()