# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import numpy as np
from aocd import get_data
from scipy.signal import convolve2d

def pmatrix(k, size=300):
    t = np.arange(size) + 1
    x = np.tile(t, (size,1))
    y = x.transpose()
    r = x + 10
    p = (r * y + k) * r
    return (p % 1000) // 100 - 5

def pwindow(k, window):
    w = np.ones((window,window), dtype='int32')
    p = convolve2d(pmatrix(k), w, mode='valid')
    (r,c) = np.unravel_index(p.argmax(), p.shape)
    return (c+1, r+1, p[r,c])

def part1(k):
    (c, r, p) = pwindow(k, 3)
    return (c, r)

def part2(k):
    # Iterating over the full size (1-300) takes forever.
    # Can we prove the optimal size is always below a threshold?
    (cc, rr, ww, pp) = (0, 0, 0, 0)
    for w in range(1,20): # TODO: This works, but why?
        (c, r, p) = pwindow(k, w)
        if p >= pp: (cc, rr, ww, pp) = (c, r, w, p)
    return (cc, rr, ww)

if __name__ == '__main__':
    input = int(get_data(day=11, year=2018))
    assert (part1(18) == (33,45))
    assert (part1(42) == (21,61))
    assert (part2(18) == (90,269,16))
    assert (part2(42) == (232,251,12))
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
