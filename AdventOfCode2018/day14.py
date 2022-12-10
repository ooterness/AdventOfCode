# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy

def iterate(recipes, elves):
    # Create new recipes.
    next = recipes[elves[0]] + recipes[elves[1]]
    if next >= 10: recipes.append(next // 10)
    recipes.append(next % 10)
    # Update elf positions.
    elves[0] = (1 + elves[0] + recipes[elves[0]]) % len(recipes)
    elves[1] = (1 + elves[1] + recipes[elves[1]]) % len(recipes)

INIT_STATE = ([3,7], [0,1])

def recipe_string(recipes):
    return ''.join([str(x) for x in recipes])

def part1(stop):
    # Simulate the recipe-making process...
    (recipes, elves) = deepcopy(INIT_STATE)
    while len(recipes) < stop + 10:
        iterate(recipes, elves)
    # Calculate the final score.
    next_ten = recipes[stop : stop+10]
    return recipe_string(next_ten)

def part2(stop):
    # Simulate the recipe-making process...
    (recipes, elves) = deepcopy(INIT_STATE)
    while True:
        # Since updating and searching the string is slow, iterate
        # many times before each search and only search new data.
        prev = max(0, len(recipes) - len(stop))
        for t in range(100):
            iterate(recipes, elves)
        ref = recipe_string(recipes[prev:])
        if stop in ref: return prev + ref.find(stop)

if __name__ == '__main__':
    input = get_data(day=14, year=2018)
    assert (part1(9) == '5158916779')
    assert (part1(5) == '0124515891')
    assert (part1(18) == '9251071085')
    assert (part1(2018) == '5941429882')
    assert (part2('51589') == 9)
    assert (part2('01245') == 5)
    assert (part2('92510') == 18)
    assert (part2('59414') == 2018)
    print(f'Part 1: {part1(int(input))}')
    print(f'Part 2: {part2(input)}')
