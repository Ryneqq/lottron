import json;
import matplotlib.pyplot as plt;
import numpy as np;

with open('data/plots/most_common.json', 'r') as file:
    data = file.read()

json = json.loads(data)

title = json['title']
data = json['data']

histogram = [];

for [x, y] in data:
    histogram += np.full(shape=int(y), fill_value=int(x)).tolist()

# TODO
# print(histogram)
xticks = np.arange(0,50,1)
bins = np.arange(0,50, 0.5)
plt.hist(histogram, rwidth=1, bins = bins)
plt.title(title)
plt.xticks(xticks)
plt.xlabel('Lotto number')
plt.ylabel('Number of occurrences')
plt.show()
