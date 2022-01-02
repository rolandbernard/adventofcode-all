
import sys

nav = [{'v': (0, 1), '^': (0, -1), '>': (1, 0), '<': (-1, 0)}[c] for c in sys.stdin.read().strip()]

pos = (0, 0)
visited = {pos}
for n in nav:
    pos = (pos[0] + n[0], pos[1] + n[1])
    visited.add(pos)

print('Result:', len(visited))

