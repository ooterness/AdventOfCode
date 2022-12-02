# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

def read_input(input):
    return [line.strip() for line in input.splitlines()]

def count_letters(line):
    '''Count number of times each letter appears.'''
    counts = {}
    for ch in line:
        counts[ch] = counts.get(ch, 0) + 1
    return counts

def similar(line1, line2):
    '''Test if two lines are exactly one character different.'''
    diff = 0
    for n,ch in enumerate(line1):
        if ch != line2[n]: diff += 1
    return (diff == 1)

def common(line1, line2):
    '''Return common characters in two similar lines.'''
    common = ''
    for n,ch in enumerate(line1):
        if ch == line2[n]: common += ch
    return common

def part1(lines):
    '''Product of the number of boxes with pairs and triplets.'''
    count2 = 0
    count3 = 0
    for line in lines:
        counts = count_letters(line)
        if 2 in counts.values(): count2 += 1
        if 3 in counts.values(): count3 += 1
    return count2 * count3

def part2(lines):
    '''Find common characters in the closest box IDs.'''
    for a in range(len(lines)):
        for b in range(a+1, len(lines)):
            if similar(lines[a], lines[b]):
                return common(lines[a], lines[b])
    return ''   # No similar boxes found

TEST1 = \
'''
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
'''

TEST2 = \
'''
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
'''

if __name__ == '__main__':
    test1 = read_input(TEST1.strip())
    test2 = read_input(TEST2.strip())
    input = read_input(get_data(day=2, year=2018))
    assert (part1(test1) == 12)
    assert (part2(test2) == 'fgij')
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
