# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from collections import Counter
from copy import deepcopy

def read_input(input):
    elves = set()
    for (r,row) in enumerate(input.splitlines()):
        for (c,col) in enumerate(row):
            if col == '#': elves.add((r,c))
    return elves

def pgen(elves, ridx):
    # Planning returns (origin, plan) for each elf.
    for (r,c) in elves:
        # Check adjacent cells.
        nw = (r-1,c-1) in elves
        nn = (r-1,c+0) in elves
        ne = (r-1,c+1) in elves
        ee = (r+0,c+1) in elves
        se = (r+1,c+1) in elves
        ss = (r+1,c+0) in elves
        sw = (r+1,c-1) in elves
        ww = (r+0,c-1) in elves
        # Set the north/south/west/east rules.
        rules = [(not any([nw,nn,ne]), (r-1,c)),
                 (not any([sw,ss,se]), (r+1,c)),
                 (not any([nw,ww,sw]), (r,c-1)),
                 (not any([ne,ee,se]), (r,c+1))]
        # Change priority order each round.
        order = [(ridx+r)%4 for r in range(4)]
        if not any([nw,nn,ne,ee,se,ss,sw,ww]):
            yield ((r,c), (r,c))    # Stand still
        elif rules[order[0]][0]:
            yield ((r,c), rules[order[0]][1])
        elif rules[order[1]][0]:
            yield ((r,c), rules[order[1]][1])
        elif rules[order[2]][0]:
            yield ((r,c), rules[order[2]][1])
        elif rules[order[3]][0]:
            yield ((r,c), rules[order[3]][1])
        else:
            yield ((r,c), (r,c))    # Stand still

def plan(elves, ridx):
    # Wrapper for pgen(). Syntax is easier using generator/yield.
    return [p for p in pgen(elves, ridx)]

def move(moves):
    # Cancel duplicate moves.
    mcount = Counter([m1 for (m0,m1) in moves])
    return set([(m0 if mcount[m1] > 1 else m1) for (m0,m1) in moves])

def bound(elves):
    # Return the bounding box.
    r0 = min([r for (r,c) in elves])
    r1 = max([r for (r,c) in elves]) + 1
    c0 = min([c for (r,c) in elves])
    c1 = max([c for (r,c) in elves]) + 1
    return (r0, r1, c0, c1)

def area(elves):
    # Total area in bounding box.
    (r0, r1, c0, c1) = bound(elves)
    return (r1 - r0) * (c1 - c0)

def debug(elves, lbl):
    # Print the current game state.
    (r0, r1, c0, c1) = bound(elves)
    if lbl: print(lbl)
    for r in range(r0, r1):
        row = ['#' if (r,c) in elves else '.'
               for c in range(c0, c1)]
        print(''.join(row))

def part1(init, verbose=False):
    elves = init
    for n in range(10):
        elves = move(plan(elves, n))
        if verbose: debug(elves, f'Step {n+1}')
    return area(elves) - len(elves)

def part2(init):
    elves = init
    for n in range(10000):
        ref = deepcopy(elves)
        elves = move(plan(elves, n))
        if ref == elves: return n+1
    return None

TEST1 = \
'''
##
#.
..
##
'''

TEST2 = \
'''
....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
'''

if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    input = read_input(get_data(day=23, year=2022))
    assert(part1(test1) == 25)
    assert(part1(test2) == 110)
    print(f'Part 1: {part1(input)}')
    assert(part2(test1) == 4)
    assert(part2(test2) == 20)
    print(f'Part 2: {part2(input)}')
