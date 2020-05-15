import urllib.request

URL = "https://www.omdbapi.com/?apikey=d99251a8&r=json&plot=full&i=tt00088170"

res = urllib.request.urlopen(urllib.request.Request(URL))

print(res)
print(res.geturl())
print(res.info())
print(res.getcode())
print(res.read())