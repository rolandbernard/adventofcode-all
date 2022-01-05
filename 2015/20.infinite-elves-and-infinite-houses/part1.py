
import sys
import numpy as np

num = int(sys.stdin.read())
houses = np.full((num // 10 + 1,), 10)

for i in range(2, len(houses)):
    houses[i::i] += 10 * i

loc = np.where(houses > num)

print('Result:', loc[0][0])

