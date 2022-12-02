# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

def read_input(filename):
    with open(filename, 'r') as file:
        lines = file.readlines()
    return [(line[0], line[2]) for line in lines]

def outcome1(them, us):
    # Them: A/B/C = Rock/Paper/Scissor
    # Us:   X/Y/Z = Rock/Paper/Scissor
    if us == 'X' and them == 'A': return 1 + 3
    if us == 'X' and them == 'B': return 1 + 0
    if us == 'X' and them == 'C': return 1 + 6
    if us == 'Y' and them == 'A': return 2 + 6  
    if us == 'Y' and them == 'B': return 2 + 3
    if us == 'Y' and them == 'C': return 2 + 0
    if us == 'Z' and them == 'A': return 3 + 0
    if us == 'Z' and them == 'B': return 3 + 6
    if us == 'Z' and them == 'C': return 3 + 3
    raise Exception('Invalid game input.')

def outcome2(them, us):
    # Them: A/B/C = Rock/Paper/Scissor
    # Us:   X/Y/Z = Lose/Draw/Win
    if us == 'X' and them == 'A': return 0 + 3  # Scissor
    if us == 'X' and them == 'B': return 0 + 1  # Rock
    if us == 'X' and them == 'C': return 0 + 2  # Paper
    if us == 'Y' and them == 'A': return 3 + 1  # Rock
    if us == 'Y' and them == 'B': return 3 + 2  # Paper
    if us == 'Y' and them == 'C': return 3 + 3  # Scissor
    if us == 'Z' and them == 'A': return 6 + 2  # Paper
    if us == 'Z' and them == 'B': return 6 + 3  # Scissor
    if us == 'Z' and them == 'C': return 6 + 1  # Rock
    raise Exception('Invalid game input.')

def part1(guide):
    return sum([outcome1(us,them) for (us,them) in guide])

def part2(guide):
    return sum([outcome2(us,them) for (us,them) in guide])

if __name__ == '__main__':
    test = read_input('../input/test02.txt')
    input = read_input('../input/input02.txt')
    assert(part1(test) == 15)
    assert(part2(test) == 12)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
