# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from heapq import heappop, heappush
import re

def read_input(input):
    read_line = lambda line: [int(x) for x in re.findall('[0-9\-]+', line)]
    return [Nanobot(*read_line(line)) for line in input.splitlines()]

# Return minimum value of abs(x) for x in [x0..x1]
def min_abs(x0, x1):
    if x0 >= 0: return x0   # Both to right
    if x1 >= 0: return 0    # Straddle origin
    return -x1              # Both to left

# Class representing a Nanobot (XYZ + Manhattan radius)
class Nanobot:
    def __init__(self, x, y, z, r):
        (self.x, self.y, self.z, self.r) = (x, y, z, r)

    def in_range(self, x, y, z):
        dd = abs(self.x - x) + abs(self.y - y) + abs(self.z - z)
        return dd <= self.r

    def count_in_range(self, bots):
        return sum([self.in_range(b.x, b.y, b.z) for b in bots])

    def corners(self):
        return [(self.x, self.y, self.z-self.r),
                (self.x, self.y, self.z+self.r),
                (self.x, self.y-self.r, self.z),
                (self.x, self.y+self.r, self.z),
                (self.x-self.r, self.y, self.z),
                (self.x+self.r, self.y, self.z)]

# Class representing a cubic bounding-box.
class Cube:
    def __init__(self, x, y, z, r):
        self.rr = r
        self.x0 = x
        self.x1 = x + r - 1
        self.y0 = y
        self.y1 = y + r - 1
        self.z0 = z
        self.z1 = z + r - 1

    # Minimum Manhattan distance to origin, for Part 2 tiebreaker.
    def score(self):
        x = min_abs(self.x0, self.x1)
        y = min_abs(self.y0, self.y1)
        z = min_abs(self.z0, self.z1)
        return x + y + z

    def __lt__(self, other):
        return self.score() < other.score()

    def corners(self):
        return [(self.x0, self.y0, self.z0),
                (self.x0, self.y0, self.z1),
                (self.x0, self.y1, self.z0),
                (self.x0, self.y1, self.z1),
                (self.x1, self.y0, self.z0),
                (self.x1, self.y0, self.z1),
                (self.x1, self.y1, self.z0),
                (self.x1, self.y1, self.z1)]

    # Halve each axis for an eight-way split.
    def split(self):
        r = self.rr // 2
        return [Cube(self.x0,   self.y0,   self.z0,   r),
                Cube(self.x0,   self.y0,   self.z0+r, r),
                Cube(self.x0,   self.y0+r, self.z0,   r),
                Cube(self.x0,   self.y0+r, self.z0+r, r),
                Cube(self.x0+r, self.y0,   self.z0,   r),
                Cube(self.x0+r, self.y0,   self.z0+r, r),
                Cube(self.x0+r, self.y0+r, self.z0,   r),
                Cube(self.x0+r, self.y0+r, self.z0+r, r)]

    def contains(self, x, y, z):
        return self.x0 <= x and x <= self.x1 \
           and self.y0 <= y and y <= self.y1 \
           and self.z0 <= z and z <= self.z1

    def overlap(self, bot):
        return self.contains(bot.x, bot.y, bot.z) \
            or any([bot.in_range(*xyz) for xyz in self.corners()]) \
            or any([self.contains(*xyz) for xyz in bot.corners()])

    def count_overlap(self, bots):
        return sum([self.overlap(b) for b in bots])

# Return a size-2^N bounding box for a list of Nanobots.
def bounding_cube_2n(bots):
    x0 = min([b.x - b.r for b in bots])
    x1 = max([b.x + b.r for b in bots])
    y0 = min([b.y - b.r for b in bots])
    y1 = max([b.y + b.r for b in bots])
    z0 = min([b.z - b.r for b in bots])
    z1 = max([b.z + b.r for b in bots])
    rr = max(x1 - x0, y1 - y0, z1 - z0)
    return Cube(x0, y0, z0, 2**rr.bit_length())

def part1(bots):
    (max_rad, max_idx) = max([(bot.r,n) for n, bot in enumerate(bots)])
    return bots[max_idx].count_in_range(bots)   # Includes self

def part2(bots):
    # Start from a bounding box containing all nanobots.
    # Keep subdividing, prioritizing by upper bound of final score.
    queue = [(-len(bots), bounding_cube_2n(bots))]
    while len(queue) > 0:
        (_, cube) = heappop(queue)
        if cube.rr < 2: return cube.score() # Size 1 = Done
        for sub in cube.split():
            score = sub.count_overlap(bots)
            heappush(queue, (-score, sub))
    return None

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
