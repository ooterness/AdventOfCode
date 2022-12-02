# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    elves = []
    for elf in input.split('\n\n'):
        food = elf.split('\n')
        elves.append([int(item) for item in food])
    return elves

def most_calories(elves):
    return max([sum(elf) for elf in elves])

def top_three(elves):
    top = sorted([sum(elf) for elf in elves])
    return sum(top[-3:])

TEST = \
'''
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=1, year=2022))
    assert (most_calories(test) == 24000)
    assert (top_three(test) == 45000)
    print(f'Part 1: {most_calories(input)}')
    print(f'Part 2: {top_three(input)}')
