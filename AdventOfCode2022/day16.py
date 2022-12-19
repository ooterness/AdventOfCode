# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from heapq import heappop, heappush

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

class SearchState(object):
    def __init__(self, nodes, rate, rsum, time, valve, vent):
        # Note: Agents take turns -> N turns per minute.
        self.nodes  = nodes # Location of each agent (tuple)
        self.rate   = rate  # Current vent flow-rate
        self.rsum   = rsum  # Maximum vent flow-rate
        self.time   = time  # Current timestamp (turns)
        self.tmax   = 30 if len(nodes) == 1 else 52
        self.valve  = valve # Bit-mask of open valves
        self.vent   = vent  # Expected total pressure relief

    def __hash__(self):     # Hash function for "visited"
        return hash((self.nodes, self.time, self.valve, self.vent))

    def __lt__(self, other):
        return self.hint() < other.hint()

    def agent(self):        # Whose turn is it to act?
        return self.time % self.agents()

    def agents(self):       # Number of agents
        return len(self.nodes)

    def trem(self):         # Remaining time in minutes
        return (self.tmax * self.time) // self.agents()

    def cost(self):         # Cost function for problem
        return self.rsum*(self.time//self.agents()) - self.vent

    def hint(self):         # Cost hint for A* search
        return self.cost()  # - self.rate * self.trem()

    def debug(self):
        print(f'@{self.time}: Vent {self.vent}, Rooms {self.nodes}')

    def key(self):          # Lookup key for best-cost dictionary
        return (self.nodes, self.time, self.valve)

    def next(self, rooms):  # Return a list of possible actions
        adj = []
        # Give credit for all open valves.
        new_vent = self.vent
        if self.agent() == 0: new_vent += self.rate
        # Final turn doesn't matter -> just wait.
        if self.time == self.tmax-1:
            return [SearchState(self.nodes, self.rate, self.rsum, self.time+1, self.valve, new_vent)]
        # Lookup actions for the current room:
        node = self.nodes[self.agent()]
        (_, rate, tunnels) = rooms[node]
        # Try opening the valve?
        vmask = 2**node
        if (rate > 0) and not (self.valve & vmask):
            adj.append(SearchState(self.nodes, self.rate + rate, self.rsum, self.time+1, self.valve|vmask, new_vent))
        # Try moving down each tunnel...
        for move in tunnels:
            if self.agents() == 1:
                new_nodes = (move,)
            elif self.agent() == 1:
                # Note: Sorting locations after each loop reduces search overlap.
                new_nodes = (min(self.nodes[0], move), max(self.nodes[0], move))
            else:
                new_nodes = (move, self.nodes[1])
            adj.append(SearchState(new_nodes, self.rate, self.rsum, self.time+1, self.valve, new_vent))
        return adj

def astar(rooms, start, verbose=0):
    rsum = sum([rate for (_,rate,_) in rooms])
    cmax = SearchState(start, 0, rsum, 9999, 0, 0).cost()
    init = SearchState(start, 0, rsum, 0, 0, 0)
    iter = 0
    dist = {}   # AKA "gscore"
    next = [init]
    vent = 0
    visited = set([init])
    while len(next) > 0:
        state = heappop(next)
        if state.time == state.tmax: break
        #if state.cost() > dist.get(state.key(), cmax): continue
        iter += 1               # Count iterations for diagnostics
        if verbose > 1: state.debug()
        if verbose > 0 and iter%10000 == 0:
            print(f'Best {vent}, Queued {len(next)}, Visited {len(dist)}')
        for adj in state.next(rooms):
            if adj.cost() >= dist.get(adj.key(), cmax): continue
            vent = max(vent, adj.vent)
            dist[adj.key()] = adj.cost()
            if adj not in visited:
                heappush(next, adj)
                visited.add(adj)
    if verbose > 0: print(f'Pressure relieved: {vent}')
    return vent

def part1(rooms):
    return astar(rooms, (0,))

def part2(rooms):
    return astar(rooms, (0,0))

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
