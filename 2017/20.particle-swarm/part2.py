
import sys
import numpy as np

parts = np.array([[[int(x) for x in (v.strip('pva=<>') + ',0').split(',')] for v in l.strip().split(', ')] for l in sys.stdin])

def removeColliding(part):
    uniq, count = np.unique(part[:,0,:], return_counts=True, axis=0)
    if max(count) > 1:
        for x in uniq[np.where(count > 1)[0]]:
            part[(part[:,0,:] == x).all(axis=1),0,3] = 1

def canCollide(part, cols):
    rem = []
    for i, j in cols:
        if not part[i,0,3] and not part[j,0,3]:
            vel = np.sign(part[i,0,:] - part[j,0,:]) == np.sign(part[j,1,:] - part[i,1,:])
            acc = np.sign(part[i,0,:] - part[j,0,:]) == np.sign(part[j,2,:] - part[i,2,:])
            if (vel | acc).all():
                cols.difference_update(rem)
                return True
        rem.append((i, j))
    return False

cols = {(i, j) for i in range(len(parts)) for j in range(i + 1, len(parts))}
while canCollide(parts, cols):
    removeColliding(parts)
    parts[:,1,:] += parts[:,2,:]
    parts[:,0,:] += parts[:,1,:]

print('Result:', (parts[:,0,3] == 0).sum())

