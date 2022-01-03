
import sys

from collections import defaultdict

dist = defaultdict(dict)
ids = defaultdict(lambda: len(ids))

for inst in sys.stdin:
    match inst.strip().split(' '):
        case [f, 'would', sign, val, 'happiness', 'units', 'by', 'sitting', 'next', 'to', t]:
            dist[ids[f]][ids[t[:-1]]] = int(val) if sign == 'gain' else -int(val)

def maxDistance(cur, visited, dist, mem = dict()):
    if visited == (1 << len(dist)) - 1:
        return dist[cur][0] + dist[0][cur]
    else:
        if (cur, visited) not in mem:
            mem[(cur, visited)] = max(
                d + dist[t][cur] + maxDistance(t, visited | (1 << t), dist, mem) for t, d in dist[cur].items()
                if (visited & (1 << t)) == 0
            )
        return mem[(cur, visited)]

print('Result:', maxDistance(0, 1, dist))

