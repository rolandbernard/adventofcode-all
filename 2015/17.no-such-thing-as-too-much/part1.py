
import sys

from functools import cache

cap = []

for line in sys.stdin:
    cap.append(int(line))

@cache
def comb(left, used = 0):
    if left == 0:
        return 1
    elif left < 0 or used >= len(cap):
        return 0
    else:
        return comb(left - cap[used], used + 1) + comb(left, used + 1)

print('Result:', comb(150))

