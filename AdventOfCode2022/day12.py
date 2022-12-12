# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from heapq import heappop, heappush

def read_input(input):
    height = {}
    for (r,row) in enumerate(input.splitlines()):
        for (c,ch) in enumerate(row):
            rc = (r,c)
            if ch == 'S':   height[rc] = 0;     start = rc
            elif ch == 'E': height[rc] = 25;    final = rc
            else:           height[rc] = ord(ch) - ord('a')
    return (height, start, final)

def neighbors(r, c):
    return [(r-1,c), (r,c-1), (r,c+1), (r+1,c)]

def dijkstra(height, final):
    # Initial state.
    UNREACHABLE = 999999
    dist = {}   # For each grid, minimum distance from source
    prev = {}   # For each grid, optimal previous step
    next = []   # Min-heap for search queue
    for rc in height.keys():
        cost = 0 if rc == final else UNREACHABLE
        dist[rc] = cost
        prev[rc] = None
        heappush(next, (cost,rc))
    # Run algorithm.
    while len(next) > 0:
        (cost,node) = heappop(next)
        if cost > dist[node]: continue      # Already found a better path?
        for rc in neighbors(*node):
            delta = height[node] - height.get(rc, -UNREACHABLE)
            if delta > 1: continue          # Too steep to (un)climb
            new_cost = cost + 1             # Fixed cost (for now)
            if new_cost < dist[rc]:
                dist[rc] = new_cost
                prev[rc] = node
                heappush(next, (new_cost,rc))
    return dist, prev

def part1(height, start, final):
    (dist, prev) = dijkstra(height, final)
    return dist[start]

def part2(height, start, final):
    (dist, prev) = dijkstra(height, final)
    return min([dist[rc] for (rc,h) in height.items() if h == 0])

TEST = \
'''
Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=12, year=2022))
    assert(part1(*test) == 31)
    assert(part2(*test) == 29)
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input)}')
