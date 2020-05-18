import requests

# res = requests.get("http://localhost:1234/title.ratings.tsv.gz", headers={'Accept-Encoding': 'gzip'})
res = requests.get("https://datasets.imdbws.com/title.ratings.tsv.gz", headers={'Accept-Encoding': 'gzip'})
# req.headers['Accept-Encoding'] = 'gzip'

# res = requests.get(req)

print(res.headers)