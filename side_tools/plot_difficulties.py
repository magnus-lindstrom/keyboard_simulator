import matplotlib.pyplot as plt
from matplotlib import cm
from modules import *

base_image = '../results/images/blank_layout.png'

coords, shift = get_blank_keyboard_coords()
with open('../src/qwerty_swe', 'r') as f:
    tmp = f.read()

tmp = tmp.splitlines()
diff_dict = {}
for line in tmp:
    if line[0] != '#':
        diff_dict[line.split()[0]] = float(line.split()[4])
max_diff = max(diff_dict.values())
min_diff = min(diff_dict.values()) - 1
norm_diff_dict = {key: (val - min_diff)/(max_diff - min_diff) for (key, val) in
                  diff_dict.items()}
colormap = cm.get_cmap('Reds', 100)
fig = plt.figure(figsize=(15, 10))
img = plt.imread(base_image)
imgplot = plt.imshow(img)
for key in diff_dict.keys():
    plt.text(coords[key][0], coords[key][1], diff_dict[key], color='k',
             fontweight='extra bold',
             backgroundcolor=colormap(norm_diff_dict[key]),
             fontsize=14, horizontalalignment='center',
             verticalalignment='center')

cur_axes = plt.gca()
cur_axes.axes.get_xaxis().set_visible(False)
cur_axes.axes.get_yaxis().set_visible(False)
fig.patch.set_visible(False)
plt.show()

