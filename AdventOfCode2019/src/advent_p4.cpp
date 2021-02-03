// Advent of Code 2019, Day 4
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/4

#include <iostream>

int main()
{
    // Count matches over the specified range...
    unsigned match1 = 0, match2 = 0;
    for (unsigned n = 347312 ; n <= 805915 ; ++n) {
        // Extract each digit.
        unsigned d0 = (n / 1) % 10;
        unsigned d1 = (n / 10) % 10;
        unsigned d2 = (n / 100) % 10;
        unsigned d3 = (n / 1000) % 10;
        unsigned d4 = (n / 10000) % 10;
        unsigned d5 = (n / 100000) % 10;
        // Test for ascending digits with at least one repeat.
        if ((d5 <= d4 && d4 <= d3 && d3 <= d2 && d2 <= d1 && d1 <= d0) &&
            (d5 == d4 || d4 == d3 || d3 == d2 || d2 == d1 || d1 == d0)) {
            // First criteria met, try the second.
            ++match1;
            if ((d5 == d4 && d4 != d3) ||
                (d5 != d4 && d4 == d3 && d3 != d2) ||
                (d4 != d3 && d3 == d2 && d2 != d1) ||
                (d3 != d2 && d2 == d1 && d1 != d0) ||
                (d2 != d1 && d1 == d0)) {
                ++match2;
            }
        }
    }

    std::cout << "Possible passwords: " << match1 << " / " << match2 << std::endl;
    return 0;
}

