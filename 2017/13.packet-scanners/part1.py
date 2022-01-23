
import sys

wall = []
for line in sys.stdin:
    lay, ran = line.strip().split(': ')
    wall.append((int(lay), int(ran)))

cost = 0
for l, r in wall:
    if l % (2 * r - 2) == 0:
        cost += l * r

print('Result:', cost)

