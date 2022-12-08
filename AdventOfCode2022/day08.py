# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
import numpy as np

def read_input(input):
    return np.array([[int(ch) for ch in line] for line in input.splitlines()])

def part1(mat):
    (rows, cols) = mat.shape
    hidden = 0
    for rr in range(1, rows-1):
        for cc in range(1, cols-1):
            hide_u = any(mat[:rr,cc]   >= mat[rr,cc])
            hide_d = any(mat[rr+1:,cc] >= mat[rr,cc])
            hide_l = any(mat[rr,:cc]   >= mat[rr,cc])
            hide_r = any(mat[rr,cc+1:] >= mat[rr,cc])
            if hide_u and hide_d and hide_l and hide_r: hidden += 1
    return rows * cols - hidden

def scan(vec, ctr):
    lo = ctr - 1; hi = ctr + 1
    while lo > 0:
        if vec[lo] >= vec[ctr]: break
        lo -= 1
    while hi < len(vec)-1:
        if vec[hi] >= vec[ctr]: break
        hi += 1
    return (ctr-lo, hi-ctr)

def part2(mat):
    (rows, cols) = mat.shape
    score = 0
    for rr in range(1, rows-1):
        for cc in range(1, cols-1):
            (cu, cd) = scan(mat[:,cc], rr)
            (cl, cr) = scan(mat[rr,:], cc)
            score = max(score, cu * cd * cl * cr)
    return score

TEST = \
'''
30373
25512
65332
33549
35390
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=8, year=2022))
    assert(part1(test) == 21)
    assert(part2(test) == 8)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
