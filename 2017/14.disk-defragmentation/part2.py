
import sys
import numpy as np

def rev(arr, i, j):
    while i < j:
        v = i % len(arr)
        u = j % len(arr)
        arr[v], arr[u] = arr[u], arr[v]
        i += 1
        j -= 1

def knot_hash(text):
    lengths = [ord(x) for x in text.strip()] + [17, 31, 73, 47, 23]
    list = [i for i in range(256)]
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
        res += '{:08b}'.format(v)
    return res

magic = sys.stdin.read().strip()

mem = np.array([
    [1 if c == '1' else 0 for c in knot_hash(magic + '-' + str(i))]
    for i in range(128)
])

def removeAll(x, y):
    if x >= 0 and x < 128 and y >= 0 and y < 128 and mem[x, y]:
        mem[x, y] = 0
        for dx, dy in [(-1, 0), (1, 0), (0, -1), (0, 1)]:
            removeAll(x + dx, y + dy)

count = 0
while mem.sum() != 0:
    count += 1
    non = mem.nonzero()
    removeAll(non[0][0], non[1][0])

print('Result:', count)

