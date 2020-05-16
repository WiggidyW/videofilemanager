from load_config import env_load
from multiprocessing import Process

def main():
	env_load()
	import fswatcher
	import httpserver
	
	p1 = Process(target=fswatcher.watch)
	p1.start()

	p2 = Process(target=httpserver.run)
	p2.start()

	p2.join()

if __name__ == '__main__':
	main()