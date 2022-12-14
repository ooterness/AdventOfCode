# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
import re

def sign(x):    # How is this not built-in!?
    return -1 if x < 0 else 1 if x > 0 else 0

def read_input(input):
    numbers = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    walls = set()
    for line in input.splitlines():
        # Parse line as a series of (R,C) pairs.
        num = [int(x) for x in re.findall('[0-9]+', line)]
        pts = [(r,c) for (c,r) in zip(num[0::2], num[1::2])]
        # Draw each line segment, one tile at a time.
        for (rc0, rc1) in zip(pts[0:-1], pts[1:]):
            dr = rc1[0] - rc0[0]
            dc = rc1[1] - rc0[1]
            assert (dr == 0 and dc != 0) or (dr != 0 and dc == 0)
            for n in range(1 + max(abs(dr), abs(dc))):
                walls.add((rc0[0] + n*sign(dr), rc0[1] + n*sign(dc)))
    return walls

def add_sand(sand, max_r, part2):
    c = 500; r = 0                      # Origin point
    if (r,c) in sand: return False      # Already full?
    while r < max_r:
        if (r+1,c) not in sand:         # Straight down
            r += 1; continue
        if (r+1,c-1) not in sand:       # Down-left
            r += 1; c -= 1; continue
        if (r+1,c+1) not in sand:       # Down-right
            r += 1; c += 1; continue
        sand.add((r,c)); return True    # Stopped
    if part2:
        sand.add((r,c))
        return True                     # Stop at floor
    else:
        return False                    # Fall forever

def part1(walls):
    sand = deepcopy(walls)
    max_r = max([r for (r,c) in sand])
    while add_sand(sand, max_r, False): None
    return len(sand) - len(walls)

def part2(walls):
    sand = deepcopy(walls)
    max_r = max([r for (r,c) in sand]) + 1
    while add_sand(sand, max_r, True): None
    return len(sand) - len(walls)

TEST = \
'''
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=14, year=2022))
    assert(part1(test) == 24)
    assert(part2(test) == 93)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
