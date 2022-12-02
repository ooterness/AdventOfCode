# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re

def read_input(filename):
    # Put lines in chronological order.
    with open(filename, 'r') as file:
        lines = sorted(file.readlines())
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

if __name__ == '__main__':
    test = read_input('../input/test04.txt')
    input = read_input('../input/input04.txt')
    assert (part1(test) == 240)
    assert (part2(test) == 4455)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
