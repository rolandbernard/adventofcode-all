
import sys
import re

from hashlib import md5
from itertools import count

magic = sys.stdin.read().strip()

def triplet(hash):
    for i in range(2, len(hash)):
        if hash[i] == hash[i - 1] == hash[i - 2]:
            return hash[i]
    return None

def fivelets(hash):
    chars = set()
    for i in range(4, len(hash)):
        if hash[i] == hash[i - 1] == hash[i - 2] == hash[i - 3] == hash[i - 4]:
            chars.add(hash[i])
    return chars

result = 0
search = []

for i in count():
    hash = md5((magic + str(i)).encode('utf-8')).hexdigest()
    chars = fivelets(hash)
    keep = []
    for k, v in search:
        if k in chars:
            result += 1
            if result == 64:
                print('Result:', v)
                exit()
        elif i < v + 1000:
            keep.append((k, v))
    search = keep
    trip = triplet(hash)
    if trip is not None:
        search.append((trip, i))


