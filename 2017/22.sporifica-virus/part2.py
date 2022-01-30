
import sys
import numpy as np

from numba import jit

map = np.array([[1 if c == '#' else 0 for c in l.strip()] for l in sys.stdin])
map = np.pad(map, 500)

@jit
def countInfections(map):
    infected = 0
    pos = (map.shape[0] // 2, map.shape[1] // 2)
    dir = (-1, 0)
    for _ in range(10000000):
        if map[pos] == 1:
            map[pos] = 2
            dir = (dir[1], -dir[0])
        elif map[pos] == 2:
            map[pos] = 0
            dir = (-dir[0], -dir[1])
        elif map[pos] == 3:
            infected += 1
            map[pos] = 1
        else:
            map[pos] = 3
            dir = (-dir[1], dir[0])
        pos = (pos[0] + dir[0], pos[1] + dir[1])
    return infected

print('Result:', countInfections(map))

