
import sys
import numpy as np

from scipy.ndimage import convolve

life = np.array([[1 if c == '#' else 0 for c in l.strip()] for l in sys.stdin])
kern = np.array([[0, 1, 0], [1, 0, 1], [0, 1, 0]])
rating = (2**np.arange(life.shape[0] * life.shape[1])).reshape(life.shape)

def lifeOneStep(life):
    neighbors = convolve(life, kern, mode='constant')
    return (~life & (neighbors == 2)) | (neighbors == 1)

seen = {(rating * life).sum()}
while True:
    life = lifeOneStep(life)
    val = (rating * life).sum()
    if val in seen:
        break
    seen.add(val)

print('Result:', (rating * life).sum())

