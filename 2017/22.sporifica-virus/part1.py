
import sys

map = [[c == '#' for c in l.strip()] for l in sys.stdin]
inf = set()
for i, r in enumerate(map):
    for j, v in enumerate(r):
        if v:
            inf.add((i - len(map) // 2, j - len(r) // 2))

infected = 0
pos = (0, 0)
dir = (-1, 0)
for _ in range(10000):
    if pos in inf:
        inf.remove(pos)
        dir = (dir[1], -dir[0])
    else:
        infected += 1
        inf.add(pos)
        dir = (-dir[1], dir[0])
    pos = (pos[0] + dir[0], pos[1] + dir[1])

print('Result:', infected)

