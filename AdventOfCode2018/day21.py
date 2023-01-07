# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from opcodes import Program, noop

def part1(prog):
    # Only instruction #28 depends on the initial state.
    # (eqrr 1 0 2 --> Terminate if Reg1 = Reg0)
    reg = [0, 0, 0, 0, 0, 0]
    ctr = 0
    while ctr != 28:
        ctr = prog.step(reg, ctr)
    # Fastest initial state is the one that matches Reg1.
    return reg[1]

def part2(prog):
    # Simulate program until the state repeats.
    # TODO: This method works but takes about an hour.
    reg1_hist = set()
    reg1_prev = None
    reg = [0, 0, 0, 0, 0, 0]
    ctr = 0
    while True:
        # Breakpoint after instruction #28...
        ctr = prog.step(reg, ctr)
        if ctr == 28:
            # Have we looped around?
            if reg[1] in reg1_hist:
                return reg1_prev
            reg1_hist.add(reg[1])
            reg1_prev = reg[1]
            # Override the state so it keeps running.
            reg[2] = 0

if __name__ == '__main__':
    input = Program(get_data(day=21, year=2018))
    input.prog
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
