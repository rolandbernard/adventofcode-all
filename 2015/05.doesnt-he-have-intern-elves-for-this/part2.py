
import sys

def isNice(str):
    return (
        any(str[i-1:i+1] in str[i+1:] for i in range(1, len(str)))
        and any(str[i - 2] == str[i] for i in range(2, len(str)))
    )

nice = sum(1 for l in sys.stdin if isNice(l))

print('Result:', nice)

