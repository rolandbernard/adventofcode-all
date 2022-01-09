
import sys

from hashlib import md5

magic = sys.stdin.read().strip()

size = 4
open = {'b', 'c', 'd', 'e', 'f'}
todo = [(0, 0, '')]

for x, y, p in todo:
    if x == size - 1 and y == size - 1:
        result = p
    else:
        hash = md5((magic + p).encode('utf-8')).hexdigest()
        if hash[0] in open and y != 0:
            todo.append((x, y - 1, p + 'U'))
        if hash[1] in open and y != size - 1:
            todo.append((x, y + 1, p + 'D'))
        if hash[2] in open and x != 0:
            todo.append((x - 1, y, p + 'L'))
        if hash[3] in open and x != size - 1:
            todo.append((x + 1, y, p + 'R'))

print('Result:', len(result))

