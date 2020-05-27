import flask

mock = flask.Flask(__name__)

TEST_FILE = '../resources/title.episode.tsv.gz'
MOCK_PORT = 5000

@mock.route('/')
def server():
	return flask.send_file(TEST_FILE, as_attachment=True)

if __name__ == '__main__':
	mock.run(host='0.0.0.0', port=MOCK_PORT)