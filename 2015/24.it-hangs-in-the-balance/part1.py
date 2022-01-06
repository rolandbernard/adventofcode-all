
import sys

from itertools import combinations
from functools import reduce

inf = 2**60
pack = set(int(x) for x in sys.stdin.read().strip().split('\n'))
weight = sum(pack) // 3

def possibleGroups(packs, weight):
    for i in range(1, len(packs) + 1):
        for g in combinations(packs, i):
            if sum(g) == weight:
                yield set(g)

def canGroup(packs, weight, groups):
    if groups == 0:
        return len(packs) == 0
    else:
        for g in possibleGroups(packs, weight):
            if canGroup(packs - g, weight, groups - 1):
                return True
        return False

best = (inf, inf)
for g in possibleGroups(pack, weight):
    if len(g) > best[0]:
        break
    else:
        qe = reduce(lambda a, b: a * b, g, 1)
        if qe < best[1] and canGroup(pack - g, weight, 2):
            best = (len(g), qe)

print('Result:', best[1])

