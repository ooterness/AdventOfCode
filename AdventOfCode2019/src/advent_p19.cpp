// Advent of Code 2019, Day 19
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/19

#include <cmath>
#include <sstream>
#include "intcode.h"

bool test_grid(const Program& prog, unsigned x, unsigned y)
{
    // Create a copy of the test program at current coordinates.
    Program copy(prog);
    std::stringstream in;
    in << x << ", " << y;

    // Run to first output.
    int64_t out = 0;
    copy.run_next(&in, out);

    return (out > 0);
}

unsigned count_grid(const Program& prog, unsigned max_xy, const char* lbl)
{
    // Print header, if requested.
    if (lbl) std::cout << lbl << ", Range = " << max_xy << std::endl;

    // Test each grid point...
    unsigned count = 0;
    for (unsigned y = 0 ; y < max_xy ; ++y) {
        for (unsigned x = 0 ; x < max_xy ; ++x) {
            bool tst = test_grid(prog, x, y);
            if (tst) ++count;
            if (lbl) std::cout << ((tst > 0) ? '#' : '.');
        }
        if (lbl) std::cout << std::endl;
    }
    return count;
}

unsigned closest_square(const Program& prog, unsigned size_xy)
{
    // Test along a circular wave-front.
    for (unsigned r = 0 ; 1 ; ++r) {
        for (unsigned x = 0 ; x <= r ; ++x) {
            unsigned y = (unsigned)sqrt(r*r - x*x);
            if (test_grid(prog, x, y) &&
                test_grid(prog, x+size_xy-1, y) &&
                test_grid(prog, x, y+size_xy-1) &&
                test_grid(prog, x+size_xy-1, y+size_xy-1)) {
                return 10000*x + y;
            }
        }
    }
}

int main()
{
    // Part-1 example and solution.
    Program prog("advent_p19.txt", 1);
    std::cout << "Affected points = " << count_grid(prog, 50, "Test50") << std::endl;
    std::cout << "Closest 100-square = " << closest_square(prog, 100) << std::endl;

    return 0;
}

