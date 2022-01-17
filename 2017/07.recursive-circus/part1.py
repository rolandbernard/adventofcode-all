
import sys

all = set()
held = set()

for line in sys.stdin:
    slt = line.split(' -> ')
    all.add(slt[0].split(' ')[0])
    if len(slt) > 1:
        held.update(slt[1].strip().split(', '))

print('Result:', (all - held).pop())

