# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

class Tree:
    def __init__(self, iter):
        num_child = next(iter)
        num_meta  = next(iter)
        self.child = [Tree(iter) for n in range(num_child)]
        self.meta  = [next(iter) for n in range(num_meta)]

    def part1(self):
        return sum(self.meta) + sum([c.part1() for c in self.child])

    def part2(self):
        if len(self.child) > 0:
            return sum([self.child[n-1].part2() for n in self.meta
                if 0 < n and n <= len(self.child)])
        else:
            return sum(self.meta)

def read_input(input):
    raw = [int(x) for x in input.split(' ')]
    return Tree(iter(raw))

TEST = '2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2'

if __name__ == '__main__':
    test = read_input(TEST)
    input = read_input(get_data(day=8, year=2018))
    assert (test.part1() == 138)
    assert (test.part2() == 66)
    print(f'Part 1: {input.part1()}')
    print(f'Part 2: {input.part2()}')
