# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
import re

def read_input(input):
    numbers = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    lines = iter(input.splitlines())
    test = []           # List of test triplets (before, instruction, after)
    prog = []           # List of program instructions (opcode a b c)
    # First read the program examples.
    while True:
        try:
            line = next(lines)
            if len(line) == 0:
                continue    # Ignore blank lines
            elif line.startswith('Before'):
                x = numbers(line)
                y = numbers(next(lines))
                z = numbers(next(lines))
                test.append((x, y, z))
            else:           # Anything else is a line from the program
                prog.append(numbers(line))
        except StopIteration:
            break
    return (test, prog)

# Store value x in register c. (Used in almost every instruction.)
def store(x, c, reg):
    if c >= 4: raise Exception('Invalid register index.')
    return [x if c == n else reg[n] for n in range(4)]

# Define each of the instructions by name:
def addr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] + reg[b], c, reg)
def addi(a, b, c, reg):
    if a >= 4: return None
    return store(reg[a] + b, c, reg)
def mulr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] * reg[b], c, reg)
def muli(a, b, c, reg):
    if a >= 4: return None
    return store(reg[a] * b, c, reg)
def banr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] & reg[b], c, reg)
def bani(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] & b, c, reg)
def borr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] | reg[b], c, reg)
def bori(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(reg[a] | b, c, reg)
def setr(a, b, c, reg):
    if a >= 4: return None
    return store(reg[a], c, reg)
def seti(a, b, c, reg):
    return store(a, c, reg)
def gtir(a, b, c, reg):
    if b >= 4: return None
    return store(int(a > reg[b]), c, reg)
def gtri(a, b, c, reg):
    if a >= 4: return None
    return store(int(reg[a] > b), c, reg)
def gtrr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(int(reg[a] > reg[b]), c, reg)
def eqir(a, b, c, reg):
    if b >= 4: return None
    return store(int(a == reg[b]), c, reg)
def eqri(a, b, c, reg):
    if a >= 4: return None
    return store(int(reg[a] == b), c, reg)
def eqrr(a, b, c, reg):
    if a >= 4 or b >= 4: return None
    return store(int(reg[a] == reg[b]), c, reg)

ALL_INSTRUCTIONS = [addr, addi, mulr, muli, banr, bani, borr, bori,
                    setr, seti, gtir, gtri, gtrr, eqir, eqri, eqrr]

'''Count potentially-matching opcodes in a test.'''
def count_instr(before, opcode, after):
    (op, a, b, c) = opcode
    test = lambda fn: fn(a, b, c, before) == after
    return sum([test(fn) for fn in ALL_INSTRUCTIONS])

'''Identify which opcode is which function, returning a dictionary.'''
def identify_instr(test):
    None #???

'''How many test instructions could be more than three opcodes?'''
def part1(test, prog):
    return sum([count_instr(*t) >= 3 for t in test])

'''Work out which opcode is which, then run program.'''
def part2(test, prog):
    map = identify_instr(test)
    reg = [0, 0, 0, 0]
    for (op, a, b, c) in prog:
        reg = map[op](a, b, c, reg)
    return reg[0]

TEST = \
'''
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=16, year=2018))
    assert(part1(*test) == 1)
    print(f'Part 1: {part1(*input)}')
    #print(f'Part 2: {part2(*input)}')
