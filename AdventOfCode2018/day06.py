# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(input):
    read_line = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    return [read_line(line) for line in input.splitlines()]

def extent_x(coords):
    x0 = min(x for x,y in coords)
    x1 = max(x for x,y in coords)
    return (x0, x1)

def extent_y(coords):
    y0 = min(y for x,y in coords)
    y1 = max(y for x,y in coords)
    return (y0, y1)

def manhattan(c0, c1):
    return abs(c0[0] - c1[0]) + abs(c0[1] - c1[1])

def part1(coords):
    # Find overall map boundary.
    (x0, x1) = extent_x(coords)
    (y0, y1) = extent_y(coords)
    # Assign each vertex in these bounds...
    area = {}
    reject = []
    for x in range(x0, x1+1):
        for y in range(y0, y1+1):
            # Calculate distance to each reference point.
            xy = [x,y]
            dd = [manhattan(xy, c) for c in coords]
            # Which indices are the closest?
            cc = [n for n,d in enumerate(dd) if d == min(dd)]
            # Is there a unique shortest distance?
            if len(cc) == 1:
                # Increment area attached to this referece.
                cc = cc[0]
                area[cc] = area.get(cc, 0) + 1
                # Anything touching outer edge has infinite area.
                if x == x0 or x == x1 or y == y0 or y == y1:
                    reject.append(cc)
    # For each reference point...
    max_area = 0
    for n in range(len(coords)):
        if n in reject: continue
        max_area = max(max_area, area[n])
    return max_area

def part2(coords, limit):
    # Find overall map boundary.
    (x0, x1) = extent_x(coords)
    (y0, y1) = extent_y(coords)
    # Test each vertex in these bounds...
    total = 0
    for x in range(x0, x1+1):
        for y in range(y0, y1+1):
            # Calculate distance to each reference point.
            xy = [x,y]
            dd = [manhattan(xy, c) for c in coords]
            if sum(dd) < limit: total += 1
    return total

TEST = \
'''
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=6, year=2018))
    assert (part1(test) == 17)
    assert (part2(test, 32) == 16)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input, 10000)}')
