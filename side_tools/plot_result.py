import matplotlib.pyplot as plt
from modules import *

image_dir = '../results/images/'
base_image = image_dir + 'blank_layout.png'

coords, shifts = get_blank_keyboard_coords()
spc_keys = get_spc_char_symbols()
with open('../results/layout_result', 'r') as f:
    tmp = f.read()
tmp = tmp.splitlines()
letter_layout = {e.split(' ')[1]: e.split(' ')[0] for e in tmp}
fig = plt.figure(figsize=(15, 10))
img = plt.imread(base_image)
imgplot = plt.imshow(img)
print(letter_layout)
print(coords)
for key in coords.keys():
    if key in letter_layout.keys():
        plt.text(coords[key][0] + shifts[0][0], coords[key][1] + shifts[0][1],
                 letter_layout[key],
                 fontsize=20, horizontalalignment='center',
                 verticalalignment='center')
cur_axes = plt.gca()
cur_axes.axes.get_xaxis().set_visible(False)
cur_axes.axes.get_yaxis().set_visible(False)
plt.savefig(image_dir + 'optimal_layout_plot.png')
plt.show()

