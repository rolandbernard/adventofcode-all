
import sys

graph = {}
for line in sys.stdin:
    frm, lst = line.strip().split(' <-> ')
    graph[frm] = lst.split(', ')

def counts(cur, vis = set()):
    if cur in vis:
        return 0
    else:
        vis.add(cur)
        return 1 + sum(counts(n, vis) for n in graph[cur])

print('Result:', counts('0'))

