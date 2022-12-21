# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy

# Return three functions:
#   1) O given L, R (e.g., O = L + R)
#   2) L given R, O (e.g., L = O - R)    
#   3) R given L, O (e.g., R = O - L)
def get_fn(op):
    if op == '+':
        return (lambda l, r: l + r,
                lambda r, o: o - r,
                lambda l, o: o - l)
    elif op == '-':
        return (lambda l, r: l - r,
                lambda r, o: r + o,
                lambda l, o: l - o)
    elif op == '*':
        return (lambda l, r: l * r,
                lambda r, o: o // r,
                lambda l, o: o // l)
    elif op == '/':
        return (lambda l, r: l // r,
                lambda r, o: r * o,
                lambda l, o: l // o)
    else:
        raise Exception(f'Bad operand: {x}')

# Parse plaintext problem input.
def read_input(input):
    monkeys = {}
    for line in input.splitlines():
        lbl = line[0:4]
        rem = line[6:]
        if len(rem) < 11:
            monkeys[lbl] = int(rem)
        else:
            aa = rem[0:4]
            op = rem[5]
            bb = rem[7:11]
            monkeys[lbl] = (aa, get_fn(op), bb)
    return monkeys

# Evaluate arithmetic, replacing solved values as we go.
def eval_cache(monkeys, node):
    m = monkeys.get(node)
    if m is None:
        return None         # Placeholder for solver
    elif isinstance(m, int):
        return m            # Simple integer
    else:
        (ll,fn,rr) = m      # Arithmetic operation
        l = eval_cache(monkeys, ll)
        r = eval_cache(monkeys, rr)
        if (l is None) or (r is None): return None
        monkeys[node] = fn[0](l, r)
        return monkeys[node]

# Assign unknowns to make node have a given value.
def make_equal(monkeys, node, val):
    assert (val is not None)
    m = monkeys[node]
    if m is None:
        monkeys[node] = val
    else:
        (ll,fn,rr) = monkeys[node]
        l = eval_cache(monkeys, ll)
        r = eval_cache(monkeys, rr)
        if l is None:
            make_equal(monkeys, ll, fn[1](r, val))
        else:
            make_equal(monkeys, rr, fn[2](l, val))

def part1(input):
    # Direct evaluation of the tree.
    monkeys = deepcopy(input)
    return eval_cache(monkeys, 'root')

def part2(input):
    # Mark specific node as unknown.
    monkeys = deepcopy(input)
    monkeys['humn'] = None
    # Which half contains the unknown?
    (ll,_,rr) = monkeys['root']
    l = eval_cache(monkeys, ll)
    r = eval_cache(monkeys, rr)
    if l is None:
        make_equal(monkeys, ll, r)
    else:
        make_equal(monkeys, rr, l)
    return monkeys['humn']

TEST = \
'''
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=21, year=2022))
    assert(part1(test) == 152)
    print(f'Part 1: {part1(input)}')
    assert(part2(test) == 301)
    print(f'Part 2: {part2(input)}')
