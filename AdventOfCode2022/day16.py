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
    # Agents take turns, N turns = one minute.
    agents = len(start)                     # Number of agents = Length of initial state
    max_time = 30 if agents == 1 else 52    # 30 minutes = 30 ticks, or 26 minutes = 52 ticks
    dist = {}                               # For each time/rooms/vmask, maximum pressure released
    next = [(-max_time,0,start,0)]          # Min-heap for search queue. (time, cost, nodes, valves)
    while len(next) > 0:
        (time,cost,nodes,valve) = heappop(next) # Next state to consider?
        if time >= 0: break                     # Out of time? Better solution?
        if cost > dist.get((time,nodes,valve), 0): continue
        if verbose: print((-time,-cost,nodes,valve))
        agent = time % agents                   # Who's moving this tick?
        time += 1
        trem = -time // agents                  # Minutes remaining (round down)
        node = nodes[agent]
        (lbl,rate,outlets) = rooms[node]        # Lookup current agent/room
        vmask = 2**node
        if (rate > 0) and not (valve & vmask):  # Try opening valve?
            new_state = (time, nodes, valve|vmask)
            new_cost = cost - trem * rate
            if new_cost < dist.get(new_state, 0):
                dist[new_state] = new_cost
                heappush(next, (time,new_cost,nodes,valve|vmask))
        for move in outlets:                    # Try each tunnel?
            new_move = tuple([move if n==agent else nodes[n] for n in range(agents)])
            new_state = (time, new_move, valve)
            if cost < dist.get(new_state, 1):
                dist[new_state] = cost
                heappush(next, (time,cost,new_move,valve))
    return dist

def part1(rooms):
    return -min(dijkstra(rooms, (0,)).values())

def part2(rooms):
    return -min(dijkstra(rooms, (0,0)).values())

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
    # TODO: Improved algorithm? No result after a full hour.
    #print(f'Part 2: {part2(input)}')
