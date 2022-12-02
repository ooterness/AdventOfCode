# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    return [int(line) for line in input.splitlines()]

def repeat_freq(delta):
    freq = 0
    seen = set([0])
    while True:
        for df in delta:
            freq += df
            if freq in seen: return freq
            seen.add(freq)

if __name__ == '__main__':
    input = read_input(get_data(day=1, year=2018))
    print(f'Part 1: {sum(input)}')
    assert (repeat_freq([+1, -1]) == 0)
    assert (repeat_freq([+3, +3, +4, -2, -4]) == 10)
    assert (repeat_freq([-6, +3, +8, +5, -6]) == 5)
    assert (repeat_freq([+7, +7, -2, -7, -4]) == 14)
    print(f'Part 2: {repeat_freq(input)}')
