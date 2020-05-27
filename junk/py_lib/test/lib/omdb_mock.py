import flask
import json

mock = flask.Flask(__name__)

TEST_FILE_1 = './resources/omdb.tt00519784.json'
TEST_FILE_2 = './resources/omdb.tt05273028.json'
TEST_FILE_3 = './resources/omdb.tt07049682.json'
MOCK_PORT = 5000

@mock.route('/')
def server():
	imdbid = flask.request.args.get('i', '')
	if '0519784' in imdbid:
		with open(TEST_FILE_1, 'r') as f:
			return json.load(f)
	elif '5273028' in imdbid:
		with open(TEST_FILE_2, 'r') as f:
			return json.load(f)
	elif '7049682' in imdbid:
		with open(TEST_FILE_3, 'r') as f:
			return json.load(f)
	else:
		return None

if __name__ == '__main__':
	mock.run(host='0.0.0.0', port=MOCK_PORT)