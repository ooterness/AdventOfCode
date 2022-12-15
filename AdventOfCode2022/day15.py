# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
import re

def read_input(input):
    numbers = lambda line: [int(x) for x in re.findall('[0-9\-]+', line)]
    return [numbers(line) for line in input.splitlines()]

def count_beacons(sensors, row):
    beacons = [bx for (sx,sy,bx,by) in sensors if by == row]
    return len(set(beacons))            # Count unique beacons

def scan_row(sensors, row, max_xy):
    scans = []
    for (sx, sy, bx, by) in sensors:
        rr = abs(sx-bx) + abs(sy-by) - abs(sy - row)
        if rr < 0: continue             # Any tiles this row?
        x0 = sx - rr; x1 = sx + rr      # Define search radius
        if max_xy:                      # Limit search range?
            x0 = max(x0, 0)
            x1 = min(x1, max_xy)
        scans.append((x0, x1))          # Add to list
    if len(scans) == 0: return []
    scans.sort()                        # Sort in-place
    merged = [scans[0]]                 # Merged list of scans
    for (x0,x1) in scans[1:]:
        (p0,p1) = merged[-1]
        if x0 <= p1:                    # Ranges overlap?
            p2 = max(x1, p1)
            merged[-1] = (p0,p2)        # Extend segment
        else:
            merged.append((x0,x1))      # New segment
    return merged
    
def count_row(sensors, row):
    return sum([1+x1-x0 for (x0,x1) in scan_row(sensors, row, None)])

def part1(sensors, row):
    return count_row(sensors, row) - count_beacons(sensors, row)

def part2(sensors, max_xy):
    for row in range(max_xy):
        scan = scan_row(sensors, row, max_xy)
        if len(scan) < 2: continue
        col = scan[0][1] + 1            # First gap
        return 4000000 * col + row
    return 0                            # No solution?

TEST = \
'''
Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=15, year=2022))
    assert(part1(test, 10) == 26)
    assert(part2(test, 20) == 56000011)
    print(f'Part 1: {part1(input, 2000000)}')
    print(f'Part 2: {part2(input, 4000000)}')
