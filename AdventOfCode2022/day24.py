# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy

DIRECTIONS = {'W':(0,0), '^':(-1,0), '>':(0,+1), 'v':(+1,0), '<':(0,-1)}

class Blizzards:
    def __init__(self, input):
        # Read the map string.
        blizzards = []      # List of tuple (r,c,d)
        self.tiles = set()  # Set of tuple (r,c)
        for (r,row) in enumerate(input.splitlines()):
            for (c,col) in enumerate(row):
                if col == '#': continue
                self.tiles.add((r,c))
                if col in DIRECTIONS.keys():
                    blizzards.append((r,c,col))
        # Find the start and goal locations.
        self.start = min(self.tiles)
        self.goal  = max(self.tiles)
        # For each blizzard, form a list of visited locations.
        self.blizz = []     # List of list of tuple (r,c)
        for (r0,c0,d) in blizzards:
            # Movement direction for this blizzard?
            (dr,dc) = DIRECTIONS[d]
            # Scan forward and backward until we hit a wall.
            fwd = 0; rev = 1
            while (r0+fwd*dr, c0+fwd*dc) in self.tiles: fwd += 1
            while (r0-rev*dr, c0-rev*dc) in self.tiles: rev += 1
            fpath = [(r0+n*dr, c0+n*dc) for n in range(fwd)]
            rpath = [(r0-n*dr, c0-n*dc) for n in range(1,rev)]
            # Store the complete path for this blizzard.
            self.blizz.append(fpath + list(reversed(rpath)))

    def predict(self, t):
        # Predict blizzard locations at given time.
        return (t, set([b[t%len(b)] for b in self.blizz]))

# Breadth first search for the fastest path.
def bfs(input, t0, start, goal):
    init  = (t0,start[0],start[1])
    queue = [init]
    visit = set()
    blizz = (t0, None)
    while len(queue) > 0:
        (t,r,c) = queue.pop(0)
        if t+1 != blizz[0]:     # Cache blizzard locations
            blizz = input.predict(t+1)
            visit = set()       # Clear obsolete cache
        for (dr,dc) in DIRECTIONS.values():
            next = (t+1, r+dr, c+dc)
            npos = (r+dr, c+dc)
            if next in visit: continue
            if npos in blizz[1]: continue
            if npos == goal: return t+1
            if npos not in input.tiles: continue
            queue.append(next)
            visit.add(next)

def part1(input):
    return bfs(input, 0, input.start, input.goal)

def part2(input):
    t1 = bfs(input, 0, input.start, input.goal)
    t2 = bfs(input, t1, input.goal, input.start)
    return bfs(input, t2, input.start, input.goal)

TEST = \
'''
#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
'''

if __name__ == '__main__':
    test = Blizzards(TEST[1:])
    input = Blizzards(get_data(day=24, year=2022))
    assert(part1(test) == 18)
    print(f'Part 1: {part1(input)}')
    assert(part2(test) == 54)
    print(f'Part 2: {part2(input)}')
