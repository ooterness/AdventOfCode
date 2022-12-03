# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data
from collections import defaultdict

def read_input(input):
    lines = [(line[5],line[36]) for line in input.splitlines()]
    nodes = set()
    prereq = defaultdict(list)
    for (pre, post) in lines:
        nodes.add(pre)
        nodes.add(post)
        prereq[post].append(pre)
    return (sorted(nodes), prereq)

def next_task(steps, prereq, start, done):
    # Scan through steps in alphabetical order.
    # Execute the first one that has all its prereqs met.
    for step in steps:
        if step in start: continue
        if all([x in done for x in prereq[step]]):
            return step
    return None

def get_duration(step, basetime):
    return basetime + ord(step) - ord('A')

def part1(steps, prereq):
    result = ''
    # Keep going until we've executed every step.
    while len(result) < len(steps):
        result += next_task(steps, prereq, result, result)
    return result

def part2(steps, prereq, workers, basetime):
    start = ''  # List of steps where work has begun
    done = ''   # List of steps where work is completed
    time = 0    # Total time spent
    work = [None] * workers
    wrem = [0] * workers
    while len(done) < len(steps):
        for n in range(workers):
            if wrem[n] > 0:     # Countdown until finished...
                wrem[n] -= 1
            else:               # Done/idle
                if work[n]:     # Previous task finished?
                    done += work[n]
                    work[n] = None
                step = next_task(steps, prereq, start, done)
                if step:        # Start new task?
                    start += step
                    work[n] = step
                    wrem[n] = get_duration(step, basetime)
        time += 1
    return time - 1

TEST = \
'''
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=7, year=2018))
    assert (part1(*test) == 'CABDFE')
    assert (part2(*test, 2, 0) == 15)
    print(f'Part 1: {part1(*input)}')
    print(f'Part 2: {part2(*input, 5, 60)}')
