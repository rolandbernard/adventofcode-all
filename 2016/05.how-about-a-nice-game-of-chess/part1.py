
import sys

from hashlib import md5
from itertools import count

magic = sys.stdin.read().strip()
passwd = ''

for i in count():
    hash = md5((magic + str(i)).encode('utf-8')).hexdigest()
    if hash[:5] == 5*'0':
        passwd += hash[5]
        if len(passwd) == 8:
            break

print('Result:', passwd)

