
import sys

state = [int(v.strip().split(' ')[-1]) for v in sys.stdin]

count = 0
for _ in range(5000000):
    while state[0] % 4 != 0:
        state[0] = (state[0] * 16807) % 2147483647
    while state[1] % 8 != 0:
        state[1] = (state[1] * 48271) % 2147483647
    if state[0] & 0xffff == state[1] & 0xffff:
        count += 1
    state[0] = (state[0] * 16807) % 2147483647
    state[1] = (state[1] * 48271) % 2147483647

print('Result:', count)

