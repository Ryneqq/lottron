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
plt.hist(histogram, rwidth=0.1, bins = np.arange(1, 49, 1))
plt.title(title)
plt.xlim(0, 49, 1)
plt.xlabel('Lotto number')
plt.ylabel('Number of occurrences')
plt.show()
