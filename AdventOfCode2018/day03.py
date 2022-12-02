# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(input):
    read_line = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    return [read_line(line) for line in input.splitlines()]

def count(claims):
    result = {}
    for [nn,xx,yy,ww,hh] in claims:
        for x in range(xx, xx+ww):
            for y in range(yy, yy+hh):
                result[(x,y)] = result.get((x,y),0) + 1
    return result

def part1(claims):
    ref = count(claims)
    return sum([n > 1 for n in ref.values()])

def part2(claims):
    ref = count(claims)
    for [nn,xx,yy,ww,hh] in claims:
        overlap = 0
        for x in range(xx, xx+ww):
            for y in range(yy, yy+hh):
                if ref.get((x,y)) > 1: overlap += 1
        if overlap == 0: return nn
    return None

TEST = \
'''
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=3, year=2018))
    assert (part1(test) == 4)
    assert (part2(test) == 3)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
