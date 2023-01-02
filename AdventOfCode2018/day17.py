# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import re, sys

def read_input(input):
    # Convert lines like "x=495, y=2..7" into individual tiles.
    clay = set()
    for line in input.splitlines():
        ints = [int(x) for x in re.findall('[0-9]+', line)]
        if line[0] == 'x':  # Fixed X?
            (x, y0, y1) = ints
            for y in range(y0, y1+1): clay.add((x,y))
        else:               # Fixed Y?
            (y, x0, x1) = ints
            for x in range(x0, x1+1): clay.add((x,y))
    return clay

def debug(clay, still):
    xmin = min([x for (x,y) in still])
    xmax = max([x for (x,y) in still])
    ymin = min([y for (x,y) in still])
    ymax = max([y for (x,y) in still])
    is_clay = lambda xy: xy in clay
    is_lake = lambda xy: xy in still and still[xy]
    is_flow = lambda xy: xy in still and not still[xy]
    get_char = lambda xy: '#' if is_clay(xy) \
                     else '~' if is_lake(xy) \
                     else '|' if is_flow(xy) else '.'
    print(f'Water tiles: {len(still) - len(clay)}')
    for y in range(ymin, ymax+1):
        print(''.join([get_char((x,y)) for x in range(xmin, xmax+1)]))

# Recursive flood to the left.
def flood_l(still, x, y, ymax, fill):
    if (x,y) in still: None         # Use cache?
    elif flood_d(still, x, y+1, ymax):
        tmp = flood_l(still, x-1, y, ymax, fill)
        if fill is None: return tmp
        else: still[(x,y)] = fill
    else:
        still[(x,y)] = False
    return still[(x,y)]

# Recursive flood to the right.
def flood_r(still, x, y, ymax, fill):
    if (x,y) in still: None         # Use cache?
    elif flood_d(still, x, y+1, ymax):
        tmp = flood_r(still, x+1, y, ymax, fill)
        if fill is None: return tmp
        else: still[(x,y)] = fill
    else:
        still[(x,y)] = False
    return still[(x,y)]

# Recursive flood downward.
def flood_d(still, x, y, ymax):
    if y > ymax: return False       # Out of bounds?
    if (x,y) in still: None         # Use cache?
    elif flood_d(still, x, y+1, ymax):
        fl = flood_l(still, x-1, y, ymax, None) # Scan first
        fr = flood_r(still, x+1, y, ymax, None)
        flood_l(still, x-1, y, ymax, fl and fr) # Then backfill
        flood_r(still, x+1, y, ymax, fl and fr)
        still[(x,y)] = fl and fr
    else:
        still[(x,y)] = False
    return still[(x,y)]

def flood(clay, verbose):
    ymin = min([y for (x,y) in clay])
    ymax = max([y for (x,y) in clay])
    still = dict.fromkeys(clay, True)
    flood_d(still, 500, ymin, ymax)
    if verbose: debug(clay, still)
    return still

def part1(clay, verbose):
    still = flood(clay, verbose)
    return len(still) - len(clay)

def part2(clay):
    still = flood(clay, False)
    return sum(still.values()) - len(clay)

TEST = \
'''
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
'''

if __name__ == '__main__':
    sys.setrecursionlimit(100000)
    verbose = len(sys.argv) > 1
    test = read_input(TEST.strip())
    input = read_input(get_data(day=17, year=2018))
    assert(part1(test, verbose) == 57)
    print(f'Part 1: {part1(input, verbose)}')
    assert(part2(test) == 29)
    print(f'Part 2: {part2(input)}')
