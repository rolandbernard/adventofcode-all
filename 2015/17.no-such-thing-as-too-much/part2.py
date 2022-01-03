
import sys

from math import inf
from functools import cache

cap = []

for line in sys.stdin:
    cap.append(int(line))

@cache
def best(left, used = 0):
    if left == 0:
        return 0
    elif left < 0 or used >= len(cap):
        return inf
    else:
        return min(1 + best(left - cap[used], used + 1), best(left, used + 1))

@cache
def comb(left, unused, used = 0):
    if left == 0:
        return 1
    elif left < 0 or unused == 0 or used >= len(cap):
        return 0
    else:
        return comb(left - cap[used], unused - 1, used + 1) + comb(left, unused, used + 1)

print('Result:', comb(150, best(150)))

