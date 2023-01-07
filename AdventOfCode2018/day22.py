# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from heapq import heappop, heappush
import numpy as np
import re

# Define constants for room and tool labels.
ROOM_ROCKY  = 0
ROOM_WET    = 1
ROOM_NARROW = 2
ROOM_WALL   = 3

TOOL_TORCH  = 0
TOOL_CLIMB  = 1
TOOL_NONE   = 2

# Tool compatibility, listed by room type:
COMPATIBILITY = {
    ROOM_ROCKY:     [TOOL_CLIMB, TOOL_TORCH],
    ROOM_WET:       [TOOL_CLIMB, TOOL_NONE],
    ROOM_NARROW:    [TOOL_TORCH, TOOL_NONE],
    ROOM_WALL:      [],
}

def read_input(input):
    (depth, ctag, rtag) = [int(x) for x in re.findall('[0-9]+', input)]
    return (depth, rtag, ctag)

def erosion(depth, rtag, ctag, rmax, cmax):
    geo = np.zeros((rmax+1, cmax+1), dtype='uint64')
    for r in range(rmax+1):
        for c in range(cmax+1):
            if r == rtag and c == ctag: tmp = 0
            elif r == 0: tmp = 16807 * c
            elif c == 0: tmp = 48271 * r
            else: tmp = geo[r-1,c] * geo[r,c-1]
            geo[r,c] = (tmp + depth) % 20183
    return geo % 3

def debug(cave):
    label = ['.', '=', '|']         # Label by index (0/1/2)
    for r in range(cave.shape[0]):
        print(''.join([label[n] for n in cave[r,:]]))

def neighbors(r, c, t):             # Row, column, tool
    return [
        (1, r-1, c, t),             # Move north
        (1, r+1, c, t),             # Move south
        (1, r, c+1, t),             # Move east
        (1, r, c-1, t),             # Move west
        (7, r, c, (t+1)%3),         # Tool change
        (7, r, c, (t+2)%3),         # Tool change
    ]

def dijkstra(cave, init, goal):
    (rmax, cmax) = cave.shape
    (r0, c0, t0) = init
    dist = {init: 0}                # For each state, minimum distance from source
    next = [(0, r0, c0, t0)]        # Min-heap for Dijkstra search queue
    while len(next) > 0:
        (cost, row, col, tool) = heappop(next)
        state = (row, col, tool)
        if cost > dist[state]: continue         # Already found a better path?
        if state == goal: return dist[state]    # Reached goal?
        for (dt, r, c, t) in neighbors(*state):
            # Legal room and tool combination?
            in_bounds = 0 <= r and r < rmax and 0 <= c and c < cmax
            new_room = cave[r,c] if in_bounds else ROOM_WALL
            if not t in COMPATIBILITY[new_room]: continue
            # Any benefit to following this path?
            new_cost = cost + dt
            new_state = (r, c, t)
            if new_cost < dist.get(new_state, 9999999):
                dist[new_state] = new_cost
                heappush(next, (new_cost, r, c, t))
    return None                                 # No solution?

def part1(depth, rtag, ctag, verbose = False):
    cave = erosion(depth, rtag, ctag, rtag, ctag)
    if verbose: debug(cave)
    return np.sum(np.sum(cave))

def part2(depth, rtag, ctag):
    cave = erosion(depth, rtag, ctag, 2*rtag, 2*ctag)
    init = (0, 0, TOOL_TORCH)
    goal = (rtag, ctag, TOOL_TORCH)
    return dijkstra(cave, init, goal)

if __name__ == '__main__':
    test = read_input('depth: 510\ntarget: 10,10')
    input = read_input(get_data(day=22, year=2018))
    assert(part1(*test) == 114)
    print(f'Part 1: {part1(*input)}')
    assert(part2(*test) == 45)
    print(f'Part 2: {part2(*input)}')
