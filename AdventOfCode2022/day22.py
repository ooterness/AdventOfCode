# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

# Treat each grid square as part of a quad-linked-list.
class Tile:
    def __init__(self, r, c, w):
        self.r = r+1    # Row index (1-based)
        self.c = c+1    # Col index (1-based)
        self.w = w      # Solid wall?
        self.adj = [None, None, None, None]

# A crosslinked lookup table of Tiles.
class TileGrid:
    # Create an unlinked lookup table.
    def __init__(self, tiles):
        self.tmap = {}
        for t in tiles: self.tmap[t.r, t.c] = t

    # Create crosslinks with wraparound.
    def grid_link(self):
        for t in self.tmap.values():
            r = self.grid_scan(t.r, t.c, 0, +1)
            d = self.grid_scan(t.r, t.c, +1, 0)
            l = self.grid_scan(t.r, t.c, 0, -1)
            u = self.grid_scan(t.r, t.c, -1, 0)
            t.adj = [r, d, l, u]

    # Adjacency scan with wraparound.
    def grid_scan(self, r0, c0, dr, dc):
        # First check regular adjacency.
        t = self.tmap.get((r0+dr, c0+dc), None)
        if t is not None: return t
        # Otherwise, wrap to the opposite edge.
        while self.tmap.get((r0-dr, c0-dc), None):
            r0 -= dr; c0 -= dc
        return self.tmap.get((r0, c0))

def read_input(input):
    start = None
    moves = []
    tiles = []
    lines = input.splitlines()
    # Read the map portion.
    for (r,line) in enumerate(lines[:-2]):
        for (c,ch) in enumerate(line):
            if ch == ' ': continue
            tile = Tile(r, c, ch == '#')
            tiles.append(tile)  # Add to the list
            if start is None and not tile.w:
                start = tile    # First open tile?
    # Read the instruction sequence.
    num = 0
    for ch in lines[-1]:
        if '0' <= ch and ch <= '9':
            num = 10*num + int(ch)
        else:
            moves.append(num)
            moves.append(ch)
            num = 0
    moves.append(num)
    return (start, moves, tiles)

def part1(start, moves, tiles):
    # Create a crosslinked map.
    grid = TileGrid(tiles)
    grid.grid_link()
    # Execute the move sequence.
    posn = start
    face = 0
    for move in moves:
        if isinstance(move, int):
            for n in range(move):
                if posn.adj[face].w: break
                posn = posn.adj[face]
        elif move == 'L':
            face = (face - 1) % 4
        elif move == 'R':
            face = (face + 1) % 4
        else:
            raise Exception('Unknown move')
    # Report final position.
    return 1000 * posn.r + 4 * posn.c + face

def part2(start, moves, tiles):
    None

TEST = \
'''
        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
'''

if __name__ == '__main__':
    test = read_input(TEST[1:])
    input = read_input(get_data(day=22, year=2022))
    assert(part1(*test) == 6032)
    print(f'Part 1: {part1(*input)}')
    #assert(part2(*test) == 301)
    #print(f'Part 2: {part2(*input)}')
