import matplotlib.pyplot as plt
import numpy as np

result_dir = '../results/'
result_files = ['statistics']
result_files = [result_dir + file for file in result_files]
colors = ['b', 'r']

fig = plt.figure(figsize=(12, 8))
ax = plt.gca()
min_x = 100
max_x = 0
for i_file, file in enumerate(result_files):
    losses = []
    counts = []
    with open(file, 'r') as f:
        for line in f:
            if line[0] != '#':
                losses.append(float(line[:-1].split()[0]))
                counts.append(float(line[:-1].split()[1]))
    losses = np.array(losses)
    counts = np.array(counts)
    # counts = counts / np.sum(counts)
    if np.amin(losses) < min_x:
        min_x = np.amin(losses)
    if np.amax(losses) > max_x:
        max_x = np.amax(losses)
    ax.fill_between(losses, counts)
    ax.plot(losses, counts, '.k', markersize=3)
    # ax.plot(losses, counts, '-', color=colors[i_file])
loss_ticks = np.linspace(min_x, max_x, 10)
ax.set_xticks(loss_ticks)
plt.savefig('loss_distribution.png')
plt.show()
