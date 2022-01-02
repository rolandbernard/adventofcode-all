
import sys

password = [ord(c) - ord('a') for c in sys.stdin.read().strip()]

def isValid(passwd):
    pairs = 0
    i = 1
    while i < len(passwd):
        if passwd[i - 1] == passwd[i]:
            pairs += 1
            i += 1
        i += 1
    return (
        pairs >= 2
        and all(c not in [8, 11, 14] for c in passwd)
        and any(
            passwd[i - 2] + 2 == passwd[i - 1] + 1 == passwd[i]
            for i in range(2, len(passwd))
        )
    )

def nextString(passwd):
    i = len(passwd) - 1
    passwd[i] += 1
    while i > 0 and passwd[i] == 26:
        passwd[i] = 0
        passwd[i - 1] += 1
        i -= 1
    if passwd[i] == 26:
        passwd[i] = 0


nextString(password)
while not isValid(password):
    nextString(password)

print('Result:', ''.join(chr(c + ord('a')) for c in password))

