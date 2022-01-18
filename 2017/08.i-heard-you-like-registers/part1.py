
import sys

from collections import defaultdict

regs = defaultdict(lambda: 0)

for line in sys.stdin:
    slt = line.strip().split(' ')
    val, left, right = (regs[x] if x.isalpha() else int(x) for x in (slt[2], slt[4], slt[6]))
    if eval(str(left) + slt[5] + str(right)):
        if slt[1] == 'inc':
            regs[slt[0]] += val
        else:
            regs[slt[0]] -= val

print('Result:', max(regs.values()))

