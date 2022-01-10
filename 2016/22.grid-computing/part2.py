
import sys
import re
import numpy as np

nodes = {}

for line in sys.stdin:
    if line[0] == '/':
        col = re.split('\s+', line)
        pos = col[0].split('-')
        x = int(pos[1][1:])
        y = int(pos[2][1:])
        used = int(col[2][:-1])
        avail = int(col[3][:-1])
        nodes[(x, y)] = [used, avail]

width = 1 + max(x[0] for x in nodes.keys())
height = 1 + max(x[1] for x in nodes.keys())

state = np.zeros((width, height), dtype=np.int8)

free = (0, 0)

for i in range(width):
    for j in range(height):
        if nodes[(i, j)][0] == 0:
            free = (i, j)
        elif nodes[(i, j)][0] > 100:
            state[i, j] = 1

dist = np.full((width, height), -1)
dist[free] = 0
todo = [free]
for x, y in todo:
    if (x, y) == (width - 1, 0):
        break
    else:
        if x > 0 and state[x - 1, y] == 0 and dist[x - 1, y] == -1:
            dist[x - 1, y] = 1 + dist[x, y]
            todo.append((x - 1, y))
        if x < width - 1 and state[x + 1, y] == 0 and dist[x + 1, y] == -1:
            dist[x + 1, y] = 1 + dist[x, y]
            todo.append((x + 1, y))
        if y > 0 and state[x, y - 1] == 0 and dist[x, y - 1] == -1:
            dist[x, y - 1] = 1 + dist[x, y]
            todo.append((x, y - 1))
        if y < height - 1 and state[x, y + 1] == 0 and dist[x, y + 1] == -1:
            dist[x, y + 1] = 1 + dist[x, y]
            todo.append((x, y + 1))
            
if not state[:,0:2].any():
    print('Result:', dist[-1, 0] + (width - 2) * 5)
else:
    print('Result: ??')

