
import sys

num = int(sys.stdin.read())

def housePresents(num):
    res = 1
    prime = 2
    while num != 1:
        this = 1
        while num % prime == 0:
            this *= prime
            num //= prime
        if this != 1:
            res *= (this * prime - 1) / (prime - 1)
        prime += 1
    return 10 * res

res = 1
while housePresents(res) < num:
    if res % 10000 == 0:
        print(res, housePresents(res))
    res += 1

print('Result:', res)

