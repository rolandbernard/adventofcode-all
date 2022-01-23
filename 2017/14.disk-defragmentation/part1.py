
import sys

def rev(arr, i, j):
    while i < j:
        tmp = arr[i % len(arr)]
        arr[i % len(arr)] = arr[j % len(arr)]
        arr[j % len(arr)] = tmp
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

count = 0
for i in range(128):
    count += sum(1 if c == '1' else 0 for c in knot_hash(magic + '-' + str(i)))

print('Result:', count)

