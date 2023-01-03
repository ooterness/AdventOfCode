# -*- coding: utf-8 -*-
# Copyright 2022 by Alex Utter
# Opcodes used for Day 16 and Day 19

# Store value x in register c. (Used in almost every instruction.)
def store(x, c, reg):
    if c >= len(reg): raise Exception('Invalid register index.')
    return [x if c == n else reg[n] for n in range(len(reg))]

# Define each of the instructions by name:
def addr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] + reg[b], c, reg)
def addi(a, b, c, reg):
    if a >= len(reg): return None
    return store(reg[a] + b, c, reg)
def mulr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] * reg[b], c, reg)
def muli(a, b, c, reg):
    if a >= len(reg): return None
    return store(reg[a] * b, c, reg)
def banr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] & reg[b], c, reg)
def bani(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] & b, c, reg)
def borr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] | reg[b], c, reg)
def bori(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(reg[a] | b, c, reg)
def setr(a, b, c, reg):
    if a >= len(reg): return None
    return store(reg[a], c, reg)
def seti(a, b, c, reg):
    return store(a, c, reg)
def gtir(a, b, c, reg):
    if b >= len(reg): return None
    return store(int(a > reg[b]), c, reg)
def gtri(a, b, c, reg):
    if a >= len(reg): return None
    return store(int(reg[a] > b), c, reg)
def gtrr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(int(reg[a] > reg[b]), c, reg)
def eqir(a, b, c, reg):
    if b >= len(reg): return None
    return store(int(a == reg[b]), c, reg)
def eqri(a, b, c, reg):
    if a >= len(reg): return None
    return store(int(reg[a] == b), c, reg)
def eqrr(a, b, c, reg):
    if a >= len(reg) or b >= len(reg): return None
    return store(int(reg[a] == reg[b]), c, reg)

ALL_INSTRUCTIONS = {
    'addr':addr, 'addi':addi, 'mulr':mulr, 'muli':muli,
    'banr':banr, 'bani':bani, 'borr':borr, 'bori':bori,
    'setr':setr, 'seti':seti, 'gtir':gtir, 'gtri':gtri,
    'gtrr':gtrr, 'eqir':eqir, 'eqri':eqri, 'eqrr':eqrr
}
