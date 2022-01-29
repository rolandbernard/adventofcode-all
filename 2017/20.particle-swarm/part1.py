
import sys
import numpy as np

parts = np.array([[[int(x) for x in v.strip('pva=<>').split(',')] for v in l.strip().split(', ')] for l in sys.stdin])

print('Result:', np.abs(parts[:,2,:]).sum(1).argmin())

