
import sys

life = {(i, j, 0) for (i, l) in enumerate(sys.stdin) for (j, c) in enumerate(l.strip()) if c == '#'}

def adjacent(pos):
    if pos[0] == 0:
        yield (1, 2, pos[2] - 1)
    if pos[0] == 4:
        yield (3, 2, pos[2] - 1)
    if pos[1] == 0:
        yield (2, 1, pos[2] - 1)
    if pos[1] == 4:
        yield (2, 3, pos[2] - 1)
    if (pos[0], pos[1]) == (1, 2):
        for i in range(5):
            yield (0, i, pos[2] + 1)
    if (pos[0], pos[1]) == (3, 2):
        for i in range(5):
            yield (4, i, pos[2] + 1)
    if (pos[0], pos[1]) == (2, 1):
        for i in range(5):
            yield (i, 0, pos[2] + 1)
    if (pos[0], pos[1]) == (2, 3):
        for i in range(5):
            yield (i, 4, pos[2] + 1)
    for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
        if (
            pos[0] + dx >= 0 and pos[1] + dy >= 0
            and pos[0] + dx < 5 and pos[1] + dy < 5
            and (pos[0] + dx, pos[1] + dy) != (2, 2)
        ):
            yield (pos[0] + dx, pos[1] + dy, pos[2])

def lifeOneStep(life):
    to_check = set()
    for pos in life:
        to_check.update(adjacent(pos))
    new = set()
    for pos in to_check:
        count = sum(1 for a in adjacent(pos) if a in life)
        if count == 1 or (count == 2 and pos not in life):
            new.add(pos)
    return new

for _ in range(200):
    life = lifeOneStep(life)

print('Result:', len(life))

