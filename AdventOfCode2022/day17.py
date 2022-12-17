# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

BLOCKS = [                  # List of (R,C) relative to origin
    [(0,0), (0,1), (0,2), (0,3)],
    [(0,1), (1,0), (1,1), (1,2), (2,1)],
    [(0,0), (0,1), (0,2), (1,2), (2,2)],
    [(0,0), (1,0), (2,0), (3,0)],
    [(0,0), (0,1), (1,0), (1,1)],
]

class GameBlock:
    def __init__(self, bidx, maxh):
        self.p = BLOCKS[bidx % len(BLOCKS)]
        self.r = maxh + 3
        self.c = 2

    # Add self to the list of frozen blocks.
    def add(self, blocks):
        maxr = 0
        for (r,c) in self.p:
            maxr = max(maxr, self.r+r)
            blocks.add((self.r+r, self.c+c))
        return maxr + 1         # Height = Row-index + 1

    # Fall one row if possible. Returns True on collision.
    def fall(self, blocks):
        for (r,c) in self.p:
            r2 = self.r + r - 1
            c2 = self.c + c
            if r2 < 0:
                return True     # Collision (floor)
            if (r2, c2) in blocks:
                return True     # Collision (blocks)
        self.r -= 1             # Fall one row
        return False            # No collision

    # Slide left or right if possible. Returns True on collision.
    def slide(self, blocks, delta):
        for (r,c) in self.p:
            r2 = self.r + r
            c2 = self.c + c + delta
            if c2 < 0 or c2 > 6:
                return True     # Collision (sides)
            if (r2, c2) in blocks:
                return True     # Collision (blocks)
        self.c += delta         # Slide one space
        return False            # No collision

class GameState:
    def __init__(self, jets):
        self.jets = jets    # Jet sequence
        self.bidx = 0       # Block index
        self.jidx = 0       # Jet index
        self.maxh = 0       # Max height
        self.blocks = set() # Dropped tiles

    def debug(self):
        print(f'After block #{self.bidx}, height {self.maxh}')
        for r in reversed(range(self.maxh)):
            row = ['#' if (r,c) in self.blocks else ' ' for c in range(7)]
            print(f'|{"".join(row)}|')

    def next_jet(self):
        next = self.jets[self.jidx % len(self.jets)]
        self.jidx += 1
        return 1 if next == '>' else -1

    def drop_block(self):     
        block = GameBlock(self.bidx, self.maxh)
        self.bidx += 1
        while True:
            block.slide(self.blocks, self.next_jet())
            if block.fall(self.blocks): break
        self.maxh = max(self.maxh, block.add(self.blocks))

def part1(jets, verbose=False):
    game = GameState(jets)
    for b in range(2022):
        game.drop_block()
        if verbose: game.debug()
    return game.maxh

def part2(jets):
    None

TEST = '>>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>'

if __name__ == '__main__':
    input = get_data(day=17, year=2022)
    assert(part1(TEST) == 3068)
    #assert(part2(TEST) == 29)
    print(f'Part 1: {part1(input)}')
    #print(f'Part 2: {part2(input)}')
