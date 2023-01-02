# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import sys

def read_input(input):
    scan = {}
    rows = len(input.splitlines())
    for (r,row) in enumerate(input.splitlines()):
        for (c,col) in enumerate(row):
            scan[(r,c)] = col
        cols = len(row)
    return (rows, cols, scan)

def debug(rows, cols, scan, label):
    print(label)
    for r in range(rows):
        print(''.join([scan.get((r,c), '.') for c in range(cols)]))

def step(rows, cols, scan):
    next = {}
    get_rc = lambda rc: scan.get(rc, '.')
    for r in range(rows):
        for c in range(cols):
            adj_rc = [(r-1,c-1), (r-1,c), (r-1,c+1), (r,c-1), (r,c+1), (r+1,c-1), (r+1,c), (r+1,c+1)]
            adj = [get_rc(rc) for rc in adj_rc]
            prev = get_rc((r,c))
            wood = sum([x == '|' for x in adj])
            yard = sum([x == '#' for x in adj])
            next[(r,c)] = '|' if prev == '.' and wood >= 3 \
                     else '#' if prev == '|' and yard >= 3 \
                     else '.' if prev == '#' and (wood == 0 or yard == 0) \
                     else prev
    return next

def score(scan):
    wood = sum([x == '|' for x in scan.values()])
    yard = sum([x == '#' for x in scan.values()])
    return wood * yard

def part1(rows, cols, scan, verbose):
    for n in range(10):
        scan = step(rows, cols, scan)
        if verbose: debug(rows, cols, scan, f'Step {n+1}:')
    return score(scan)

def part2(rows, cols, scan):
    # Iterate until we find a repeated state.
    prev = []
    while scan not in prev:
        prev.append(scan)
        scan = step(rows, cols, scan)
    # Determine the repetition interval.
    idx1 = prev.index(scan) # 1st index
    idx2 = len(prev)        # 2nd index
    # Project to requested time index.
    idx3 = idx1 + (1000000000 - idx1) % (idx2 - idx1)
    return score(prev[idx3])

TEST = \
'''
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.
'''

if __name__ == '__main__':
    verbose = len(sys.argv) > 1
    test = read_input(TEST.strip())
    input = read_input(get_data(day=18, year=2018))
    assert(part1(*test, verbose) == 1147)
    print(f'Part 1: {part1(*input, verbose)}')
    print(f'Part 2: {part2(*input)}')
