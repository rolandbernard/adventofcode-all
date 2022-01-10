
import sys
import re

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

count = 0
for pi, i in nodes.items():
    for pj, j in nodes.items():
        if pi != pj:
            if i[0] != 0 and i[0] < j[1]:
                count += 1

print('Result:', count)

