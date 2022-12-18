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
    # (Sorting both lists ensures room 'AA' is at index zero.)
    labels = sorted([line[6:8] for line in input.splitlines()])
    rooms = [read_line(line, labels) for line in input.splitlines()] 
    return sorted(rooms)

def dijkstra(rooms, start, verbose=False):
    # Agents take turns. Divide each minute into N ticks.
    agents = len(start)                     # Number of agents = Length of initial state
    max_time = 30 if agents == 1 else 52    # 30 minutes = 30 ticks, or 26 minutes = 52 ticks
    # Lambda functions for cost-lookup and queue state-vectors.
    # (Indirect makes it easier to change search order.)
    make_cost = lambda time,vent,nodes,valve: vent
    make_dist = lambda time,vent,nodes,valve: (time,nodes,valve)
    make_next = lambda time,vent,nodes,valve: (time,vent,nodes,valve)
    get_state = lambda time,vent,nodes,valve: (time,vent,nodes,valve)
    # Prioritized search using Dijstra's algorithm:
    dist = {}                               # For each time/rooms/vmask, maximum pressure released
    next = [make_next(-max_time,0,start,0)] # Min-heap for search queue. (time, cost, nodes, valves)
    while len(next) > 0:
        # Next state to consider?
        (time,vent,nodes,valve) = get_state(*heappop(next))
        cost = make_cost(time,vent,nodes,valve)
        dref = make_dist(time,vent,nodes,valve)
        if time >= 0: break                     # Out of time? Better solution?
        if cost > dist.get(dref, 0): continue
        if verbose: print(dref)
        agent = time % agents                   # Who's moving this tick?
        time += 1
        trem = -time // agents                  # Minutes remaining (round down)
        node = nodes[agent]
        (lbl,rate,outlets) = rooms[node]        # Lookup current agent/room
        vmask = 2**node
        if (rate > 0) and not (valve & vmask):  # Try opening valve?
            new_vent = vent - trem * rate
            new_cost = make_cost(time, new_vent, nodes, valve|vmask)
            new_dist = make_dist(time, new_vent, nodes, valve|vmask)
            new_next = make_next(time, new_vent, nodes, valve|vmask)
            if new_cost < dist.get(new_dist, 0):
                dist[new_dist] = new_cost
                heappush(next, new_next)
        for move in outlets:                    # Try each tunnel?
            if agents == 2 and agent == 1:
                new_move = (min(nodes[0], move), max(nodes[0], move))
            elif agents == 2:
                new_move = (move, nodes[1])
            else:
                new_move = (move,)
            new_dist = make_dist(time, vent, new_move, valve)
            new_next = make_next(time, vent, new_move, valve)
            if cost < dist.get(new_dist, 1):
                dist[new_dist] = cost
                heappush(next, new_next)
    return -min(dist.values())

def part1(rooms):
    return dijkstra(rooms, (0,))

def part2(rooms):
    return dijkstra(rooms, (0,0))

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
    assert(part2(test) == 1707)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
    # 2215 = Too low
