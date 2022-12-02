# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(input):
    # Put lines in chronological order.
    lines = sorted(input.splitlines())
    # Find all numbers on each line.
    read_line = lambda line: [int(x) for x in re.findall('[0-9]+', line)]
    events = [read_line(line) for line in lines]
    # Turn the raw event log into a list of times spent asleep.
    guard = None
    sleep = None
    result = []
    for evt in events:
        if len(evt) == 6:
            assert(sleep is None)
            guard = evt[5]  # Guard index
        elif sleep is None:
            sleep = evt[4]  # Sleep time
        else:
            wake = evt[4]   # Wake time
            result.append((guard, sleep, wake))
            sleep = None
    assert(sleep is None)
    return result

def part1(events):
    # Find the guard that spent the most time sleeping.
    total = {}
    for (guard,sleep,wake) in events:
        total[guard] = total.get(guard, 0) + (wake - sleep)
    target = max(total, key=total.get)
    # For that guard, find the sleepiest timeslot.
    times = [0] * 60
    for (guard,sleep,wake) in events:
        if guard != target: continue
        for t in range(sleep,wake):
            times[t] += 1
    minute = times.index(max(times))
    return target * minute

def part2(events):
    # Count sleep events for each unique guard/minute combo.
    counts = {}
    for (guard,sleep,wake) in events:
        for t in range(sleep,wake):
            counts[(guard,t)] = counts.get((guard,t),0) + 1
    # Find the guard/minute combo with the most sleep events.
    (guard,time) = max(counts, key=counts.get)
    return guard * time

TEST = \
'''
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=4, year=2018))
    assert (part1(test) == 240)
    assert (part2(test) == 4455)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
