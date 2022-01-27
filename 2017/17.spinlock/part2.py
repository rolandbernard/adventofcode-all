
import sys

step = int(sys.stdin.read())

zero = 0
pos = 0
for i in range(50000000):
    pos = (pos + step) % (i + 1) + 1
    if pos == 1:
        zero = i + 1

print('Result:', zero)

