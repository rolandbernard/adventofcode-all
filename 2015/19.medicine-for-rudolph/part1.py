
import re
import sys

from scipy.ndimage import convolve

rawrec, med = (l.strip() for l in sys.stdin.read().split('\n\n'))
rec = [(l.split(' => ')[0], l.split(' => ')[1]) for l in rawrec.split('\n')]

pos = set()

for k, v in rec:
    for m in re.finditer(k, med):
        pos.add(med[:m.start()] + v + med[m.end():])

print('Result:', len(pos))

