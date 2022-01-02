
import re
import sys

from collections import defaultdict

dist = defaultdict(dict)
ids = defaultdict(lambda: len(ids))

for inst in sys.stdin:
    spl = re.split(' = | to ', inst)
    f, t, d = spl[0].strip(), spl[1].strip(), int(spl[2].strip())
    dist[ids['']][ids[t]] = 0
    dist[ids['']][ids[f]] = 0
    dist[ids[f]][ids[t]] = d
    dist[ids[t]][ids[f]] = d

def minDistance(cur, visited, dist, mem = dict()):
    if visited == (1 << len(dist)) - 1:
        return 0
    else:
        if (cur, visited) not in mem:
            mem[(cur, visited)] = min(
                d + minDistance(t, visited | (1 << t), dist, mem) for t, d in dist[cur].items()
                if (visited & (1 << t)) == 0
            )
        return mem[(cur, visited)]

print('Result:', minDistance(ids[''], 1 << ids[''], dist))

