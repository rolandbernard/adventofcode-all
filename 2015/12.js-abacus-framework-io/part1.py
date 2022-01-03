
import sys
import json

obj = json.loads(sys.stdin.read().strip())

def sumObj(obj):
    if isinstance(obj, int):
        return obj
    elif isinstance(obj, list):
        return sum(sumObj(o) for o in obj)
    elif isinstance(obj, dict):
        return sum(sumObj(o) for o in obj.values())
    else:
        return 0

print('Result:', sumObj(obj))

