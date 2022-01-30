
import sys
import numpy as np

raw_rules = [[np.array([[c == '#' for c in x] for x in v.split('/')]) for v in l.strip().split(' => ')] for l in sys.stdin]

def mat_id(mat):
    powers = 2**np.arange(mat.shape[0] * mat.shape[1]).reshape(mat.shape)
    return (powers * mat).sum()

def rots(mat):
    for _ in range(4):
        for _ in range(2):
            yield mat_id(mat)
            mat = np.flip(mat, axis=0)
            yield mat_id(mat)
            mat = np.flip(mat, axis=1)
        mat = np.rot90(mat)
        yield mat_id(mat)

def rule_id(mat):
    can = mat_id(mat)
    for m in rots(mat):
        if m < can:
            can = m
    return (mat.shape, can)

rules = {}
for r in raw_rules:
    id = rule_id(r[0])
    rules[id] = r[1]

def enhance(img):
    if len(img) % 2 == 0:
        new = np.zeros((img.shape[0] * 3 // 2, img.shape[1] * 3 // 2), dtype=np.bool8)
        for i in range(img.shape[0] // 2):
            for j in range(img.shape[1] // 2):
                new[3*i:3*i+3,3*j:3*j+3] = rules[rule_id(img[2*i:2*i+2,2*j:2*j+2])]
    else:
        new = np.zeros((img.shape[0] * 4 // 3, img.shape[1] * 4 // 3), dtype=np.bool8)
        for i in range(img.shape[0] // 3):
            for j in range(img.shape[1] // 3):
                new[4*i:4*i+4,4*j:4*j+4] = rules[rule_id(img[3*i:3*i+3,3*j:3*j+3])]
    return new

img = np.array([
    [False, True, False],
    [False, False, True],
    [True, True, True],
])
for _ in range(5):
    img = enhance(img)

print('Result:', img.sum())

