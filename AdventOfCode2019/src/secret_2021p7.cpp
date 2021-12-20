// Advent of Code 2021, Day 7 Easter Egg
// Copyright 2021 by Alex Utter
// https://adventofcode.com/2021/day/7

#include "intcode.h"

int main()
{
    // Load the crab coordinates as an intcode program.
    Program prog(
        "../../AdventOfCode2021/input/input07.txt",
        Program::RUNMODE_LOADFILE);
    std::stringstream strm_out;

    // Run program until it terminates or needs more input.
    prog.run(0, &strm_out);

    // Print results.
    print_ascii(strm_out);
    return 0;
}

