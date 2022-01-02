
import re
import sys

diff = 0
for inst in sys.stdin:
    reduced = re.sub('\\\\\\\\|\\\\"|\\\\x..', '?', inst[1:-1])
    diff += len(inst) - len(reduced)

print('Result:', diff)

