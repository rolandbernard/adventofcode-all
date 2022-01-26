
import sys

cmds = sys.stdin.read().strip().split(',')

order = [chr(x) for x in range(ord('a'), ord('p') + 1)]
for cmd in cmds:
    match [cmd[0], cmd[1:].split('/')]:
        case ['s', [a]]:
            order = order[-int(a):] + order[:-int(a)]
        case ['x', [a, b]]:
            order[int(a)], order[int(b)] = order[int(b)], order[int(a)]
        case ['p', [a, b]]:
            order = [a if c == b else b if c == a else c for c in order]

print('Result:', ''.join(order))

