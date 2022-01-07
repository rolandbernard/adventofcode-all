
import sys

from hashlib import md5
from itertools import count

magic = sys.stdin.read().strip()
passwd = ['_'] * 8

for i in count():
    hash = md5((magic + str(i)).encode('utf-8')).hexdigest()
    if hash[:5] == 5*'0':
        idx = int(hash[5], 16)
        if idx < 8 and passwd[idx] == '_':
            passwd[idx] = hash[6]
            if '_' not in passwd:
                break

print('Result:', ''.join(passwd))

