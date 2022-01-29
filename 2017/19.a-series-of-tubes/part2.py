
import sys

map = [[c for c in l.strip('\n')] for l in sys.stdin]

dir = (0, 1)
pos = (map[0].index('|'), 0)

def dirs(dir):
    yield dir
    if dir[0] == 0:
        yield (1, 0)
        yield (-1, 0)
    else:
        yield (0, 1)
        yield (0, -1)

def next_dir():
    for d in dirs(dir):
        mov = (pos[0] + d[0], pos[1] + d[1])
        if min(mov) >= 0 and mov[1] < len(map) and mov[0] < len(map[mov[1]]) and not map[mov[1]][mov[0]].isspace():
            return mov, d
    return pos, None

steps = 1
while True:
    mov, dir = next_dir()
    if pos != mov:
        pos = mov
        steps += 1
    else:
        break

print('Result:', steps)

