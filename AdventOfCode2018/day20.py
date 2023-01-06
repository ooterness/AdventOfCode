# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

from aocd import get_data

DIRECTIONS = {
    'N': (-1,0),
    'S': (+1,0),
    'E': (0,+1),
    'W': (0,-1),
}

# An individual step.
class Step:
    def __init__(self, step):
        self.step = DIRECTIONS[step]

    def explore(self, rooms, posn):
        new_posn = set()
        for rc0 in posn:
            rc1 = (rc0[0] + self.step[0], rc0[1] + self.step[1])
            if rc1 in rooms:
                rooms[rc0] = min(rooms[rc0], rooms[rc1] + 1)
                rooms[rc1] = min(rooms[rc1], rooms[rc0] + 1)
            else:
                rooms[rc1] = rooms[rc0] + 1
            new_posn.add(rc1)
        return new_posn

# A series of individual Steps or Branches.
class Sequence:
    def __init__(self, regex):
        rdpos = 0               # Current parsing index
        self.steps = []         # List of parsed tokens.
        while rdpos < len(regex):
            if regex[rdpos] == '(':
                self.steps.append(Branch(regex[rdpos+1:]))
                rdpos += self.steps[-1].len + 2
            elif regex[rdpos] == '$' or regex[rdpos] == '^':
                rdpos += 1
            else:
                self.steps.append(Step(regex[rdpos]))
                rdpos += 1

    def explore(self, rooms, posn):
        for step in self.steps:
            posn = step.explore(rooms, posn)
        return posn

# A list of possible Sequences.
class Branch:
    def __init__(self, regex):
        level = 0               # Current parenthesis nesting level.
        tstart = 0              # Starting index for nested tokens.
        self.len = len(regex)   # Length of input consumed.
        self.seq = []           # Working list of parsed tokens.
        for (n,ch) in enumerate(regex):
            if ch == '|' and level == 0:
                self.seq.append(Sequence(regex[tstart:n]))
                tstart = n + 1
            elif ch == '(': level += 1
            elif ch == ')' and level == 0:
                self.len = n
                break
            elif ch == ')': level -= 1
        self.seq.append(Sequence(regex[tstart:self.len]))

    def explore(self, rooms, posn):
        new_posn = set()
        for opt in self.seq:
            for p in opt.explore(rooms, posn): new_posn.add(p)
        return new_posn

def explore(regex):
    seq = Sequence(regex)
    rooms = {(0,0):0}
    seq.explore(rooms, set(rooms.keys()))
    return rooms

def part1(rooms):
    return max([d for d in rooms.values()])

def part2(rooms):
    return sum([d >= 1000 for d in rooms.values()])

if __name__ == '__main__':
    TEST1 = explore('^WNE$')
    TEST2 = explore('^ENWWW(NEEE|SSE(EE|N))$')
    TEST3 = explore('^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$')
    TEST4 = explore('^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$')
    TEST5 = explore('^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$')
    assert(part1(TEST1) == 3)
    assert(part1(TEST2) == 10)
    assert(part1(TEST3) == 18)
    assert(part1(TEST4) == 23)
    assert(part1(TEST5) == 31)
    input = explore(get_data(day=20, year=2018))
    print(f'Part 1: {part1(input)}')
    print(f'Part 2: {part2(input)}')
