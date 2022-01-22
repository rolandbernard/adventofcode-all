
import sys

graph = {}
for line in sys.stdin:
    frm, lst = line.strip().split(' <-> ')
    graph[frm] = lst.split(', ')

def counts(cur, vis):
    if cur in vis:
        return 0
    else:
        vis.add(cur)
        return 1 + sum(counts(n, vis) for n in graph[cur])

vis = set()
found = 0
for g in graph.keys():
    if g not in vis:
        found += 1
        counts(g, vis)

print('Result:', found)

