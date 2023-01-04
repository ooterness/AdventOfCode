# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
from opcodes import ALL_INSTRUCTIONS, store
import re

def read_input(input):
    prog = []
    for (n, line) in enumerate(input.splitlines()):
        if n == 0:
            iptr = int(line[4:])
        else:
            op = ALL_INSTRUCTIONS[line[0:4]]
            (a, b, c) = [int(x) for x in re.findall('[0-9]+', line)]
            prog.append((op, a, b, c))
    return (iptr, prog)

# Define some new psuedo-instructions.
def noop(a, b, c, reg):
    return reg
def modr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    div = (reg[b] % reg[a] == 0)    # Is RegA a factor of RegB?
    return store(reg[b] if div else 0, c, reg)

def optimize(iptr, prog):
    # The problem input has a nested loop that checks if a given
    # register divides evenly into another register.  We replace
    # the inner loop with new instructions but leave the rest.
    prog[3] = (modr, prog[3][1], prog[4][2], prog[3][3])
    prog[11] = (noop, 0, 0, 0)

def run(iptr, prog, reg):
    assert (0 <= iptr and iptr < len(reg))
    ctr = 0
    while 0 <= ctr and ctr < len(prog):
        (op, a, b, c) = prog[ctr]
        reg[iptr] = ctr
        reg = op(a, b, c, reg)
        ctr = reg[iptr] + 1
    return reg

def part1(iptr, prog):
    reg = run(iptr, prog, [0, 0, 0, 0, 0, 0])
    return reg[0]

def part2(iptr, prog):
    reg = run(iptr, prog, [1, 0, 0, 0, 0, 0])
    return reg[0]

TEST = \
'''
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=19, year=2018))
    optimize(*input)
    assert(part1(*test) == 6)
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input)}')
