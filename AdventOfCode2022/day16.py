# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import copy
from heapq import heappop, heappush

def read_line(line, labels):
    # Reads room parameters: Label, Valve/rate, Exits
    words = line.split(' ')
    name = words[1]     # "AA"
    rate = words[4]     # "rate=0";
    outs = words[9:]    # "DD," "II," "BB" (note commas)
    idx  = labels.index(name)
    rate_i = int(rate[5:-1])
    outs_i = [labels.index(x[0:2]) for x in outs]
    return (idx, rate_i, outs_i)

def read_input(input):
    # Sort the input rows to ensure room 'AA' is at index zero.
    lines = sorted(input.splitlines())
    # Two passes: Labels only, then room parameters.
    labels = [line[6:8] for line in lines]
    rooms = [read_line(line, labels) for line in lines]
    # Simplify the network before returning.
    return simplify(sorted(rooms))

# Given a network of rooms, find min-distance to each relevant room.
# (i.e., There's never a reason to stop in a room except to open a valve.)
def simplify(rooms):
    # Initialize the distance matrix.
    dist = [[999 for x in rooms] for y in rooms]
    for (idx,rate,outs) in rooms:
        dist[idx][idx] = 0
        for out in outs: dist[idx][out] = 1
    # Floyd-Warshall for the win.
    for (x,_,_) in rooms:
        for (y,_,_) in rooms:
            for (z,_,_) in rooms:
                dist[y][z] = min(dist[y][z], dist[y][x] + dist[x][z])
    # Compress to the "good" rooms where rate > 0.
    good_rooms = [r for (r,rate,outs) in rooms if r == 0 or rate > 0]
    good_rate = lambda r: rooms[r][1]
    good_dist = lambda r: [dist[r][n] for n in good_rooms]
    return [(n, good_rate(r), good_dist(r)) for (n,r) in enumerate(good_rooms)]

class SearchState(object):
    def __init__(self, agents, rsum, tmax):
        self.agents = agents    # Number of active agents
        self.node0  = 0         # Where is each agent located or headed?
        self.node1  = 0
        self.busy0  = 0         # Travel time remaining for each agent
        self.busy1  = 0 if agents > 1 else tmax
        self.rate   = 0         # Rate of currently open valves
        self.rsum   = rsum      # Sum of all possible valves
        self.time   = 0         # Current timestep
        self.trem   = tmax      # Remaining time
        self.valve  = 0         # Bit-mask of open valves
        self.vent   = 0         # Accumulated pressure relief

    def __hash__(self):         # Hash function for "visited"
        return hash(self.key())

    def __lt__(self, other):    # Sorting function for min-heap
        return self.cost() < other.cost()
        
    def cost(self):             # Total-cost function for Dijkstra
        return self.time * self.rsum - self.vent

    def key(self):              # Unique key function for memoization
        return (self.node0, self.node1, self.busy0, self.busy1, self.time, self.valve)

    def step(self, rooms):      # Simulate up to the next decision point.
        # Move forward in time...
        dt = min(self.busy0, self.busy1, self.trem)
        self.busy0 -= dt
        self.busy1 -= dt
        self.time  += dt
        self.trem  -= dt
        self.vent  += dt * self.rate
        # Lookup current room for each agent.
        (idx0,rate0,_) = rooms[self.node0]
        (idx1,rate1,_) = rooms[self.node1]
        mask0 = 2**idx0
        mask1 = 2**idx1
        # Mark designated valves as open.
        if self.time > 0 and (not self.valve & mask0) and (not self.busy0):
            self.valve |= 2**idx0
            self.rate  += rate0
        if self.time > 0 and (not self.valve & mask1) and (not self.busy1):
            self.valve |= 2**idx1
            self.rate  += rate1
        return self

    def next(self, rooms):      # Return a list of possible actions
        result = []
        # Lookup current room for each agent.
        (_,_,dist0) = rooms[self.node0]
        (_,_,dist1) = rooms[self.node1]
        # Assign first agent if idle.
        if (not self.busy0) and (not result):
            for (n,dist) in enumerate(dist0):
                (_,rate,_) = rooms[n]
                if not rate: continue
                if self.valve & 2**n: continue
                if dist >= self.trem: continue
                if self.busy1 and n == self.node1: continue
                adj = copy(self)    # Travel to valve N and close it
                adj.node0 = n
                adj.busy0 = dist + 1
                result.append(adj.step(rooms))
            if not result:
                adj = copy(self)    # Nothing to do but wait
                adj.busy0 = self.trem
                result.append(adj.step(rooms))
        # Assign second agent if idle.
        if (not self.busy1) and (not result):
            for (n,dist) in enumerate(dist1):
                (_,rate,_) = rooms[n]
                if not rate: continue
                if self.valve & 2**n: continue
                if dist >= self.trem: continue
                if self.busy0 and n == self.node0: continue
                adj = copy(self)    # Travel to valve N and close it
                adj.node1 = n
                adj.busy1 = dist + 1
                result.append(adj.step(rooms))
            if not result:
                adj = copy(self)    # Nothing to do but wait
                adj.busy1 = self.trem
                result.append(adj.step(rooms))
        return result

def astar(rooms, agents, tmax):
    rsum = sum([rate for (n,rate,dist) in rooms])
    cmax = tmax * rsum
    init = SearchState(agents, rsum, tmax)
    dist = {}
    next = [init]
    vent = 0
    while len(next) > 0:
        state = heappop(next)
        if state.trem == 0: break
        if state.cost() > dist.get(state.key(), cmax): continue
        for adj in state.next(rooms):
            if adj.cost() >= dist.get(adj.key(), cmax): continue
            vent = max(vent, adj.vent)
            dist[adj.key()] = adj.cost()
            heappush(next, adj)
    return vent

def part1(rooms):
    return astar(rooms, 1, 30)

def part2(rooms):
    return astar(rooms, 2, 26)

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
