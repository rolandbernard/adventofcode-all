
import sys

reindeer = []

for inst in sys.stdin:
    match inst.strip().split(' '):
        case [name, 'can', 'fly', speed, 'km/s', 'for', dur, 'seconds,', 'but', 'then', 'must', 'rest', 'for', pause, 'seconds.']:
            reindeer.append([0, 0, int(dur), (int(speed), int(dur)), (0, int(pause))])

for _ in range(2503):
    for r in reindeer:
        r[2] -= 1
        r[1] += r[3][0]
        if r[2] == 0:
            r[3], r[4] = r[4], r[3]
            r[2] = r[3][1]
    lead = max(r[1] for r in reindeer)
    for r in reindeer:
        if r[1] == lead:
            r[0] += 1

print('Result:', max(r[0] for r in reindeer))

