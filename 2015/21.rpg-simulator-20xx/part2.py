
import sys

from math import inf

bos = [int(x.split(':')[1]) for x in sys.stdin.read().strip().split('\n')]

weapons = [
    (8, 4, 0),
    (10, 5, 0),
    (25, 6, 0),
    (40, 7, 0),
    (74, 8, 0),
]
armors = [
    (13,  0, 1),
    (31, 0, 2),
    (53, 0, 3),
    (75, 0, 4),
    (102, 0, 5),
]
rings = [
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
]

def canWin(dam, arm):
    boss_damage = max(1, dam - bos[2])
    play_damage = max(1, bos[1] - arm)
    to_kill = (bos[0] + boss_damage - 1) // boss_damage
    return (to_kill - 1) * play_damage < 100

def maxCost():
    best = 0
    for w in weapons:
        for a in ((0, 0, 0), *armors):
            for r0 in ((0, 0, 0), *rings):
                for r1 in ((0, 0, 0), *rings):
                    if r0 == (0, 0, 0) or r0 != r1:
                        if not canWin(w[1] + a[1] + r0[1] + r1[1], w[2] + a[2] + r0[2] + r1[2]):
                            cost = w[0] + a[0] + r0[0] + r1[0]
                            if best < cost:
                                best = cost
    return best

print('Result:', maxCost())

