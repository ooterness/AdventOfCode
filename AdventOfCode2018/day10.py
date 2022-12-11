# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(input):
    readline = lambda line: [int(x) for x in re.findall('[0-9\\-]+', line)]
    return [readline(line) for line in input.splitlines()]

def predict(input, t):
    x = [x0 + vx*t for [x0, y0, vx, vy] in input]
    y = [y0 + vy*t for [x0, y0, vx, vy] in input]
    return (x, y)

def render(xp, yp):
    result = []
    xx = range(min(xp), max(xp)+1)
    yy = range(min(yp), max(yp)+1)
    for y in yy:
        filter = [x for (n,x) in enumerate(xp) if yp[n] == y]
        render = ['#' if x in filter else '.' for x in xx]
        result.append(''.join(render))
    return '\n'.join(result)

def part1(input):
    '''Given initial position/velocity set, render the final message.'''
    (x, y) = predict(input, part2(input))
    return render(x, y)

def part2(input):
    '''Given initial position/velocity set, predict time to final message.'''
    score = 99999999; btime = None;
    for t in range(20000):
        (x, y) = predict(input, t)
        size = (max(x) - min(x)) * (max(y) - min(y))
        if (size < score): (score, btime) = (size, t)
    return btime

def contrast(text):
    return text.replace('#', '█').replace('.', ' ')

TEST = \
'''
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
'''

REF1 = \
'''
#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=10, year=2018))
    assert (part1(test) == REF1.strip())
    assert (part2(test) == 3)
    print(f'Part 1:\n{contrast(part1(input))}')
    print(f'Part 2: Time = {part2(input)}')
