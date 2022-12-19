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
        self.costs = [                      # Cost to build each robot:
            np_array([raw[1],0,0,0]),       # Ore robot
            np_array([raw[2],0,0,0]),       # Clay robot
            np_array([raw[3],raw[4],0,0]),  # Obsidian robot
            np_array([raw[5],0,raw[6],0]),  # Geode robot
        ]

def read_input(input):
    return [Blueprint(line) for line in input.splitlines()]

def can_build(factory, build):          # Can we afford a given robot?
    (bp, time, ores, robots) = factory
    if build is None: return True
    else: return all(ores >= bp.costs[build])

def step(factory, build):               # Simulate one timestep
    (bp, time, ores, robots) = factory
    new_ores    = ores + robots         # Robots gather resources
    new_robots  = np_array(robots)
    if build is not None:               # Build a new robot?
        new_ores -= bp.costs[build]
        new_robots[build] += 1
    return (bp, time+1, new_robots, new_ores)

def max_geodes(factory):                # Max geodes from initial state?
    (bp, time, ores, robots) = factory
    if time >= 24: return ores[3]       # Final score = Number of geodes
    if can_build(factory, 3):           # If we can build a geode robot, do so.
        return max_geodes(step(factory, 3))
    return max([                        # Recursively try each valid option
        max_geodes(step(factory, robot))
        for robot in [None,0,1,2]
        if can_build(factory, robot)
    ])

def quality(bp):                        # Calculate quality score
    init = (bp, 0, INIT_ORES, INIT_ROBOTS)
    print(bp.id * max_geodes(init))#???
    return bp.id * max_geodes(init)

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
