# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def unique(x):
    return len(x) == len(set(x))

def scan(input, count):
    for n in range(len(input) + 1 - count):
        if unique(input[n:n+count]): return n + count
    return 0

def part1(input):
    return scan(input, 4)

def part2(input):
    return scan(input, 14)

TEST1 = 'mjqjpqmgbljsphdztnvjfqwrcgsmlb'
TEST2 = 'bvwbjplbgvbhsrlpgdmjqwftvncz'
TEST3 = 'nppdvjthqldpwncqszvftbrmjlhg'
TEST4 = 'nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg'
TEST5 = 'zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw'

if __name__ == '__main__':
    input = get_data(day=6, year=2022)
    assert(part1(TEST1) == 7)
    assert(part1(TEST2) == 5)
    assert(part1(TEST3) == 6)
    assert(part1(TEST4) == 10)
    assert(part1(TEST5) == 11)
    assert(part2(TEST1) == 19)
    assert(part2(TEST2) == 23)
    assert(part2(TEST3) == 23)
    assert(part2(TEST4) == 29)
    assert(part2(TEST5) == 26)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
