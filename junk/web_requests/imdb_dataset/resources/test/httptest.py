import requests

res = requests.get('https://datasets.imdbws.com/title.ratings.tsv.gz')

print(res.headers)
# print('\n')
# print(res.content)