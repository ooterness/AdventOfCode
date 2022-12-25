# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

SNAFU2DEC = {'=':-2, '-':-1, '0':0, '1':1, '2':2}
DEC2SNAFU = {-2:'=', -1:'-', 0:'0', 1:'1', 2:'2'}

# Convert Base-5 SNAFU string to an integer.
def snafu2dec(in_str):
    result = 0
    for ch in in_str:
        result = 5*result + SNAFU2DEC[ch]
    return result

# Convert integer to Base-5 SNAFU string.
def dec2snafu(in_dec):
    if in_dec == 0: return '0'
    result = ''
    while in_dec != 0:
        digit = (in_dec + 2) % 5 - 2
        in_dec = (in_dec - digit) // 5
        result = DEC2SNAFU[digit] + result
    return result

def read_input(input):
    return [snafu2dec(line) for line in input.splitlines()]

def part1(input):
    return dec2snafu(sum(input))

TEST = \
'''
1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=25, year=2022))
    assert(sum(test) == 4890)
    assert(dec2snafu(12345) == '1-0---0')
    assert(dec2snafu(314159265) == '1121-1110-1=0')
    assert(part1(test) == '2=-1=0')
    print(f'Part 1: {part1(input)}')
    #assert(part2(test) == 54)
    #print(f'Part 2: {part2(input)}')
