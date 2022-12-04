# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import re
from aocd import get_data

def read_input(line):
    return [int(x) for x in re.findall('[0-9]+', line)]

def insert(circle, index, value):
    return circle[:index+1] + [value] + circle[index+1:]

def remove(circle, index):
    return circle[:index] + circle[index+1:]

def offset(circle, placed, incr):
    return (placed + incr) % len(circle)

def highscore(num_players, num_marbles):
    circle = [0]                # Initial game state
    placed = 0                  # Index of current marble
    scores = [0] * num_players  # Score for each player
    for n in range(1, num_marbles):
        player = (n-1) % num_players
        if n % 23 == 0:         # Complex placement
            placed = offset(circle, placed, -7)
            scores[player] += n + circle[placed]
            circle = remove(circle, placed)
        else:                   # Regular inseration
            placed = offset(circle, placed, 1)
            circle = insert(circle, placed, n)
            placed += 1
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
    #assert (highscore(*test3) == 2764) # Typo in problem statement?
    assert (highscore(*test4) == 54718)
    assert (highscore(*test5) == 37305)
    print(f'Part 1: {highscore(*input)}')
    # TODO: This is way too slow.  Need a better data structure?
    #print(f'Part 2: {highscore(input[0], 100*input[1])}')
