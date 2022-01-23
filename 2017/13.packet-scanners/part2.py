
import sys

from math import lcm
from itertools import product

wall = []
for line in sys.stdin:
    lay, ran = line.strip().split(': ')
    stp = 2 * int(ran) - 2
    wall.append((stp, set(i for i in range(stp) if (i + int(lay)) % stp != 0)))
wall = sorted(wall)

comb = (1, {0})
for w in wall:
    mul = lcm(w[0], comb[0])
    comb = (mul, set(
        comb[0] * m + i
        for m, i in product(range(mul // comb[0]), comb[1])
        if (comb[0] * m + i) % w[0] in w[1]
    ))
    if len(comb[1]) == 1:
        best = comb

print('Result:', min(comb[1]))

