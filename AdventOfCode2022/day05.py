# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data
from copy import deepcopy

def read_input(input):
    # Find the blank line in the input.
    lines = input.splitlines()
    split = lines.index('')
    # Everything above the split is the initial state diagram.
    # Get the character-index and label for each column...
    labels = [x for x in re.finditer('[0-9]+', lines[split-1])]
    stacks = [(x.start(), int(x.group())) for x in labels]
    # Read the initial state from the bottom up...
    init = {lbl:[] for (col,lbl) in stacks}
    for line in reversed(lines[:split-1]):
        for (col, lbl) in stacks:
            if line[col] != ' ': init[lbl].append(line[col])
    # Everything below the split is the move list.
    moves = []
    for line in lines[split+1:]:
        moves.append([int(x) for x in re.findall('[0-9]+', line)])
    return (init, moves)

def top_str(stacks):
    return ''.join([stack[-1] for stack in stacks.values()])

def part1(init, moves):
    stacks = deepcopy(init)
    for (qty, fr, to) in moves:
        for n in range(qty):
            stacks[to].append(stacks[fr].pop())
    return top_str(stacks)

def part2(init, moves):
    stacks = deepcopy(init)
    for (qty, fr, to) in moves:
        stacks[to].extend(stacks[fr][-qty:])
        stacks[fr] = stacks[fr][:-qty]
    return top_str(stacks)

TEST= \
'''
    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
'''

if __name__ == '__main__':
    test = read_input(TEST[1:])
    input = read_input(get_data(day=5, year=2022))
    assert(part1(*test) == 'CMZ')
    assert(part2(*test) == 'MCD')
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input)}')
