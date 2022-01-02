
import sys

nav = [{'v': (0, 1), '^': (0, -1), '>': (1, 0), '<': (-1, 0)}[c] for c in sys.stdin.read().strip()]

pos = [(0, 0), (0, 0)]
visited = {pos[0]}
for i, n in enumerate(nav):
    pos[i%2] = (pos[i%2][0] + n[0], pos[i%2][1] + n[1])
    visited.add(pos[i%2])

print('Result:', len(visited))

