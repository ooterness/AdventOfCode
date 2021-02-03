// Advent of Code 2019, Day 1
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/1

#include <cassert>
#include <fstream>
#include <iostream>

// Naive fuel calculation (Part 1)
unsigned fuel_naive(unsigned mass) {
    if (mass > 8)
        return (mass / 3) - 2;
    else
        return 0;
}

// Recursive fuel calculation (Part 2)
unsigned fuel_iter(unsigned mass) {
    unsigned total = 0, next = fuel_naive(mass);
    while (next > 0) {
        total += next;
        next = fuel_naive(next);
    }
    return total;
}

int main()
{
    std::ifstream in("advent_p1.txt");

    // Tests based on instructions:
    assert (fuel_naive(14) == 2);
    assert (fuel_naive(1969) == 654);
    assert (fuel_iter(1969) == 966);
    assert (fuel_iter(100756) == 50346);

    // Fuel for each module as instructed.
    unsigned mass, total_naive = 0, total_iter = 0;
    while (in >> mass, in) {
        total_naive += fuel_naive(mass);
        total_iter  += fuel_iter(mass);
    }

    std::cout << "Naive fuel: " << total_naive << std::endl;
    std::cout << "Total fuel: " << total_iter << std::endl;
    return 0;
}

