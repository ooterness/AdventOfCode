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
MAX_TIME    = 24

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
    if robot == 3: return True              # Always good to build geode bots
    (bp, time, ores, robots) = factory
    if all(ores >= bp.costs[3]): return False   # Always prioritize geode bots
    return robots[robot] < bp.cmax[robot]   # Max one robot built per turn

def next(factory, build):                   # Simulate to next decision point
    (bp, time, ores, robots) = factory
    ores2   = np_array(ores)                # Copy input vectors
    robots2 = np_array(robots)
    while time < MAX_TIME and build >= 0:
        if all(ores2 >= bp.costs[build]):   # Can we afford to build?
            robots2[build] += 1
            ores2 -= bp.costs[build]
            build = -1
        ores2 += robots                     # Robots gather resources
        time += 1                           # Advance one timestep
    return (bp, time, ores2, robots2)

def max_geodes(factory):                    # Max geodes from initial state?
    (bp, time, ores, robots) = factory
    if time >= MAX_TIME: return ores[3]     # Final score = Number of geodes
    return max([                            # Recursively try each valid option
        max_geodes(next(factory, robot))
        for robot in range(4)
        if should_build(factory, robot)
    ])

def quality(bp):                        # Calculate quality score
    init = (bp, 0, INIT_ORES, INIT_ROBOTS)
    geodes = max_geodes(init)
    return bp.id * geodes

def part1(bp_list):
    return sum([quality(bp) for bp in bp_list])

def part2(rooms):
    None

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
    #assert(part2(test) == 1707)
    #print(f'Part 2: {part2(input)}')
