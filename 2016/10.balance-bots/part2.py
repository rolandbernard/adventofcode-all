
import sys

from collections import defaultdict

connections = {}
outputs = defaultdict(lambda: set())
bots = defaultdict(lambda: set())
dests = {'output': outputs, 'bot': bots}

for line in sys.stdin:
    match line.strip().split(' '):
        case ['bot', b, 'gives', 'low', 'to', k0, n0, 'and', 'high', 'to', k1, n1]:
            connections[int(b)] = ((k0, int(n0)), (k1, int(n1)))
        case ['value', v, 'goes', 'to', 'bot', b]:
            bots[int(b)].add(int(v))

while len(bots) != 0:
    for bot, chips in bots.copy().items():
        if len(chips) >= 2:
            dst = connections[bot]
            dests[dst[0][0]][dst[0][1]].add(min(chips))
            dests[dst[1][0]][dst[1][1]].add(max(chips))
            chips.remove(min(chips))
            chips.remove(max(chips))
        elif len(chips) == 0:
            del bots[bot]

print('Result:', outputs[0].pop() * outputs[1].pop() * outputs[2].pop())

