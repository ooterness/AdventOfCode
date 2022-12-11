# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    return [None if line.startswith('noop') else int(line[5:])
            for line in input.splitlines()]

def simulate(program):
    (t, x) = (1, 1)
    xval = {t: x}
    for line in program:
        if line is None:
            t += 1
            xval[t] = x
        else:
            t += 1
            xval[t] = x
            t += 1
            x += line
            xval[t] = x
    return xval

def part1(program):
    xval = simulate(program)
    return sum([t * xval[t] for t in [20, 60, 100, 140, 180, 220]])

def pixel(xval, r, c):
    x = xval[40 * r + c + 1]
    return '#' if c-1 <= x and x <= c+1 else '.'

def pline(xval, r):
    return ''.join([pixel(xval, r, c) for c in range(40)])

def part2(program):
    xval = simulate(program)
    return '\n'.join([pline(xval, r) for r in range(6)])

def contrast(text):
    return text.replace('#', '█').replace('.', ' ')

CRT_TEST = \
'''
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
'''

if __name__ == '__main__':
    test = read_input(open('day10.txt').read())
    input = read_input(get_data(day=10, year=2022))
    assert(part1(test) == 13140)
    assert(part2(test) == CRT_TEST.strip())
    print(f'Part 1: {part1(input)}')
    print(f'Part 2:\n{contrast(part2(input))}')
