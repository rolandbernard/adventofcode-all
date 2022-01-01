
import sys

def isNice(str):
    return (
        sum(1 for c in str if c in 'aeiou') >= 3
        and any(str[i - 1] == str[i] for i in range(1, len(str)))
        and all(not s in str for s in ['ab', 'cd', 'pq', 'xy'])
    )

nice = sum(1 for l in sys.stdin if isNice(l))

print('Result:', nice)

