# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
import opcodes, re

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

ALL_INSTRUCTIONS = list(opcodes.ALL_INSTRUCTIONS.values())
ALL_OPCODES = range(len(ALL_INSTRUCTIONS))

'''Check whether a given test could match each instruction.'''
def check_instr(before, instr, after):
    (op, a, b, c) = instr
    test = lambda fn: fn(a, b, c, before) == after
    return [test(fn) for fn in ALL_INSTRUCTIONS]

'''Element-wise AND of a check matrix.'''
def all_rows(opcode, check):
    ok = [True] * len(ALL_INSTRUCTIONS)
    for (op,chk) in check:
        if op == opcode: ok = [ok[n] and chk[n] for n in ALL_OPCODES]
    return ok

'''Count the number of legal pairings in a truth matrix.'''
def count_pairs(match):
    return sum([sum(row) for row in match])

'''Sudoku-style logical identification and cross-eliminations.'''
def sudoku_rows(match, map):
    for rr in ALL_OPCODES:
        if rr in map.keys(): continue   # Already solved?
        row = [match[rr][c] for c in ALL_OPCODES]
        if sum(row) == 1:               # Unique ID?
            cc = row.index(True)
            map[rr] = ALL_INSTRUCTIONS[cc]
            for r in ALL_OPCODES:       # Eliminations?
                if r != rr: match[r][cc] = False
            return True
    return False

def sudoku_cols(match, map):
    for cc in ALL_OPCODES:
        if ALL_INSTRUCTIONS[cc] in map.values(): continue
        col = [match[r][cc] for r in ALL_OPCODES]
        if sum(col) == 1:               # Unique ID?
            rr = col.index(True)
            map[rr] = ALL_INSTRUCTIONS[cc]
            for c in ALL_OPCODES:       # Eliminations?
                if c != cc: match[rr][c] = False
            return True
    return False

'''Identify which opcode is which function, returning a dictionary.'''
def identify_instr(test, verbose=False):
    # First check the legality for each test instruction.
    check = [(i[0],check_instr(b,i,a)) for (b,i,a) in test]
    # Check matching test vectors for each numeric opcode.
    # (Each row is a numeric opcode, each column is an instruction.)
    match = [all_rows(opcode, check) for opcode in ALL_OPCODES]
    # Sudoku-style elimination until we find a unique solution.
    map = {}
    if verbose: print(count_pairs(match))
    while sudoku_rows(match, map) or sudoku_cols(match, map):
        if verbose: print(count_pairs(match))
    return map

'''How many test instructions could be more than three opcodes?'''
def part1(test, prog):
    return sum([sum(check_instr(*t)) >= 3 for t in test])

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
    print(f'Part 2: {part2(*input)}')
