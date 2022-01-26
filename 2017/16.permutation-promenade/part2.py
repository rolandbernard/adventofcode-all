
import sys
import numpy as np

cmds = sys.stdin.read().strip().split(',')

M = 1000000000
N = 16
perm = np.identity(N, dtype=np.int32)
part = np.identity(N, dtype=np.int32)
for cmd in cmds:
    match [cmd[0], cmd[1:].split('/')]:
        case ['s', [a]]:
            perm = np.roll(perm, int(a), axis=0)
        case ['x', [a, b]]:
            perm[[int(a), int(b)],:] = perm[[int(b), int(a)],:]
        case ['p', [a, b]]:
            acs = ord(a) - ord('a')
            bcs = ord(b) - ord('a')
            part[:,[acs, bcs]] = part[:,[bcs, acs]]
            
perm = np.linalg.matrix_power(perm, M)
part = np.linalg.matrix_power(part, M)

order = perm @ part @ np.arange(N)
print('Result:', ''.join(chr(ord('a') + x) for x in order))

