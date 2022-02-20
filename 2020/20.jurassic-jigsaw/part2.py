
import sys
import numpy as np

tiles = [
    [
        int(t[0].split(' ')[1][:-1]),
        np.array([[1 if c == '#' else 0 for c in r] for r in t[1:]])
    ] for t in (t.split('\n') for t in sys.stdin.read().strip().split('\n\n'))
]
edges = [
    set([
        min(tuple(t[0,:]), tuple(t[0,::-1])),
        min(tuple(t[-1,:]), tuple(t[-1,::-1])),
        min(tuple(t[:,0]), tuple(t[::-1,0])), 
        min(tuple(t[:,-1]), tuple(t[::-1,-1])),
    ])
    for _, t in tiles
]
neight = [
    set(j for j in range(len(tiles)) if i != j and len(edges[i] & edges[j]) != 0)
    for i in range(len(tiles))
]
corners = set(i for i, n in enumerate(neight) if len(n) == 2)
sides = set(i for i, n in enumerate(neight) if len(n) == 3)
center = set(i for i, n in enumerate(neight) if len(n) == 4)
size = int(len(tiles)**0.5 + 0.5)
tile_size = tiles[0][1].shape[0] - 2

assembly = [[-1 for _ in range(size)] for _ in range(size)]

for i in range(size):
    for j in range(size):
        if (i == 0 or i == size - 1) and (j == 0 or j == size - 1):
            consider = corners
        elif i == 0 or j == 0 or i == size - 1 or j == size - 1:
            consider = sides
        else:
            consider = center
        possible = consider.copy()
        if i != 0:
            possible.intersection_update(neight[assembly[i - 1][j]])
        if j != 0:
            possible.intersection_update(neight[assembly[i][j - 1]])
        setting = possible.pop()
        assembly[i][j] = setting
        consider.remove(setting)

def rotations(tile):
    for _ in range(4):
        tile = np.rot90(tile)
        yield tile
        tile = np.flip(tile, 0)
        yield tile
        tile = np.flip(tile, 1)
        yield tile
        tile = np.flip(tile, 0)
        yield tile
        tile = np.flip(tile, 1)

for i in range(size):
    for j in range(size):
        for g in rotations(tiles[assembly[i][j]][1]):
            if i != 0 and min(tuple(g[0,:]), tuple(g[0,::-1])) not in edges[assembly[i - 1][j]]:
                continue
            if j != 0 and min(tuple(g[:,0]), tuple(g[::-1,0])) not in edges[assembly[i][j - 1]]:
                continue
            if i != size - 1 and min(tuple(g[-1,:]), tuple(g[-1,::-1])) not in edges[assembly[i + 1][j]]:
                continue
            if j != size - 1 and min(tuple(g[:,-1]), tuple(g[::-1,-1])) not in edges[assembly[i][j + 1]]:
                continue
            tiles[assembly[i][j]][1] = g
            break

image = np.zeros((size * tile_size, size * tile_size), dtype=tiles[0][1].dtype)
for i in range(size):
    for j in range(size):
        image[tile_size * i:tile_size * (i + 1), tile_size * j:tile_size * (j + 1)] = tiles[assembly[i][j]][1][1:-1,1:-1]

pattern = np.array([
    [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1,0],
    [1,0,0,0,0,1,1,0,0,0,0,1,1,0,0,0,0,1,1,1],
    [0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,1,0,0,0],
])

for r in rotations(image):
    image = r
    found = False
    for i in range(image.shape[0] - pattern.shape[0]):
        for j in range(image.shape[1] - pattern.shape[1]):
            if (image[i:i+pattern.shape[0],j:j+pattern.shape[1]] * pattern).sum() == pattern.sum():
                image[i:i+pattern.shape[0],j:j+pattern.shape[1]][pattern == 1] = 0
                found = True
    if found:
        break

print(image.sum())

