
import sys

held = set()
progs = {}

for line in sys.stdin:
    slt = line.split(' -> ')
    name = slt[0].split(' ')[0]
    progs[name] = [0, int(slt[0].split(' ')[1].strip('()\n')), []]
    if len(slt) > 1:
        progs[name][2].extend(slt[1].strip().split(', '))
        held.update(slt[1].strip().split(', '))

def computeWeight(node):
    progs[node][0] = progs[node][1] + sum(computeWeight(n) for n in progs[node][2])
    return progs[node][0]

def wrongWeight(node, diff):
    cw = [progs[n][0] for n in progs[node][2]]
    com = max(set(cw), key=cw.count)
    for i, w in enumerate(cw):
        if w != com:
            return wrongWeight(progs[node][2][i], com - w)
    return progs[node][1] + diff

root = (progs.keys() - held).pop()
computeWeight(root)
print('Result:', wrongWeight(root, 0))

