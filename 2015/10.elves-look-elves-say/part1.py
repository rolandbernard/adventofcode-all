
import sys

string = sys.stdin.read().strip()

def expand(string):
    chars = []
    prev = string[0]
    count = 1
    for c in string[1:]:
        if c == prev:
            count += 1
        else:
            chars.append(str(count))
            chars.append(prev)
            prev = c
            count = 1
    chars.append(str(count))
    chars.append(prev)
    return ''.join(chars)

for _ in range(40):
    string = expand(string)

print('Result:', len(string))

