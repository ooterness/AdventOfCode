# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(input):
    readline = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    return [readline(line) for line in input.splitlines()]

def part1(ranges):
    total = 0
    for [a0, a1, b0, b1] in ranges:
        a_in_b = (b0 <= a0 and a1 <= b1)
        b_in_a = (a0 <= b0 and b1 <= a1)
        if a_in_b or b_in_a: total += 1
    return total

def part2(ranges):
    total = 0
    for [a0, a1, b0, b1] in ranges:
        ovr1 = (a0 <= b0 and b0 <= a1)
        ovr2 = (a0 <= b1 and b1 <= a1)
        ovr3 = (b0 <= a0 and a0 <= b1)
        ovr4 = (b0 <= a1 and a1 <= b1)
        if ovr1 or ovr2 or ovr3 or ovr4: total += 1
    return total

TEST= \
'''
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=4, year=2022))
    assert(part1(test) == 2)
    assert(part2(test) == 4)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
