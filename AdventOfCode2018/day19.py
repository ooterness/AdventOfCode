# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from opcodes import Program, modr, noop

# Modify the program in-place to run in a reasonable time period.
def optimize(prog):
    # The problem input has a nested loop that checks if a given
    # register divides evenly into another register.  We replace
    # the inner loop with new instructions but leave the rest.
    prog[3] = (modr, prog[3][1], prog[4][2], prog[3][3])
    prog[11] = (noop, 0, 0, 0)

def part1(prog):
    reg = prog.run([0, 0, 0, 0, 0, 0])
    return reg[0]

def part2(prog):
    reg = prog.run([1, 0, 0, 0, 0, 0])
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
    test = Program(TEST.strip())
    input = Program(get_data(day=19, year=2018))
    optimize(input.prog)
    assert(part1(test) == 6)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
