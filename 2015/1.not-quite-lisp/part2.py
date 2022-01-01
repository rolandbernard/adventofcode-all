
import sys

instr = list(1 if c == '(' else -1 for c in sys.stdin.read().strip())

sum = 0
res = 0
for i, d in enumerate(instr):
    sum += d
    if sum < 0:
        res = i + 1
        break

print('Result:', res)

