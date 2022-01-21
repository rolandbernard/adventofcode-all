
import sys

lengths = [int(x) for x in sys.stdin.read().strip().split(',')]

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
for length in lengths:
    rev(list, pos, pos + length - 1)
    pos += length + skip
    skip += 1

print('Result:', list[0] * list[1])

