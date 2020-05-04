import json

with open('data.json', 'r') as f:
	data = json.load(f)

i = 0
for val in data:
	print(val)
	i += 1
	if i > 30:
		break