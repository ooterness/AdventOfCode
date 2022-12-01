# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

def read_input(filename):
    with open(filename, 'r') as file:
        lines = file.read().strip()
    elves = []
    for elf in lines.split('\n\n'):
        food = elf.split('\n')
        elves.append([int(item) for item in food])
    return elves

def most_calories(elves):
    return max([sum(elf) for elf in elves])

def top_three(elves):
    top = sorted([sum(elf) for elf in elves])
    return sum(top[-3:])

if __name__ == '__main__':
    test = read_input('../input/test01.txt')
    input = read_input('../input/input01.txt')
    assert (most_calories(test) == 24000)
    assert (top_three(test) == 45000)
    print(f'Part 1: {most_calories(input)}')
    print(f'Part 2: {top_three(input)}')
