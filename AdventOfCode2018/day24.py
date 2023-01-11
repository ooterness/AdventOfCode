# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import copy, deepcopy
import re

def read_input(input):
    lines = list(input.splitlines())
    split = lines.index('')
    immune = [Unit(line) for line in lines[1:split]]
    infect = [Unit(line) for line in lines[split+2:]]
    return (immune, infect)

# Return string on match, empty string otherwise.
def search_or_empty(pattern, string):
    match = re.search(pattern, string)
    return '' if match is None else match[1]

class Unit:
    # Create unit from input string, e.g.:
    # '846 units each with 91 hit points (weak to fire) with an attack that does 99 fire damage at initiative 4'
    def __init__(self, line):
        # Parse the basic parameters using regular expressions.
        (self.count, self.hp, self.damage, self.initiative) = \
            [int(x) for x in re.findall('[0-9]+', line)]
        self.dtype = re.search(r'\d+ (\w+) damage', line)[1]    # Numeral + (word) + "damage"
        # Do we have special defense types?
        parens = search_or_empty(r'\((.*)\)', line)             # Everything in parenthesis
        immune = search_or_empty(r'immune to ([^;]*)', parens)  # From "immune to " up to ; or end
        weak = search_or_empty(r'weak to ([^;]*)', parens)      # From "weak to " up to ; or end
        self.immune = list(filter(None, immune.split(', ')))    # Split on commas, ignoring empties
        self.weak = list(filter(None, weak.split(', ')))

    # Effective attack power
    def power(self):
        return self.count * self.damage

    # Predict damage against a specified opponent.
    def predict(self, other):
        if other.count == 0: return 0
        elif self.dtype in other.immune: return 0
        elif self.dtype in other.weak: return 2*self.power()
        else: return self.power()

    # Attack the selected opponent, reducing their unit count.
    def attack(self, verbose):
        if self.target is None: return 0
        killed = self.predict(self.target) // self.target.hp
        if verbose: print(f'Init={self.initiative}: Attack {self.target.initiative}, killing {killed} units.')
        self.target.count -= min(self.target.count, killed)
        return killed

    # Choose a target to attack.
    def choose(self, targets):
        priority = lambda t: (self.predict(t), t.power(), t.initiative)
        if all([self.predict(t)==0 for t in targets]):
            self.target = None
        else:
            self.target = max(targets, key=priority)
            targets.remove(self.target)

# Count surviving units on the given team.
def alive(team):
    return sum([t.count for t in team])

# Simulate a single round of battle.
def battle_once(team_a, team_b, verbose=False):
    init_descending = lambda x: -x.initiative
    power_descending = lambda x: -x.power()
    # Target selection.
    temp_a = copy(team_a)
    temp_b = copy(team_b)
    for a in sorted(team_a, key=power_descending):
        a.choose(temp_b)
    for b in sorted(team_b, key=power_descending):
        b.choose(temp_a)
    # Resolve attacks.
    total_killed = sum([x.attack(verbose) for x in
        sorted(team_a + team_b, key=init_descending)])
    if verbose: print(f'Surviving units {alive(team_a)} - {alive(team_b)}')
    return total_killed

# Simulate rounds until the battle is resolved.
def battle_all(team_a, team_b, boost_a):
    team_a = deepcopy(team_a)
    team_b = deepcopy(team_b)
    for a in team_a: a.damage += boost_a
    while alive(team_a) > 0 and alive(team_b) > 0 \
        and battle_once(team_a, team_b): None
    return (alive(team_a), alive(team_b))

def part1(input):
    # Simulate the battle from the given initial state.
    (immune, infect) = battle_all(*input, 0)
    return immune + infect

def part2(input):
    # Binary search until immune system wins.
    min_boost = 1
    max_boost = 2**20
    while (min_boost < max_boost):
        boost = (min_boost + max_boost) // 2
        (a, b) = battle_all(*input, boost)
        if b == 0:      # Immune won -> Decrease upper bound
            max_boost = boost
        else:           # Infect won or tie -> Increase lower bound
            min_boost = boost + 1
    (immune, infect) = battle_all(*input, min_boost)
    return immune + infect

TEST = \
'''
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    input = read_input(get_data(day=24, year=2018))
    assert(part1(test) == 5216)
    print(f'Part 1: {part1(input)}')
    assert(part2(test) == 51)   # Boost = 1569
    print(f'Part 2: {part2(input)}')
