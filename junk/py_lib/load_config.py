import yaml
import os

def env_load():
	with open('config.yml', 'r') as f:
		data = yaml.load(f, Loader=yaml.Loader)
	for k, v in data.items():
		os.environ[k] = v