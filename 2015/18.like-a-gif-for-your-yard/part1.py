
import sys
import numpy as np

from scipy.ndimage import convolve

life = np.array([[1 if c == '#' else 0 for c in l.strip()] for l in sys.stdin])
kern = np.array([[1, 1, 1], [1, 0, 1], [1, 1, 1]])

def lifeOneStep(life):
    neighbors = convolve(life, kern, mode='constant')
    return (life & (neighbors == 2)) | (neighbors == 3)

for _ in range(100):
    life = lifeOneStep(life)

print('Result:', life.sum())

