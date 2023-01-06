# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter
# Opcodes used for Day 16 and Day 19

class BadRegister(Exception):
    def __init__(self):
        super('Invalid register index')

# Define each of the instructions by name:
def addr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] + reg[b]; return True
def addi(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] + b
def mulr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] * reg[b]
def muli(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] * b
def banr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] & reg[b]
def bani(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] & b
def borr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] | reg[b]
def bori(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a] | b
def setr(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = reg[a]
def seti(a, b, c, reg):
    if c >= len(reg): raise BadRegister
    reg[c] = a
def gtir(a, b, c, reg):
    if b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(a > reg[b])
def gtri(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(reg[a] > b)
def gtrr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(reg[a] > reg[b])
def eqir(a, b, c, reg):
    if b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(a == reg[b])
def eqri(a, b, c, reg):
    if a >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(reg[a] == b)
def eqrr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg) or c >= len(reg): raise BadRegister
    reg[c] = int(reg[a] == reg[b])

ALL_INSTRUCTIONS = {
    'addr':addr, 'addi':addi, 'mulr':mulr, 'muli':muli,
    'banr':banr, 'bani':bani, 'borr':borr, 'bori':bori,
    'setr':setr, 'seti':seti, 'gtir':gtir, 'gtri':gtri,
    'gtrr':gtrr, 'eqir':eqir, 'eqri':eqri, 'eqrr':eqrr
}

# Define some new psuedo-instructions.
# (Not part of the original set, but useful for optimization.)
def noop(a, b, c, reg):
    None
def modr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    div = (reg[b] % reg[a] == 0)    # Is RegA a factor of RegB?
    reg[c] = reg[b] if div else 0   # Store result in RegC

# A sequence of instructions with an instruction-pointer register.
class Program:
    def __init__(self, input):
        import re
        self.prog = []
        for (n, line) in enumerate(input.splitlines()):
            if n == 0:
                self.iptr = int(line[4:])
            else:
                op = ALL_INSTRUCTIONS[line[0:4]]
                (a, b, c) = [int(x) for x in re.findall('[0-9]+', line)]
                self.prog.append((op, a, b, c))

    # Execute a single instruction.
    def step(self, reg, ctr):
        assert (0 <= self.iptr and self.iptr < len(reg))
        assert (0 <= ctr and ctr < len(self.prog))
        (op, a, b, c) = self.prog[ctr]
        reg[self.iptr] = ctr
        op(a, b, c, reg)
        return reg[self.iptr] + 1

    # Run program to completion.
    def run(self, reg):
        ctr = 0
        while 0 <= ctr and ctr < len(self.prog):
            ctr = self.step(reg, ctr)
        return reg
