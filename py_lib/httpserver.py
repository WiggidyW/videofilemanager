from flask import Flask, request
from imdbid import Imdbid
import util
from constants import Constants

FLASK_PORT = 5000
app = Flask(__name__)

def run(host:str=Constants.SERVER_IP, port:int=Constants.SERVER_PORT):
	app.run(host=host, port=port)

@app.route('/vfm/subtitles')
def subtitles():
	try:
		params = {}
		for key in request.args:
			params[key] = str(request.args[key])
		result = subtitles_dispatch(params)
		return str(result)
	except Exception as e:
		return str(e)

def _ost_fetch(params:dict):
	return ost_fetch(params)

SUBTITLES_OPS = {
	'ost-fetch': _ost_fetch,
}

def subtitles_dispatch(params:dict):
	if 'op' not in params:
		return 'error: no op is specified'
	elif params['op'] not in SUBTITLES_OPS:
		return 'error: op is not legal'
	else:
		return SUBTITLES_OPS[params['op']](params)

def ost_fetch(params:dict):
	try:
		ost_fetch_validate(params)
	except Exception as e:
		return str(e)
	imdbid = Imdbid(params['imdbid'])
	result = []
	for i in range(int(params['count'])):
		try:
			sub = imdbid.new_ost_subtitle(params['lang'])
			if not sub:
				return 'error: got {} new subtitles for tt{} with language {} - no additional subs were available'.format(i+1, imdbid.imdbid, params['lang'])
		except Exception as e:
			return 'error: got {} new subtitles for tt{} with language {} - encountered exception: {}'.format(i+1, imdbid.imdbid, params['lang'], e)
	return 'success: got {} new subtitles for tt{} with language {}'.format(params['count'], imdbid.imdbid, params['lang'])

def ost_fetch_validate(params:dict) -> None:
	if not params.get('imdbid'):
		raise Exception('error: imdbid parameter is missing')
	if not util.Imdbid.valid(params['imdbid']):
		raise Exception('error: imdbid parameter is not legal')
	if not params.get('lang'):
		raise Exception('error: lang parameter is missing')
	if len(params['lang']) != 3:
		raise Exception('error: lang parameter is not legal')
	if not params.get('count'):
		raise Exception('error: count parameter is missing')
	if not params['count'].isdigit():
		raise Exception('error: count parameter is not legal')