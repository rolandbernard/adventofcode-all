
import sys

packs = [[int(x) for x in l.split('x')] for l in sys.stdin]
measu = [([2 * (p[i] + p[(i + 1) % 3]) for i in range(3)], p[0]*p[1]*p[2]) for p in packs]
ribbon = sum(min(p) + v for p, v in measu)

print('Result:', ribbon)

