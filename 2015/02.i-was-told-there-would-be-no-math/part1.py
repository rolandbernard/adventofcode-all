
import sys

packs = [[int(x) for x in l.split('x')] for l in sys.stdin]
area = [[p[i] * p[(i + 1) % 3] for i in range(3)] for p in packs]
paper = sum(2*sum(p) + min(p) for p in area)

print('Result:', paper)

