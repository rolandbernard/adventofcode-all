
import sys

floor = sum(1 if c == '(' else -1 for c in sys.stdin.read().strip())

print('Result:', floor)

