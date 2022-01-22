
import sys

moves = sys.stdin.read().strip().split(',')

def steps(pos):
    return abs(pos[0]) + max(0, abs(pos[1]) - abs(pos[0])) // 2

best = 0
pos = [0, 0]
for m in moves:
    match m:
        case 'n':
            pos[1] -= 2
        case 's':
            pos[1] += 2
        case 'ne':
            pos[0] += 1
            pos[1] -= 1
        case 'se':
            pos[0] += 1
            pos[1] += 1
        case 'nw':
            pos[0] -= 1
            pos[1] -= 1
        case 'sw':
            pos[0] -= 1
            pos[1] += 1
    best = max(best, steps(pos))

print('Result:', best)

