# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
import numpy as np
import re

def np_array(x):
    return np.array(x, dtype='int64')

def np_zeros(n):
    return np.zeros(n, dtype='int64')

INIT_ROBOTS = np_array([1,0,0,0])
INIT_ORES   = np_zeros(4)

class Blueprint:
    def __init__(self, line):
        raw = [int(x) for x in re.findall('[0-9]+', line)]
        self.id = raw[0]                    # ID for this blueprint
        self.costs = np_array([             # Cost to build each robot:
            [raw[1],0,0,0],                 # Ore robot
            [raw[2],0,0,0],                 # Clay robot
            [raw[3],raw[4],0,0],            # Obsidian robot
            [raw[5],0,raw[6],0],            # Geode robot
        ])
        self.cmax = self.costs.max(0)       # Max each column

def read_input(input):
    return [Blueprint(line) for line in input.splitlines()]

def can_afford(ore, cost):                  # Can we afford a given robot?
    return all(ore >= cost)

def should_build(factory, robot):           # Do we need more of a given robot?
    (bp, time, tmax, ores, robots) = factory
    # Always prioritize geode bots
    if robot == 3: return True
    elif all(ores >= bp.costs[3]): return False
    # Do we already have enough ore and/or income of this type?
    return robots[robot] < bp.cmax[robot] \
       and ores[robot] < 2*bp.cmax[robot]

def next(factory, build):                   # Simulate to next decision point
    (bp, time, tmax, ores, robots) = factory
    ores2   = np_array(ores)                # Copy input vectors
    robots2 = np_array(robots)
    while time < tmax and build >= 0:
        if all(ores2 >= bp.costs[build]):   # Can we afford to build?
            robots2[build] += 1
            ores2 -= bp.costs[build]
            build = -1
        ores2 += robots                     # Robots gather resources
        time += 1                           # Advance one timestep
    return (bp, time, tmax, ores2, robots2)

def simulate(factory):                      # Max geodes from initial state?
    (bp, time, tmax, ores, robots) = factory
    if time >= tmax: return ores[3]         # Final score = Number of geodes
    return max([                            # Recursively try each valid option
        simulate(next(factory, robot))
        for robot in range(4)
        if should_build(factory, robot)
    ])

def max_geodes(bp, tmax):                   # Max geodes from blueprint?
    return simulate((bp, 0, tmax, INIT_ORES, INIT_ROBOTS))

def quality(bp):                            # Calculate quality score
    return bp.id * max_geodes(bp, 24)

def part1(bp_list):
    return sum([quality(bp) for bp in bp_list])

def part2(bp_list):
    g = [max_geodes(bp, 32) for bp in bp_list[0:3]]
    return g[0] * g[1] * g[2]

TEST = \
'''
Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=19, year=2022))
    assert(quality(test[0]) == 1*9)
    assert(quality(test[1]) == 2*12)
    assert(part1(test) == 33)
    print(f'Part 1: {part1(input)}')
    assert(max_geodes(test[0], 32) == 56)
    assert(max_geodes(test[1], 32) == 62)
    print(f'Part 2: {part2(input)}')
