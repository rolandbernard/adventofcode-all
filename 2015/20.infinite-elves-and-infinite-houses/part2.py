
import sys
import numpy as np

num = int(sys.stdin.read())
houses = np.zeros((num // 11 + 1,), dtype=np.int32)

for i in range(1, len(houses)):
    houses[i:51*i + 1:i] += 11 * i

loc = np.where(houses > num)

print('Result:', loc[0][0])

