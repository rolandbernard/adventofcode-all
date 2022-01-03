
import sys

ingr = []

for inst in sys.stdin:
    match inst.strip().split(' '):
        case [name, 'capacity', x0, 'durability', x1, 'flavor', x2, 'texture', x3, 'calories', x4]:
            ingr.append((int(x0[:-1]), int(x1[:-1]), int(x2[:-1]), int(x3[:-1]), int(x4)))

def maxScore(i, vals, ingr):
    if i == len(ingr) - 1:
        vals[i] = 100 - sum(vals[:i])
        return (
            max(0, sum(vals[j] * ing[0] for j, ing in enumerate(ingr)))
            * max(0, sum(vals[j] * ing[1] for j, ing in enumerate(ingr)))
            * max(0, sum(vals[j] * ing[2] for j, ing in enumerate(ingr)))
            * max(0, sum(vals[j] * ing[3] for j, ing in enumerate(ingr)))
        )
    else:
        best = 0
        for a in range(100 - sum(vals[:i])):
            vals[i] = a
            best = max(best, maxScore(i + 1, vals, ingr))
        return best

print('Result:', maxScore(0, [0] * len(ingr), ingr))

