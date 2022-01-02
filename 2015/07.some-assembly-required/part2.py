
import sys

values = {}
connections = []

for inst in sys.stdin:
    splt = inst.split('->')
    wire = splt[1].strip()
    connections.append(splt[0].strip().split(' ') + [wire])
    for n in connections[-1]:
        if n.isdigit():
            values[n] = int(n)

def propagate(vals, connections):
    values = vals.copy()
    while 'a' not in values:
        rem = []
        for c in connections:
            match c:
                case [a, 'AND', b, wire] if a in values and b in values:
                    values[wire] = values[a] & values[b]
                case [a, 'OR', b, wire] if a in values and b in values:
                    values[wire] = values[a] | values[b]
                case [a, 'LSHIFT', b, wire] if a in values and b in values:
                    values[wire] = values[a] << values[b]
                case [a, 'RSHIFT', b, wire] if a in values and b in values:
                    values[wire] = values[a] >> values[b]
                case ['NOT', a, wire] if a in values:
                    values[wire] = ~values[a]
                case [a, wire] if a in values:
                    values[wire] = values[a]
                case _:
                    rem.append(c)
        connections = rem
    return values

values['b'] = propagate(values, connections)['a']
connections = [c for c in connections if c[-1] != 'b']
result = propagate(values, connections)['a']

print('Result:', result & 0xffff)

