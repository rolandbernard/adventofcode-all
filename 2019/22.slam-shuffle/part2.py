
import sys

N = 119315717514047
M = 101741582076661

b = 0
a = 1

lines = [l.strip() for l in sys.stdin]
lines.reverse()
for l in lines:
    match l.split(' '):
        case ['deal', 'into', 'new', 'stack']:
            a = (-a) % N
            b = (-1 - b) % N
        case ['cut', n]:
            b = (b + int(n)) % N
        case ['deal', 'with', 'increment', n]:
            inv = pow(int(n), -1, N)
            b = (b * inv) % N
            a = (a * inv) % N

aa = pow(a, M, N)
bb = (b * (aa - 1) * pow((a - 1) % N, -1, N)) % N

print('Result:', (aa * 2020 + bb) % N)

