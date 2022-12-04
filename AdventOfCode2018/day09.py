# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data
from llist import dllist

def read_input(line):
    return [int(x) for x in re.findall('[0-9]+', line)]

def shift(circle, node, offset):
    ptr = node
    while offset > 0:
        ptr = ptr.next
        offset -= 1
        if ptr is None: ptr = circle.first
    while offset < 0:
        ptr = ptr.prev
        offset += 1
        if ptr is None: ptr = circle.last
    return ptr

def highscore(num_players, num_marbles):
    circle = dllist([0])        # Initial game state
    placed = circle.first       # Pointer to current marble
    scores = [0] * num_players  # Score for each player
    for n in range(1, num_marbles+1):
        player = (n-1) % num_players
        if n % 23 == 0:         # Complex placement
            target = shift(circle, placed, -7)
            placed = shift(circle, target, +1)
            scores[player] += n + target.value
            circle.remove(target)
        else:                   # Regular inseration
            target = shift(circle, placed, +1)
            placed = circle.insertafter(n, target)
    return max(scores)

if __name__ == '__main__':
    test0 = read_input('9 players; last marble is worth 25 points')
    test1 = read_input('10 players; last marble is worth 1618 points')
    test2 = read_input('13 players; last marble is worth 7999 points')
    test3 = read_input('17 players; last marble is worth 1104 points')
    test4 = read_input('21 players; last marble is worth 6111 points')
    test5 = read_input('30 players; last marble is worth 5807 points')
    input = read_input(get_data(day=9, year=2018))
    assert (highscore(*test0) == 32)
    assert (highscore(*test1) == 8317)
    assert (highscore(*test2) == 146373)
    assert (highscore(*test3) == 2764)
    assert (highscore(*test4) == 54718)
    assert (highscore(*test5) == 37305)
    print(f'Part 1: {highscore(*input)}')
    print(f'Part 2: {highscore(input[0], 100*input[1])}')
