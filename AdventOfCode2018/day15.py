# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data
from copy import deepcopy
from collections import deque

'''A row and column coordinate.'''
class Position:
    def __init__(self, r, c):
        self.r = r
        self.c = c

    def __eq__(self, pos):
        return (self.r == pos.r) and (self.c == pos.c)

    def __hash__(self):
        return hash((self.r, self.c))

    '''Get a list of unoccupied adjacent positions.'''
    def adj(self, occupied):
        x = [Position(self.r-1, self.c),    # Up
             Position(self.r, self.c-1),    # Left
             Position(self.r, self.c+1),    # Right
             Position(self.r+1, self.c)]    # Down
        return [p for p in x if not occupied[p.r][p.c]]

    '''Manhattan distance to another Position.'''
    def dist(self, pos):
        return abs(self.r - pos.r) + abs(self.c - pos.c)

    '''Manhattan distance to a list of Positions.'''
    def dists(self, list_pos):
        return [self.dist(pos) for pos in list_pos]

'''Any combat unit (i.e., an elf or goblin).'''
class Unit(Position):
    def __init__(self, type, r, c):
        Position.__init__(self, r, c)
        self.ap = 3         # Attack power
        self.hp = 200       # Hit points
        self.range = 1      # Attack range
        self.type = type    # Faction

    '''Move towards nearest enemy.'''
    def move(self, occupied, enemies):
        # Remain still if we can already attack an enemy unit.
        in_range = lambda pos: \
            any([r <= self.range for r in pos.dists(enemies)])
        if in_range(self): return False
        # Breadth-first search for nearest target.
        self_adj = self.adj(occupied)
        search = deque([(m,m) for m in self_adj])
        visited = set(self_adj)
        while len(search) > 0:
            (pos,first) = search.popleft()
            if in_range(pos):       # Nearest target found?
                self.r = first.r    # Step towards target
                self.c = first.c
                return True
            for next in pos.adj(occupied):
                if not next in visited:
                    search.append((next,first))
                    visited.add(next)
        return False                # No reachable enemies

    '''Attack weakest enemy, if one is in range.'''
    def attack(self, enemies):
        target = None
        for enemy in enemies:       # Attack first enemy in range
            if self.dist(enemy) > self.range: continue
            if (target is None) or (enemy.hp < target.hp): target = enemy
        if target: target.hp -= self.ap

'''Sort a list of live units in reading order.'''
def live_units(units):
    reading = lambda unit: (unit.r, unit.c)
    return sorted([u for u in units if u.hp > 0], key=reading)

'''Complete combat scenario with walls and units.'''
class Scenario:
    '''Read initial state from text input.'''
    def __init__(self, input):
        # Current timestep.
        self.round = 0
        # Read the locations of each fixed wall.
        line2walls = lambda line: [ch == '#' for ch in line]
        self.walls = [line2walls(line) for line in input.splitlines()]
        # Read the initial locations of each unit.
        self.elf = []
        self.gob = []
        for (r, line) in enumerate(input.splitlines()):
            for (c, ch) in enumerate(line):
                if ch == 'E': self.elf.append(Unit(ch, r, c))
                if ch == 'G': self.gob.append(Unit(ch, r, c))

    '''Print the current combat state.'''
    def debug(self):
        # Draw the walls first, then overlay units.
        units = live_units(self.elf + self.gob)
        state = [['█' if w else ' ' for w in row] for row in self.walls]
        for unit in units: state[unit.r][unit.c] = unit.type
        # Convert to string representation.
        hp_str = lambda row: str([unit.hp for unit in units if unit.r == row])
        hp_tot = sum([unit.hp for unit in units])
        state_str = '\n'.join([
            ''.join(row) + ' ' + hp_str(r)
            for (r,row) in enumerate(state)])
        print(f'Round {self.round}: {hp_tot}\n{state_str}')

    '''Get a list of enemies for the given unit.'''
    def enemies_of(self, unit):
        return live_units(self.gob if unit.type == 'E' else self.elf)

    '''Is combat finished? (i.e., One side completely eliminated.)'''
    def finished(self):
        return (len(self.elf) == 0) or (len(self.gob) == 0)

    '''Create a grid of occupied squares from current state.'''
    def occupied(self):
        state = deepcopy(self.walls)
        for unit in self.elf: state[unit.r][unit.c] = unit.hp > 0
        for unit in self.gob: state[unit.r][unit.c] = unit.hp > 0
        return state

    '''Combat outcome score.'''
    def outcome(self):
        elf_hp = sum([unit.hp for unit in self.elf])
        gob_hp = sum([unit.hp for unit in self.gob])
        return self.round * (elf_hp + gob_hp)

    '''Advance simulation by one timestep.'''
    def iterate(self):
        # Update unit states in reading order.
        full_round = True
        for unit in live_units(self.elf + self.gob):
            if unit.hp <= 0: continue
            enemies = self.enemies_of(unit)
            if len(enemies) == 0:
                full_round = False; break
            unit.move(self.occupied(), enemies)
            unit.attack(enemies)
        # Prune dead units and update time counter.
        self.elf = live_units(self.elf)
        self.gob = live_units(self.gob)
        if full_round: self.round += 1

def part1(init, verbose=0):
    state = deepcopy(init)
    while not state.finished():
        if verbose > 1: state.debug()
        state.iterate()
    if verbose > 0: state.debug()
    return state.outcome()

TEST1 = \
'''
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######
'''

TEST2 = \
'''
#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######
'''

TEST3 = \
'''
#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######
'''

TEST4 = \
'''
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######
'''

TEST5 = \
'''
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######
'''

TEST6 = \
'''
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########
'''

if __name__ == '__main__':
    # Read all initial states.
    test1 = Scenario(TEST1[1:])
    test2 = Scenario(TEST2[1:])
    test3 = Scenario(TEST3[1:])
    test4 = Scenario(TEST4[1:])
    test5 = Scenario(TEST5[1:])
    test6 = Scenario(TEST6[1:])
    input = Scenario(get_data(day=15, year=2018))
    # Unit tests for part 1:
    assert(part1(test1) == 27730)
    assert(part1(test2) == 36334)
    assert(part1(test3) == 39514)
    assert(part1(test4) == 27755)
    assert(part1(test5) == 28944)
    assert(part1(test6) == 18740)
    # Problem solution:
    print(f'Part 1: {part1(input)}')
