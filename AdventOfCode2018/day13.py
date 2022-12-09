# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
from numpy import argsort
import sys

CART_INIT = {       # Underlying track from initial state
    '<':'-', 'v':'|', '>':'-', '^':'|'}
GET_CROSS = {
    '<': 'v<^',     # Old direction -> New direction
    'v': '>v<',     # (Cycle left, center, right, ...)
    '>': '^>v',
    '^': '<^>'}
GET_TURN = {        # For a given turn, old -> new
    '/': {'<':'v', 'v':'<', '>':'^', '^':'>'},
   '\\': {'<':'^', 'v':'>', '>':'v', '^':'<'}}
verbose = False     # Enable debug mode?

def read_input(input):
    lines = input.splitlines()
    track = [[CART_INIT.get(ch,ch) for ch in line] for line in lines]
    carts = []
    for (r,row) in enumerate(lines):
        for (c,ch) in enumerate(row):
            if ch in CART_INIT: carts.append((r, c, ch, 0))
    if verbose > 0: debug(track, carts)
    return (track, carts)

def debug(track, carts):
    temp = deepcopy(track)
    for (r, c, d, s) in filter(carts):
        temp[r][c] = 'X' if temp[r][c] in CART_INIT else d
    [print(''.join(row)) for row in temp]

def filter(carts):
    return [c for c in carts if c is not None]

def collision(carts):
    valid = filter(carts)
    rc = set([(r,c) for (r,c,ch,s) in valid])
    return len(rc) < len(valid)

def step_one(track, cart):
    if cart is None: return None
    (r, c, d, s) = cart
    # First, move forward one step.
    if d == '<': c -= 1
    if d == 'v': r += 1
    if d == '>': c += 1
    if d == '^': r -= 1
    # Update cart direction.
    if verbose > 1: print(f'{(r,c,d)} -> {track[r][c]}')
    trk = track[r][c]
    assert (trk != ' ') # Off the rails?
    if trk in GET_TURN:
        d = GET_TURN[trk][d]
    elif trk == '+':
        d = GET_CROSS[d][s]
        s = (s + 1) % 3
    return (r, c, d, s)

def step_all(track, carts, part1):
    # Figure out the update order...
    order = [x[0] for x in sorted(enumerate(carts), key=lambda x:x[1])]
    for n in order:
        carts[n] = step_one(track, carts[n])
        if collision(carts):
            if part1:   # Part 1: Stop immediately
                return (carts, n)
            else:       # Part 2: Nullify both carts
                (rr, cc, _, _) = carts[n]
                crashed = lambda r,c,d,s: r == rr and c == cc
                carts = [None if cart and crashed(*cart) else cart for cart in carts]
    # Return remaining carts only.
    return (filter(carts), None)

def part1(track, carts):
    while True:
        (carts, crash) = step_all(track, carts, True)
        if verbose > 0: debug(track, carts)
        if crash is not None:
            (rr, cc, _, _) = carts[crash]
            if verbose > 0: print(f'Crash @ {(rr, cc, crash)}')
            return (cc, rr)

def part2(track, carts):
    while True:
        (carts, _) = step_all(track, carts, False)
        if verbose > 0: debug(track, carts)
        if len(carts) == 1:
            (rr, cc, _, _) = carts[0]
            return (cc, rr)

TEST1 = \
r'''
/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
'''

TEST2 = \
r'''
/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/
'''

if __name__ == '__main__':
    if len(sys.argv) > 1: verbose = int(sys.argv[1])
    test1 = read_input(TEST1[1:])
    assert (part1(*test1) == (7,3))
    test2 = read_input(TEST2[1:])
    assert (part2(*test2) == (6,4))
    input = read_input(get_data(day=13, year=2018))
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input)}')
