
import sys

step = int(sys.stdin.read())

list = [0]
pos = 0
for i in range(2017):
    pos = (pos + step) % len(list)
    list.insert(pos + 1, i + 1)
    pos += 1

print('Result:', list[(pos + 1) % len(list)])

