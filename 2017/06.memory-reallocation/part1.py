
import sys
import numpy as np

mem = np.fromstring(sys.stdin.read(), dtype=np.int32, sep='\t')

steps = 0
seen = set()
while tuple(mem) not in seen:
    seen.add(tuple(mem))
    idx = np.argmax(mem)
    tmp = mem[idx]
    mem[idx] = 0
    mem += tmp // len(mem)
    mem[idx + 1:idx + 1 + (tmp % len(mem))] += 1
    if len(mem) < idx + 1 + (tmp % len(mem)):
        mem[:idx + 1 + (tmp % len(mem)) - len(mem)] += 1
    steps += 1

print('Result:', steps)

