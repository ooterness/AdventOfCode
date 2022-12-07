# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter

import sys
from aocd import get_data

class Directory:
    def __init__(self, name, parent):
        self.name = name        # Name of this folder
        self.dirs = {}          # Child folders by name
        self.files = {}         # File sizes by name
        self.parent = parent    # Parent folder (top = None)
        self.total = 0          # Total size of all children

    def debug(self, level=0):
        # Print folder tree, mimic example format.
        prefix = ' ' * (2*level)
        print(f'{prefix}- {self.name} (dir, size={self.total})')
        for dir in self.dirs.values():
            dir.debug(level+1)
        for file, size in self.files.items():
            print(f'{prefix}  - {file} (file, size={size})')

    def add_dir(self, name):
        new_dir = Directory(name, self)
        self.dirs[name] = new_dir

    def add_file(self, name, size):
        self.files[name] = size
        self.increment(size)

    def increment(self, size):
        self.total += size
        if self.parent: self.parent.increment(size)

    def part1(self, threshold=100000):
        # Total size of directories under threshold.
        total = sum([dir.part1() for dir in self.dirs.values()])
        if self.total <= threshold: total += self.total
        return total

    def part2_search(self, threshold):
        # Smallest directory with total size of at least threshold.
        assert(self.total >= threshold)
        result = self.total
        for dir in self.dirs.values():
            if dir.total >= threshold:
                result = min(result, dir.part2_search(threshold))
        return result

    def part2(self, need=30000000, disk=70000000):
        # Smallest directory that would free enough space.
        threshold = self.total + need - disk
        return 0 if threshold <= 0 else self.part2_search(threshold)

def read_input(input):
    root = Directory('/', None)
    work = root
    for line in input.splitlines():
        if line.startswith('$ cd /'):
            work = root
        elif line.startswith('$ cd ..'):
            work = work.parent
        elif line.startswith('$ cd'):
            work = work.dirs[line[5:]]
        elif line.startswith('$ ls'):
            continue
        elif line.startswith('dir'):
            work.add_dir(line[4:])
        else:
            [size, name] = line.split(' ')
            work.add_file(name, int(size))
    return root

TEST = \
'''
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
'''

if __name__ == '__main__':
    test = read_input(TEST.strip())
    if len(sys.argv) > 1: test.debug()
    input = read_input(get_data(day=7, year=2022))
    assert(test.part1() == 95437)
    assert(test.part2() == 24933642)
    print(f'Part 1: {input.part1()}')
    print(f'Part 2: {input.part2()}')
