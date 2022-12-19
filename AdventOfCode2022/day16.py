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

class SearchState:
    def __init__(self, nodes, time=0, valve=0, vent=0):
        # Note: Agents take turns -> N turns per minute.
        self.nodes  = nodes # Location of each agent (tuple)
        self.time   = time  # Current timestamp (turns)
        self.tmax   = 30 if len(nodes) == 1 else 52
        self.valve  = valve # Bit-mask of open valves
        self.vent   = vent  # Expected total pressure relief

    def __lt__(self, other):
        return self.cost() < other.cost()

    def agent(self):        # Whose turn is it to act?
        return self.time % len(self.nodes)

    def cost(self):         # Cost function for min-heap
        return (-self.vent, self.time)

    def debug(self):
        print(f'@{self.time}: Vent {self.vent}, Rooms {self.nodes}')

    def key(self):          # Lookup key for best-cost dictionary
        return (self.nodes, self.time, self.valve)

    def next(self, rooms):  # Return a list of possible actions
        adj = []
        # Out of time?
        if self.time >= self.tmax: return adj
        # Lookup actions for the current room:
        node = self.nodes[self.agent()]
        (lbl, rate, tunnels) = rooms[node]
        # Try opening the valve?
        vmask = 2**node
        if (rate > 0) and not (self.valve & vmask):
            trem = (self.tmax - self.time - 1) // len(self.nodes)
            new_mask = self.valve | vmask
            new_vent = self.vent + trem * rate
            adj.append(SearchState(self.nodes, self.time+1, new_mask, new_vent))
        # Try moving down each tunnel...
        for move in tunnels:
            if len(self.nodes) == 1:
                new_nodes = (move,)
            elif self.agent() == 1:
                # Note: Sorting locations after each loop reduces search overlap.
                new_nodes = (min(self.nodes[0], move), max(self.nodes[0], move))
            else:
                new_nodes = (move, self.nodes[1])
            adj.append(SearchState(new_nodes, self.time+1, self.valve, self.vent))
        return adj

def dijkstra(rooms, start, verbose=0):
    cmax = SearchState(start, 9999).cost()
    iter = 0
    dist = {}
    next = [SearchState(start)]
    vent = 0
    while len(next) > 0:
        state = heappop(next)
        if state.cost() > dist.get(state.key(), cmax): continue
        iter += 1               # Count iterations for diagnostics
        if verbose > 1: state.debug()
        if verbose > 0 and iter%10000 == 0:
            print(f'Best {vent}, Queued {len(next)}, Visited {len(dist)}')
        for adj in state.next(rooms):
            if adj.cost() >= dist.get(adj.key(), cmax): continue
            vent = max(vent, adj.vent)
            dist[adj.key()] = adj.cost()
            heappush(next, adj)
    if verbose > 0: print(f'Pressure relieved: {vent}')
    return vent

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
    print(f'Part 1: {part1(input)}')
    assert(part2(test) == 1707)
    print(f'Part 2: {part2(input)}')
