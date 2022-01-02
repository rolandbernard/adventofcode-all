
import sys

from hashlib import md5
from itertools import count

magic = sys.stdin.read().strip()

for i in count():
    if md5((magic + str(i)).encode('utf-8')).hexdigest()[:5] == 5*'0':
        break;

print('Result:', i)

