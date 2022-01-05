
import sys
import re

program = [re.split(',? ', x) for x in sys.stdin.read().strip().split('\n')]

def runProgram(prog):
    reg = { 'a': 1, 'b': 0 }
    pc = 0
    while pc >= 0 and pc < len(prog):
        match prog[pc]:
            case ['hlf', r]:
                reg[r] //= 2
            case ['tpl', r]:
                reg[r] *= 3
            case ['inc', r]:
                reg[r] += 1
            case ['jmp', off]:
                pc += int(off) - 1
            case ['jie', r, off]:
                if reg[r] % 2 == 0:
                    pc += int(off) - 1
            case ['jio', r, off]:
                if reg[r] == 1:
                    pc += int(off) - 1
            case instr:
                print('Invalid instruction', instr)
        pc += 1
    return reg

print('Result:', runProgram(program)['b'])

