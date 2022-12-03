# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

DOUBLETS = [chr(65+n) + chr(97+n) for n in range(26)] \
         + [chr(97+n) + chr(65+n) for n in range(26)]

def react(chem):
    while True:
        ref = len(chem)                     # Original length
        for d in DOUBLETS:                  # For each doublet (aA, bB, ...)
            chem = chem.replace(d, '')      # Eliminate all instances of doublet
        if ref == len(chem): return chem    # Stop if there were no eliminations

def part1(chem):
    return len(react(chem))                 # Part1 = Final length

def part2(chem):
    best = len(chem)
    for n in range(26):                     # Try removing each reagent
        chem2 = chem.replace(chr(65+n), '') \
                    .replace(chr(97+n), '')
        best = min(best, len(react(chem2)))
    return best

if __name__ == '__main__':
    test = 'dabAcCaCBAcCcaDA'
    input = get_data(day=5, year=2018).strip()
    assert (part1(test) == 10)
    assert (part2(test) == 4)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
