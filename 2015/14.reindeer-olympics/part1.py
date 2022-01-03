
import sys

reindeer = []

for inst in sys.stdin:
    match inst.strip().split(' '):
        case [name, 'can', 'fly', speed, 'km/s', 'for', dur, 'seconds,', 'but', 'then', 'must', 'rest', 'for', pause, 'seconds.']:
            reindeer.append((int(speed), int(dur), int(pause)))

def travelDistance(deer, time):
    return (
        deer[0] * deer[1] * (time // (deer[1] + deer[2]))
        + deer[0] * min(deer[1], time % (deer[1] + deer[2]))
    )

result = max(travelDistance(d, 2503) for d in reindeer)

print('Result:', result)

