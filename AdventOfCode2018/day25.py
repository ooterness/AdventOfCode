# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import re

def read_input(input):
    read_line = lambda line: [int(x) for x in re.findall('[0-9\-]+', line)]
    return set([tuple(read_line(line)) for line in input.splitlines()])

def adjacent(star):
    (w0,x0,y0,z0) = star
    adj = []
    for w in range(-3, 4):
        for x in range(-3, 4):
            for y in range(-3, 4):
                for z in range(-3, 4):
                    dd = abs(w) + abs(x) + abs(y) + abs(z)
                    if 0 < dd and dd <= 3:
                        adj.append((w0+w, x0+x, y0+y, z0+z))
    return adj

def part1(stars):
    constellations = []
    for star in stars:
        # Find a list of adjacent constellations.
        test = adjacent(star)   # List of adjacent coordinates to test
        join = [con for con in constellations if any([t in con for t in test])]
        # How many constellations in range?
        if len(join) == 0:      # New constellation?
            constellations.append(set([star]))
        elif len(join) == 1:    # Simple join
            join[0].add(star)
        else:                   # Form crosslink
            join[0].add(star)
            for con in join[1:]:
                constellations.remove(con)
                for c in con: join[0].add(c)
    return len(constellations)

TEST1 = \
'''
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
'''

TEST2 = \
'''
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
'''

TEST3 = \
'''
1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
'''

TEST4 = \
'''
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
'''
if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    test3 = read_input(TEST3.strip())
    test4 = read_input(TEST4.strip())
    input = read_input(get_data(day=25, year=2018))
    assert(part1(test1) == 2)
    assert(part1(test2) == 4)
    assert(part1(test3) == 3)
    assert(part1(test4) == 8)
    print(f'Part 1: {part1(input)}')
