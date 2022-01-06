
import sys
import re

match = re.match('.+ (\d+), .+ (\d+)\.', sys.stdin.read())
r, c = int(match[1]), int(match[2])

d = r + c - 2
p = (1 + d) * d // 2 + c

v = (20151125 * pow(252533, p - 1, 33554393)) % 33554393

print('Result:', v)

