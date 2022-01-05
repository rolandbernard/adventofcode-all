
import sys

num = int(sys.stdin.read())

def housePresents(num):
    res = 0
    div = max(num // 50, 1)
    while div <= num:
        if num % div == 0:
            res += div
        div += 1
    return 11 * res

res = 1
while housePresents(res) < num:
    if res % 10000 == 0:
        print(res, housePresents(res))
    res += 1

print('Result:', res)

