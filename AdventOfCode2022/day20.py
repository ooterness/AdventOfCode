# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy

# Node in a doubly-linked list.
class Node:
    def __init__(self, value):
        self.prev  = None
        self.value = value
        self.next  = None

    def left(self, qty):
        node = self
        for n in range(qty): node = node.prev
        return node

    def right(self, qty):
        node = self
        for n in range(qty): node = node.next
        return node

    def mix(self, key, size):
        # Nothing to do if value is zero.
        if self.value == 0: return
        # Remove self from current position.
        self.prev.next = self.next
        self.next.prev = self.prev
        # Find the nodes before/after our new location.
        shift = abs(self.value * key) % (size-1)
        if self.value < 0:
            r = self.next.left(shift)
            l = r.prev
        else:
            l = self.prev.right(shift)
            r = l.next
        # Insert self between the two target nodes.
        l.next = self
        self.prev = l
        r.prev = self
        self.next = r

# Create a circular doubly-linked chain from a list.
def circle(input):
    out = [Node(x) for x in input]
    for (n,x) in enumerate(out):
        x.prev = out[(n-1) % len(out)]
        x.next = out[(n+1) % len(out)]
    return out  # List of nodes in original order

# Find the node with the given value.
def find_value(seq, value):
    for node in seq:
        if node.value == value: return node

def read_input(input):
    return [int(x) for x in input.splitlines()]

def part1(input):
    seq = circle(input)
    for node in seq: node.mix(1, len(input))
    n0 = find_value(seq, 0)
    n1 = n0.right(1000)
    n2 = n1.right(1000)
    n3 = n2.right(1000)
    return n1.value + n2.value + n3.value

def part2(input):
    key = 811589153
    seq = circle(input)
    for n in range(10):
        for node in seq: node.mix(key, len(input))
    n0 = find_value(seq, 0)
    n1 = n0.right(1000)
    n2 = n1.right(1000)
    n3 = n2.right(1000)
    return (n1.value + n2.value + n3.value) * key

TEST = '1\n 2\n -3\n 3\n -2\n 0\n 4'

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=20, year=2022))
    assert(part1(test) == 3)
    print(f'Part 1: {part1(input)}')
    assert(part2(test) == 1623178306)
    print(f'Part 2: {part2(input)}')
