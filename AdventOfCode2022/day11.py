# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
from numpy import product
import re

'''Read all numbers from a line of text, ignoring everything else.'''
def parse_numeric(line):
    return [int(x) for x in re.findall('[0-9]+', line)]

'''Create a lambda function that applies a monkey's "operation".
   e.g., "Operation: new = old + 7" --> lambda old: old + 7'''
def parse_operation(line):
    is_adder    = '+' in line
    fixed_val   = parse_numeric(line)
    if is_adder and len(fixed_val) > 0:
        return lambda old: old + fixed_val[0]
    elif is_adder:
        return lambda old: old + old
    elif len(fixed_val) > 0:
        return lambda old: old * fixed_val[0]
    else:
        return lambda old: old * old

'''Create a lambda function that applies an "if divisible..." criterion.
   e.g., "Test: divisible by 13..." --> lambda val: x if new else y'''
def parse_target(lines):
    divisor     = parse_numeric(lines[0])[0]
    val_true    = parse_numeric(lines[1])[0]
    val_false   = parse_numeric(lines[2])[0]
    return lambda val: val_true if val % divisor == 0 else val_false

class Monkey:
    def __init__(self, lines):
        numeric = lambda line: [int(x) for x in re.findall('[0-9]+', lines[1])]
        self.label  = lines[0]                  # Human-readable label
        self.items  = parse_numeric(lines[1])   # Starting items
        self.oper   = parse_operation(lines[2]) # Operation
        self.divide = parse_numeric(lines[3])   # Divider for test
        self.target = parse_target(lines[3:])   # Target function
        self.count  = 0                         # Items inspected

    def debug(self):
        print(f'{self.label} {self.items}')

'''Create a list of Monkey objects from text input.'''
def read_input(input):
    lines = input.splitlines()
    count = (len(lines) + 1) // 7
    return [Monkey(lines[7*n:]) for n in range(count)]

'''Run a full round of monkey business. Updates in-place.'''
def simulate(monkeys, lcm):
    for monkey in monkeys:
        monkey.count += len(monkey.items)
        for item in monkey.items:
            new_val = monkey.oper(item)
            if lcm is None: new_val = new_val // 3
            else:           new_val = new_val % lcm
            target  = monkey.target(new_val)
            monkeys[target].items.append(new_val)
        monkey.items = []

'''Call "simulate" N times from the given initial state,
   counting events to calculate the monkey-business score.'''
def run(init, iter, div3, verbose):
    monkeys = deepcopy(init)
    lcm_or_divide = None if div3 else \
        int(product([m.divide for m in monkeys]))
    for n in range(iter):
        simulate(monkeys, lcm_or_divide)
        if verbose > 1:     # Print held items
            print(f'After round {n+1}:')
            [m.debug() for m in monkeys]
    if verbose > 0:         # Print inspection counts
        print(f'Inspections: {[m.count for m in monkeys]}')
    inspect = sorted([m.count for m in monkeys])
    return inspect[-1] * inspect[-2]

'''Return the Part 1 solution (20 iterations, divide by three).'''
def part1(init, verbose=0):
    return run(init, 20, True, verbose)

'''Return the Part 2 solution (10k iterations, no divide).'''
def part2(init, verbose=0):
    return run(init, 10000, False, verbose)

TEST = \
'''
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=11, year=2022))
    assert(part1(test) == 10605)
    assert(part2(test) == 2713310158)
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
