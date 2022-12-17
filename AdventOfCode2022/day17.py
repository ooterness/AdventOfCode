# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

# Each row is defined as a bit-mask, MSB = left.
# Blocks are defined as a list of rows, starting from the bottom.
BLOCKS = [
    [0b1111000],
    [0b0100000, 0b1110000, 0b0100000],
    [0b1110000, 0b0010000, 0b0010000],
    [0b1000000, 0b1000000, 0b1000000, 0b1000000],
    [0b1100000, 0b1100000],
]

class GameBlock:
    def __init__(self, bidx, maxh):
        self.p = BLOCKS[bidx % len(BLOCKS)]
        self.r = maxh + 3
        self.c = 2

    # Add self to the list of frozen blocks.
    def add(self, blocks):
        for (r,mask) in enumerate(self.p):
            r2 = self.r + r
            blocks[r2] = (mask >> self.c) | blocks.get(r2, 0)
        return self.r + len(self.p)

    # Fall one row if possible. Returns True on collision.
    def fall(self, blocks):
        for (r,mask) in enumerate(self.p):
            r2 = self.r + r - 1
            if r2 < 0:
                return True     # Collision (floor)
            if blocks.get(r2,0) & (mask >> self.c):
                return True     # Collision (blocks)
        self.r -= 1             # Fall one row
        return False            # No collision

    # Slide left or right if possible. Returns True on collision.
    def slide(self, blocks, delta):
        for (r,mask) in enumerate(self.p):
            r2 = self.r + r
            if delta < 0 and self.c == 0:
                return True     # Collision (left wall)
            if delta > 0 and (1 & (mask >> self.c)):
                return True     # Collision (right wall)
            if (mask >> self.c+delta) & blocks.get(r2, 0):
                return True     # Collision (blocks)
        self.c += delta         # Slide one space
        return False            # No collision

class GameState:
    def __init__(self, jets):
        self.jets = jets    # Jet sequence
        self.bidx = 0       # Block index
        self.jidx = 0       # Jet index
        self.maxh = 0       # Max height
        self.blocks = {}    # Dropped tiles (Bit mask by row)

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
