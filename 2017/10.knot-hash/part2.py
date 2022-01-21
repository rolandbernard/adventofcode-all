
import sys

lengths = [ord(x) for x in sys.stdin.read().strip()] + [17, 31, 73, 47, 23]

list = [i for i in range(256)]

def rev(arr, i, j):
    while i < j:
        tmp = arr[i % len(arr)]
        arr[i % len(arr)] = arr[j % len(arr)]
        arr[j % len(arr)] = tmp
        i += 1
        j -= 1

pos = 0
skip = 0
for _ in range(64):
    for length in lengths:
        rev(list, pos, pos + length - 1)
        pos += length + skip
        skip += 1

res = ''
for i in range(16):
    v = 0
    for j in range(16):
        v ^= list[16 * i + j]
    res += '{:02x}'.format(v)

print('Result:', res)

