
import re
import sys

diff = 0
for inst in sys.stdin:
    expanded = re.sub('\\\\|"', '??', inst.strip())
    diff += len(expanded) + 2 - len(inst.strip())

print('Result:', diff)

