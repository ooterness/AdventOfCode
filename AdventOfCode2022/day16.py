# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from heapq import heappop, heappush
import re

def read_line(line, labels):
    # Reads room parameters: Label, Valve/rate, Exits
    words = line.split(' ')
    node = words[1]     # "AA"
    rate = words[4]     # "rate=0";
    outs = words[9:]    # "DD," "II," "BB" (note commas)
    rate_i = int(rate[5:-1])
    outs_i = [labels.index(x[0:2]) for x in outs]
    return (node, rate_i, outs_i)

def read_input(input):
    # Two passes: Labels only, then room parameters.
    labels = [line[6:8] for line in input.splitlines()]
    return [read_line(line, labels) for line in input.splitlines()]

def dijkstra(rooms, verbose=False):
    dist = {}               # For each time/room/vmask, maximum pressure released
    next = [(-30,0,0,0)]    # Min-heap for search queue. (time, cost, node, valves)
    while len(next) > 0:
        (time,cost,node,valve) = heappop(next)  # Next state to consider?
        if time >= 0: continue                  # Out of time? Already explored?
        if cost > dist.get((time,node), 0): continue
        if verbose: print((-time,-cost,node,valve))
        time += 1
        (lbl,rate,outlets) = rooms[node]        # Lookup current room
        vmask = 2**node
        if (rate > 0) and not (valve & vmask):  # Try opening valve?
            new_state = (time, node, valve|vmask)
            new_cost = cost + (time) * rate
            if new_cost < dist.get(new_state, 0):
                dist[new_state] = new_cost
                heappush(next, (time,new_cost,node,valve|vmask))
        for move in outlets:                    # Try each tunnel?
            new_state = (time, move, valve)
            if cost < dist.get(new_state, 1):
                dist[new_state] = cost
                heappush(next, (time,cost,move,valve))
    return dist

def part1(rooms):
    return -min(dijkstra(rooms).values())

def part2(rooms):
    None

TEST = \
'''
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=16, year=2022))
    assert(part1(test) == 1651)
    #assert(part2(test) == 29)
    print(f'Part 1: {part1(input)}')
    #print(f'Part 2: {part2(input)}')
    #??? 2285 = Too high
    #??? 2484 = Too high
