# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

DIR = {'U': ( 0, +1),
       'R': (+1,  0),
       'D': ( 0, -1),
       'L': (-1,  0)}

# Basic arithmetic on XY tuples:
def add(z1, z2):
    return (z1[0] + z2[0], z1[1] + z2[1])
def sub(z1, z2):
    return (z1[0] - z2[0], z1[1] - z2[1])
def dist(z1, z2):
    return max([abs(z) for z in sub(z1, z2)])
def sign(x):    # Not built-in!?
    return 1 if x > 0 else 0 if x == 0 else -1
def unit(z1, z2):
    return (sign(z1[0] - z2[0]), sign(z1[1] - z2[1]))

# Convert input to a list of tuples with direction and number of steps.
def read_input(input):
    return [(DIR[line[0]], int(line[2:])) for line in input.splitlines()]

# Move tail to keep up with head.
def move(head, tail):
    if dist(head, tail) < 2:
        return tail
    else:
        return add(tail, unit(head, tail))

def part1(cmds):
    head = (0, 0)
    tail = (0, 0)
    visited = set([tail])
    for (step, qty) in cmds:
        for n in range(qty):
            head = add(head, step)
            tail = move(head, tail)
            visited.add(tail)
    return len(visited)

def part2(cmds):
    rope = [(0, 0)] * 10
    visited = set(rope)
    for (step, qty) in cmds:
        for n in range(qty):
            rope[0] = add(rope[0], step)
            for r in range(1, len(rope)):
                rope[r] = move(rope[r-1], rope[r])
            visited.add(rope[-1])
    return len(visited)
        

TEST1 = \
'''
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
'''

TEST2 = \
'''
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
'''

if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    input = read_input(get_data(day=9, year=2022))
    assert(part1(test1) == 13)
    assert(part2(test1) == 1)
    assert(part2(test2) == 36)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
