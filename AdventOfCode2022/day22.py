# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
from math import sqrt
import numpy as np

# Right, Down, Left, Up
DIRECTIONS = [(0,+1), (+1,0), (0,-1), (-1,0)]

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
        self.side = int(round(sqrt(len(tiles) // 6)))
        self.tmap = {}
        for t in deepcopy(tiles): self.tmap[(t.r, t.c)] = t

    # Find the starting tile.
    def start(self, ignore_walls=False):
        r0 = 1
        c0 = min([t.c
            for t in self.tmap.values()
            if (t.r == r0) and (ignore_walls or not t.w)])
        return self.tmap[(r0, c0)]

    # Create crosslinks with wraparound (2D rules)
    def grid_link(self):
        for t in self.tmap.values():
            t.adj = [self.grid_scan(t, d) for d in range(4)]

    # Adjacency scan with wraparound (2D rules)
    def grid_scan(self, tile, d):
        # First check regular adjacency.
        (dr,dc) = DIRECTIONS[d]
        r = tile.r + dr; c = tile.c + dc;
        if (r, c) in self.tmap:
            return (d, self.tmap[(r, c)])
        # Otherwise, wrap to the opposite edge.
        # (Only check at side-length intervals.)
        dr *= self.side; dc *= self.side
        while (r-dr, c-dc) in self.tmap:
            r -= dr; c -= dc
        return (d, self.tmap[(r, c)])

    # Create crosslinks with wraparound (3D rules)
    def cube_link(self):
        # Find X,Y,Z coordinates for the outer perimeter.
        edges = self.cube_edges()
        # Form direct + indirect crosslinks.
        for t in self.tmap.values():
            t.adj = [self.cube_scan(t, d, edges) for d in range(4)]

    # Generate a list of folded of X,Y,Z coordinates (d0, d1, rc, xyz)
    # for each of the tiles along the map perimeter.
    def cube_edges(self):
        # Start from the upper-left corner of the map.
        init = self.start(True)
        (r,c,d) = (init.r, init.c, 0)
        # Define a clockwise circuit of the perimeter.
        xyz = np.array([1,0,0])     # XYZ of virtual edge
        ref = np.array([1,1,0])     # XYZ of reference tile
        vel = np.array([1,0,0])     # XYZ current velocity
        # Perimeter always has 14 equal-length segments...
        edges = []
        for seg in range(14):
            # Advance along the current edge.
            (dr,dc) = DIRECTIONS[d]
            d0 = (d - 1) % 4      # Direction towards edge
            d1 = (d + 1) % 4      # Direction away from edge
            for n in range(self.side):
                edges.append((d0, d1, self.tmap[(r,c)], xyz + n*vel))
                r += dr; c += dc
            xyz += vel * self.side
            ref += vel * self.side
            # Have we reached a corner in the map?
            (dr,dc) = DIRECTIONS[(d-1)%4]
            chk_l = (r+dr,c+dc) in self.tmap
            chk_r = (r,c) in self.tmap
            if chk_l and chk_r:     # Left turn
                d = (d-1) % 4       # Turn+step in 2D
                (dr,dc) = DIRECTIONS[d]
                r += dr; c += dc
                tmp = np.cross(vel, ref - xyz)
                vel = -vel
                xyz += vel
                ref = xyz + tmp
            elif chk_r:             # Straight line
                vel = np.cross(vel, ref - xyz)
                xyz += vel
                ref += vel
            else:                   # Right turn
                (dr,dc) = DIRECTIONS[d]
                r -= dr; c -= dc
                d = (d+1) % 4
                tmp = ref - xyz
                xyz += tmp
                ref -= vel
                vel = tmp
        # Sanity check that we've completed a loop.
        assert (all(xyz == np.array([1,0,0])))
        assert (all(ref == np.array([1,1,0])))
        assert (all(vel == np.array([1,0,0])))
        return edges

    # Adjacency scan with wraparound (3D rules)
    def cube_scan(self, tile, d, edges):
        # First check regular adjacency.
        (dr,dc) = DIRECTIONS[d]
        r = tile.r + dr; c = tile.c + dc;
        if (r, c) in self.tmap:
            return (d, self.tmap[(r, c)])
        # Check for another edge with the same X,Y,Z coordinates.
        rc0 = (tile.r, tile.c)
        xyz0 = [xyz for (d0, d1, t, xyz) in edges
                if t == tile and d == d0][0]
        return [(d1, t) for (d0, d1, t, xyz) in edges
                if t != tile and all(xyz == xyz0)][0]

# Parse raw problem input into a list of Tile objects and a move-list.
def read_input(input):
    moves = []
    tiles = []
    lines = input.splitlines()
    # Read the map portion.
    for (r,line) in enumerate(lines[:-2]):
        for (c,ch) in enumerate(line):
            if ch == ' ': continue
            tile = Tile(r, c, ch == '#')
            tiles.append(tile)
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
    return (moves, tiles)

# Execute the designated move sequence.
def execute(start, moves):
    posn = start
    face = 0
    for move in moves:
        if isinstance(move, int):
            for n in range(move):
                if posn.adj[face][1].w: break
                (face, posn) = posn.adj[face]
        elif move == 'L':
            face = (face - 1) % 4
        elif move == 'R':
            face = (face + 1) % 4
        else:
            raise Exception('Unknown move')
    return (posn, face)

def part1(moves, tiles):
    # Create a crosslinked map.
    grid = TileGrid(tiles)
    grid.grid_link()
    # Execute the move sequence.
    (posn, face) = execute(grid.start(), moves)
    # Report final position.
    return 1000 * posn.r + 4 * posn.c + face

def part2(moves, tiles):
    # Create a crosslinked map.
    cube = TileGrid(tiles)
    cube.cube_link()
    # Execute the move sequence.
    (posn, face) = execute(cube.start(), moves)
    # Report final position.
    return 1000 * posn.r + 4 * posn.c + face

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
    assert(part2(*test) == 5031)
    print(f'Part 2: {part2(*input)}')
