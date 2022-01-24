
import sys

state = [int(v.strip().split(' ')[-1]) for v in sys.stdin]

count = 0
last = 0
for i in range(40000000):
    if state[0] & 0xffff == state[1] & 0xffff:
        last = i
        count += 1
    state[0] = (state[0] * 16807) % 2147483647
    state[1] = (state[1] * 48271) % 2147483647

print('Result:', count)

