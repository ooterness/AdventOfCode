# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    return input.splitlines()

def common(sack):
    mid     = len(sack) // 2
    left    = sack[:mid]
    right   = sack[mid:]
    for item in left:
        if item in right: return item
    return None

REF_a = ord('a')
REF_A = ord('A')
def priority(x):
    y = ord(x)
    if y >= REF_a:  return 1 + y - REF_a
    else:           return 27 + y - REF_A

def part1(sacks):
    return sum([priority(common(sack)) for sack in sacks])

def badge(sack1, sack2, sack3):
    for item in sack1:
        if (item in sack2) and (item in sack3): return item
    return None

def part2(sacks):
    return sum([priority(badge(sacks[n], sacks[n+1], sacks[n+2]))
        for n in range(0, len(sacks), 3)])

TEST= \
'''
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=3, year=2022))
    assert(part1(test) == 157)
    assert(part2(test) == 70)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
