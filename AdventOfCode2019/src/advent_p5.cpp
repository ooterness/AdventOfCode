// Advent of Code 2019, Day 5
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/5

#include <cassert>
#include <fstream>
#include <iostream>
#include "intcode.h"

int run_diagnostic(const Program& ref, int input)
{
    static const bool VERBOSE = false;

    // Construct streams and run a new copy of the program.
    std::stringstream strm_in, strm_out;
    Program prog1(ref);
    strm_in << input;
    prog1.run(&strm_in, &strm_out, VERBOSE);

    // Result is the last value in the stream.
    int64_t result = -1;
    while (read_next(strm_out, result)) {}
    return result;
}

int main()
{
    // Unit tests for immediate mode must self-modify to halt.
    Program test1("1002,4,3,4,33");
    Program test2("1101,100,-1,4,0");
    assert (test1.run());
    assert (test2.run());

    // Selene's helpful test program.
    Program test3("3,15,1001,1,2,14,4,14,99,0,0,0,0,0,0,0,0");
    assert (test3.run_simple(123) == 17 && test3.m_prog[15] == 123);
    std::cout << "BIST 1 passed!" << std::endl;

    // Run the Part-1 diagnostic program.
    Program diagnostic("advent_p5.txt", 1);
    std::cout << "Diagnostic #1 result: " << run_diagnostic(diagnostic, 1) << std::endl;

    // Part-2 unit tests.
    Program test4("3,9,8,9,10,9,4,9,99,-1,8");  // Input == 8?
    Program test5("3,9,7,9,10,9,4,9,99,-1,8");  // Input < 8?
    Program test6("3,3,1108,-1,8,3,4,3,99");    // Input == 8?
    Program test7("3,3,1107,-1,8,3,4,3,99");    // Input < 8?
    Program test8("3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9");
    Program test9("3,3,1105,-1,9,1101,0,0,12,4,12,99,1");
    Program test10("3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,"\
                   "1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,"\
                   "999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99");
    assert (test4.run_simple(7) == 0 && test4.run_simple(8) == 1);
    assert (test5.run_simple(7) == 1 && test5.run_simple(8) == 0);
    assert (test6.run_simple(7) == 0 && test6.run_simple(8) == 1);
    assert (test7.run_simple(7) == 1 && test7.run_simple(8) == 0);
    assert (run_diagnostic(test8, 0) == 0 && run_diagnostic(test8, 1) == 1);
    assert (run_diagnostic(test9, 0) == 0 && run_diagnostic(test9, 1) == 1);
    assert (run_diagnostic(test10, 7) == 999 &&
            run_diagnostic(test10, 8) == 1000 &&
            run_diagnostic(test10, 9) == 1001);
    std::cout << "BIST 2 passed!" << std::endl;

    // Run the Part-1 diagnostic program.
    std::cout << "Diagnostic #2 result: " << run_diagnostic(diagnostic, 5) << std::endl;

    return 0;
}

