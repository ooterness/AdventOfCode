# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from functools import cmp_to_key

def read_input(input):
    lines = input.splitlines()
    count = (len(lines) + 1) // 3
    return [(eval(lines[3*n+0]), eval(lines[3*n+1]))
        for n in range(count)]

def compare(left, right):
    # Handle the simple cases first:
    li = isinstance(left, int)
    ri = isinstance(right, int)
    if li and ri:   # Both Integers
        if left < right: return -1
        if left > right: return +1
        return 0
    elif li:        # Integer + List
        return compare([left], right)
    elif ri:        # List + Integer
        return compare(left, [right])
    # Handle the List + List case:
    ll = len(left)
    rr = len(right) 
    for n in range(min(ll,rr)):
        cmp = compare(left[n], right[n])
        if cmp: return cmp
    return ll - rr

def part1(pairs):
    total = 0
    for (n,pair) in enumerate(pairs):
        cmp = compare(*pair)
        if cmp < 0: total += n+1
    return total

def part2(pairs):
    # Form a list with all the packets.
    DIV1 = [[2]]; DIV2 = [[6]]
    packets = [DIV1, DIV2]
    for pair in pairs: packets.extend(pair)
    # In-place sort.
    packets.sort(key = cmp_to_key(compare))
    # Find the index of each divider packet.
    idx1 = packets.index(DIV1)
    idx2 = packets.index(DIV2)
    return (idx1+1) * (idx2+1)

TEST = \
'''
[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=13, year=2022))
    assert(part1(test) == 13)
    assert(part2(test) == 140)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
