# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re

def read_input(filename):
    read_line = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    with open(filename, 'r') as file:
        return [read_line(line) for line in file.readlines()]

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
    
if __name__ == '__main__':
    test = read_input('../input/test03.txt')
    input = read_input('../input/input03.txt')
    assert (part1(test) == 4)
    assert (part2(test) == 3)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
