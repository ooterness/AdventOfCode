# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    lines = [line.split(' ') for line in input.splitlines()]
    init = lines[0][2]
    rules = {}
    for [x, y, z] in lines[2:]:
        rules[x] = z
    return (init, rules)

def step(offset, state, rules):
    curr = '.'*4 + state + '.'*4
    next = ''.join([rules.get(curr[n:n+5], '.') for n in range(len(state)+4)])
    first = next.find('#')
    final = next.rfind('#')
    return (first+offset-2, next[first:final+1])

def score(offset, state):
    return sum([offset+n for (n, ch) in enumerate(state) if ch == '#'])

def part1(init, rules):
    (offset, state) = (0, init)
    for gen in range(20):
        (offset, state) = step(offset, state, rules)
    return score(offset, state)

def part2(init, rules, maxgen=50000000000):
    (gen, offset, state) = (0, 0, init)
    while gen < maxgen: # Run simulation until it converges...
        (gen, poffset, pstate) = (gen+1, offset, state)
        (offset, state) = step(offset, state, rules)
        if pstate == state: break
    delta = (offset - poffset) * (maxgen - gen)
    return score(offset + delta, state)

TEST = \
'''
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=12, year=2018))
    assert (part1(*test) == 325)
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input)}')
