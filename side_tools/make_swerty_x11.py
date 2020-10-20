import matplotlib.pyplot as plt
from modules import *

layout_name = 'qwerty_swe_layout'
image_dir = '../results/images/'
x11_dir = '../results/x11/'
base_image = 'blank_layout.png'

coords, shifts = get_blank_keyboard_coords()
spc_letters = get_altgr_letters()
nr_row_let = get_number_row_letters()
unused_keys = get_unused_keys()

with open('../src/{}'.format(layout_name), 'r') as f:
    tmp = f.read()
tmp = tmp.splitlines()
letter_layout = {}
for e in tmp:
    if e.split(' ')[0] not in ['\'', ',', '.']:
        letter_layout[e.split(' ')[1]] = (e.split(' ')[0], e.split(' ')[0].upper())
    elif e.split(' ')[0] == '\'':
        letter_layout[e.split(' ')[1]] = (e.split(' ')[0], '\"')
    elif e.split(' ')[0] == ',':
        letter_layout[e.split(' ')[1]] = (e.split(' ')[0], '<')
    elif e.split(' ')[0] == '.':
        letter_layout[e.split(' ')[1]] = (e.split(' ')[0], '>')

all_letters = assemble_keyboard(letter_layout, spc_letters, nr_row_let, unused_keys)

save_x11_file(all_letters, x11_dir, layout_name)

fig = plt.figure(figsize=(15, 10))
img = plt.imread(image_dir + base_image)
imgplot = plt.imshow(img)

for key in all_letters.keys():
    for i in range(4):
        plt.text(coords[key][0] + shifts[i][0], coords[key][1] + shifts[i][1],
                 all_letters[key][i],
                 fontsize=18, horizontalalignment='center',
                 verticalalignment='center')

cur_axes = plt.gca()
cur_axes.axes.get_xaxis().set_visible(False)
cur_axes.axes.get_yaxis().set_visible(False)

plt.savefig(image_dir + layout_name)
plt.show()

