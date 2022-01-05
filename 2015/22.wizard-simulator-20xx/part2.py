
import sys

from math import inf
from functools import cache

bos = [int(x.split(':')[1]) for x in sys.stdin.read().strip().split('\n')]

@cache
def minCost(turn = True, hp = 50, mp = 500, ef = (0, 0, 0), bhp = bos[0]):
    if turn:
        hp -= 1
        if hp <= 0:
            return inf
    if ef[0] > 0:
        arm = 7
        ef = (ef[0] - 1, ef[1], ef[2])
    else:
        arm = 0
    if ef[1] > 0:
        bhp -= 3
        ef = (ef[0], ef[1] - 1, ef[2])
    if ef[2] > 0:
        mp += 101
        ef = (ef[0], ef[1], ef[2] - 1)
    if hp <= 0:
        return inf
    elif bhp <= 0:
        return 0
    elif turn:
        best = inf
        if mp >= 53:
            best = min(best, 53 + minCost(not turn, hp, mp - 53, ef, bhp - 4))
        if mp >= 73:
            best = min(best, 73 + minCost(not turn, hp + 2, mp - 73, ef, bhp - 2))
        if mp >= 113 and ef[0] == 0:
            best = min(best, 113 + minCost(not turn, hp, mp - 113, (6, ef[1], ef[2]), bhp))
        if mp >= 173 and ef[1] == 0:
            best = min(best, 173 + minCost(not turn, hp, mp - 173, (ef[0], 6, ef[2]), bhp))
        if mp >= 229 and ef[2] == 0:
            best = min(best, 229 + minCost(not turn, hp, mp - 229, (ef[0], ef[1], 5), bhp))
        return best
    else:
        return minCost(not turn, hp - max(1, bos[1] - arm), mp, ef, bhp)

print('Result:', minCost())

