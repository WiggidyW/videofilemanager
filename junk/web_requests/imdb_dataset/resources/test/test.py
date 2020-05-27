import os
from flask import Flask, send_file
app = Flask(__name__)

FILES = [
	"name.basics.tsv.gz",
	"title.akas.tsv.gz",
	"title.basics.tsv.gz",
	"title.crew.tsv.gz",
	"title.episode.tsv.gz",
	"title.principals.tsv.gz",
	"title.ratings.tsv.gz",
]

@app.route("/<filename>")
def download(filename):
	if filename in FILES:
		return send_file(os.path.dirname(__file__) + filename, as_attachment=True)

if __name__ == '__main__':
	app.run(port=os.environ.get('PORT', 1234))