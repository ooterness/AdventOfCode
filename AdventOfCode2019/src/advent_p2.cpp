// Advent of Code 2019, Day 2
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/2

#include <cassert>
#include <iostream>
#include "intcode.h"

// Test programs from instructions.
const char TEST1[] = "1,0,0,0,99";
const char TEST2[] = "2,3,0,3,99";
const char TEST3[] = "2,4,4,5,99,0";
const char TEST4[] = "1,1,1,4,99,5,6,0,99";

// The main "gravity assist" program.
const char GRAV1[] =
    "1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,10,1,19,1,19,5,23,1,23,9,27,"\
    "2,27,6,31,1,31,6,35,2,35,9,39,1,6,39,43,2,10,43,47,1,47,9,51,1,51,6,55,"\
    "1,55,6,59,2,59,10,63,1,6,63,67,2,6,67,71,1,71,5,75,2,13,75,79,1,10,79,83,"\
    "1,5,83,87,2,87,10,91,1,5,91,95,2,95,6,99,1,99,6,103,2,103,6,107,"\
    "2,107,9,111,1,111,5,115,1,115,6,119,2,6,119,123,1,5,123,127,1,127,13,131,"\
    "1,2,131,135,1,135,10,0,99,2,14,0,0";

// Run the grav1 program with the specified input pair.
// (Makes a copy of the program to ensure no side effects.)
int run_grav(int a, int b)
{
    // Make a copy of the original grav1 program.
    Program grav1(GRAV1);

    // Overwrite arguments and run.
    // If successful, output is in index zero.
    grav1.m_prog[1] = a;
    grav1.m_prog[2] = b;
    if (grav1.run())
        return grav1.m_prog[0];
    else
        return -1;
}

int main()
{
    // Execute the test programs.
    Program test1(TEST1);
    Program test2(TEST2);
    Program test3(TEST3);
    Program test4(TEST4);
    assert(test1.run() && test1.m_prog[0] == 2);
    assert(test2.run() && test2.m_prog[3] == 6);
    assert(test3.run() && test3.m_prog[5] == 9801);
    assert(test4.run() && test4.m_prog[0] == 30 && test4.m_prog[4] == 2);

    // Part 1: Execute the gravity assist program with specified arguments.
    std::cout << "Part 1 output = " << run_grav(12,2) << std::endl;

    // Part 2: Find the input pair that gets the desired result.
    for (int a = 0 ; a < 100 ; ++a) {
        for (int b = 0 ; b < 100 ; ++b) {
            int result = run_grav(a, b);
            if (result == 19690720) {
                std::cout << "Part 2 output = " << 100*a + b << std::endl;
                return 0;   // Success!
            }
        }
    }

    std::cout << "Part 2 unsolved :(" << std::endl;
    return 1;   // No match found...
}

