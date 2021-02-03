// Advent of Code 2019, Day 9
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/9

#include <sstream>
#include "intcode.h"

// Test the quine provided as one of the Part-1 unit tests.
int quine_test(const char* str)
{
    Program uut(str);
    std::stringstream out;
    if (uut.run(0, &out))
        return out.str().compare(str);
    else
        return -1;
}

int main()
{
    // Part-1 unit tests:
    const char* TEST1("109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99,");
    Program test2("1102,34915192,34915192,7,4,7,99,0");
    Program test3("104,1125899906842624,99");
    assert (quine_test(TEST1) == 0);
    assert (test2.run_simple(0) == 1219070632396864LL);
    assert (test3.run_simple(0) == 1125899906842624LL);

    // Run the main program in "test mode"
    Program prog1("advent_p9.txt", 1);
    std::cout << "BOOST test result = " << prog1.run_simple(1) << std::endl;

    // Run the main program in ""sensor mode"
    Program prog2("advent_p9.txt", 1);
    std::cout << "BOOST sense result = " << prog2.run_simple(2) << std::endl;

    return 0;
}

