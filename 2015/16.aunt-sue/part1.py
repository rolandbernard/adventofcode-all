
import sys

sues = []

for inst in sys.stdin:
    sue, *attr = (x.strip() for x in inst.split(':'))
    attr = [a.split(':') for a in ':'.join(attr).split(',')]
    sues.append((int(sue.split(' ')[1]), dict((a[0].strip(),int(a[1])) for a in attr)))

trace = {
    'children': 3,
    'cats': 7,
    'samoyeds': 2,
    'pomeranians': 3,
    'akitas': 0,
    'vizslas': 0,
    'goldfish': 5,
    'trees': 3,
    'cars': 2,
    'perfumes': 1,
}

def isMatch(sue):
    for k, v in sue[1].items():
        if trace[k] != v:
            return False
    return True

posible = [s for s in sues if isMatch(s)]

print('Result:', posible[0][0])

