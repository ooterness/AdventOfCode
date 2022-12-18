# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import re

def read_input(input):
    numbers = lambda line: [int(x) for x in re.findall('[0-9\-]+', line)]
    return set([tuple(numbers(line)) for line in input.splitlines()])

def adj(cube):
    (x,y,z) = cube
    return [(x-1,y,z), (x+1,y,z), (x,y-1,z), (x,y+1,z), (x,y,z-1), (x,y,z+1)]

def part1(lava):
    # Find total exposed surface area.
    surface = 0
    for cube in lava:
        surface += 6 - sum([p in lava for p in adj(cube)])
    return surface

def part2(lava):
    # Find dimensions of bounding box.
    max_x = max([x for (x,y,z) in lava]) + 1
    max_y = max([y for (x,y,z) in lava]) + 1
    max_z = max([z for (x,y,z) in lava]) + 1
    # Flood-fill from a point outside the box.
    # Inputs range from 0-N so pick (-1, -1, -1).
    queue = [(-1, -1, -1)]
    visited = set(queue)
    surface = 0
    while len(queue) > 0:
        for (x,y,z) in adj(queue.pop()):
            if (x,y,z) in lava: surface += 1; continue
            if (x,y,z) in visited: continue
            if x < -1 or max_x < x: continue
            if y < -1 or max_y < y: continue
            if z < -1 or max_z < z: continue
            visited.add((x,y,z))
            queue.append((x,y,z))
    return surface

TEST1 = \
'''
1,1,1
2,1,1
'''

TEST2 = \
'''
2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
'''

if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    input = read_input(get_data(day=18, year=2022))
    assert(part1(test1) == 10)
    assert(part1(test2) == 64)
    assert(part2(test1) == 10)
    assert(part2(test2) == 58)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
    # 2459 too low???
