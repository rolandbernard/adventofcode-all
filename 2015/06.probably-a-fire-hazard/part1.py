
import re
import sys
import numpy as np

lights = np.zeros((1000, 1000), dtype=bool)
for inst in sys.stdin:
    match re.split(' |,', inst):
        case ['toggle', x0, y0, 'through', x1, y1]:
            lights[int(x0):int(x1)+1, int(y0):int(y1)+1] = (lights[int(x0):int(x1)+1, int(y0):int(y1)+1] == False)
        case ['turn', 'off', x0, y0, 'through', x1, y1]:
            lights[int(x0):int(x1)+1, int(y0):int(y1)+1] = False
        case ['turn', 'on', x0, y0, 'through', x1, y1]:
            lights[int(x0):int(x1)+1, int(y0):int(y1)+1] = True

print('Result:', lights.sum())

