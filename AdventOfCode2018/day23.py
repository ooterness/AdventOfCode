# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import re

def read_input(input):
    read_bot = lambda line: tuple([int(x) for x in re.findall('[0-9\-]+', line)])
    return [read_bot(line) for line in input.splitlines()]

def in_range(bot_a, bot_b):
    (xa,ya,za,ra) = bot_a
    (xb,yb,zb,rb) = bot_b
    return (abs(xa-xb) + abs(ya-yb) + abs(za-zb)) <= max(ra, rb)

def count_in_range(bots, ref):
    return sum([in_range(ref, b) for b in bots])

def part1(bots):
    (max_rad, max_idx) = max([(r,n) for n,(x,y,z,r) in enumerate(bots)])
    return count_in_range(bots, bots[max_idx])  # Includes self

def part2(bots):
    # Update running best for a given xyz coordinate.
    count = lambda x,y,z: count_in_range(bots, (x,y,z,0))
    dist  = lambda x,y,z: abs(x) + abs(y) + abs(z)
    score = lambda x,y,z: (count(x,y,z), -dist(x,y,z))
    # Optimal solution will always be at a "corner".
    corners = []
    for (x,y,z,r) in bots:
        corners.append(score(x, y, z-r))
        corners.append(score(x, y, z+r))
        corners.append(score(x, y-r, z))
        corners.append(score(x, y+r, z))
        corners.append(score(x-r, y, z))
        corners.append(score(x+r, y, z))
    return -max(corners)[1]

TEST1 = \
'''
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
'''

TEST2 = \
'''
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
'''

if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    input = read_input(get_data(day=23, year=2018))
    assert(part1(test1) == 7)
    print(f'Part 1: {part1(input)}')
    assert(part2(test2) == 36)
    print(f'Part 2: {part2(input)}')
