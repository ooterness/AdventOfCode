// Advent of Code 2019, Day 24
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/24

#include <cassert>
#include <iostream>
#include <set>
#include <string>
#include <vector>

// Simulate one iteration of the 5x5 cellular automata.
#define BIT_MASK(r,c)   (1u << (5*(r)+(c)))
#define SET_BIT(x,r,c)  (x) |= BIT_MASK(r,c)
#define CLR_BIT(x,r,c)  (x) &= ~BIT_MASK(r,c)
#define TEST_BIT(x,r,c) ((x) & BIT_MASK(r,c))
unsigned iterate_flat(unsigned x)
{
    // For each cell...
    unsigned y = 0;
    for (unsigned r = 0 ; r < 5 ; ++r) {
        for (unsigned c = 0 ; c < 5 ; ++c) {
            // Count neighbors: Up, down, left, right
            // (Anything out-of-bounds counts as empty.)
            unsigned ct = 0;
            if ((r > 0) && TEST_BIT(x,r-1,c)) ++ct;
            if ((r < 4) && TEST_BIT(x,r+1,c)) ++ct;
            if ((c > 0) && TEST_BIT(x,r,c-1)) ++ct;
            if ((c < 4) && TEST_BIT(x,r,c+1)) ++ct;
            // Update state for this cell.
            if (ct == 1) SET_BIT(y,r,c);
            if ((ct == 2) && !TEST_BIT(x,r,c)) SET_BIT(y,r,c);
        }
    }
    return y;
}

// Iterate N steps
unsigned iterate_flat(unsigned x, unsigned n)
{
    for (unsigned a = 0 ; a < n ; ++a)
        x = iterate_flat(x);
    return x;
}

// Convert map string to integer.
unsigned read_map(const char* str)
{
    unsigned x = 0;
    for (unsigned r = 0 ; r < 5 ; ++r) {
        for (unsigned c = 0 ; c < 5 ; ++c) {
            if (str[5*r+c] == '#') SET_BIT(x,r,c);
        }
    }
    return x;
}

// Print integer as a map string.
void print_map(unsigned x)
{
    for (unsigned r = 0 ; r < 5 ; ++r) {
        std::string row(5, '.');
        for (unsigned c = 0 ; c < 5 ; ++c) {
            if (TEST_BIT(x,r,c)) row[c] = '#';
        }
        std::cout << row << std::endl;
    }
}

// Find the first layout that appears twice.
unsigned find_repeat(unsigned x)
{
    // Keep a list of previously-visited states.
    std::set<unsigned> visited;
    visited.insert(x);

    // Iterate until we find a repeat.
    while (1) {
        x = iterate_flat(x);
        if (visited.find(x) != visited.end()) return x;
        visited.insert(x);
    }
}

// Iterate once from a recursively-nested initial state.
std::vector<unsigned> iterate_nest(const std::vector<unsigned> x)
{
    const unsigned lmax = x.size() - 1;
    std::vector<unsigned> y(x.size(), 0);
    for (unsigned l = 0 ; l <= lmax ; ++l) {
        for (unsigned r = 0 ; r < 5 ; ++r) {
            for (unsigned c = 0 ; c < 5 ; ++c) {
                // Immediately skip the central placeholder.
                if (r == 2 && c == 2) continue;
                // Otherwise, count regulator neighbors.
                unsigned ct = 0;
                if ((r > 0) && TEST_BIT(x[l],r-1,c)) ++ct;
                if ((r < 4) && TEST_BIT(x[l],r+1,c)) ++ct;
                if ((c > 0) && TEST_BIT(x[l],r,c-1)) ++ct;
                if ((c < 4) && TEST_BIT(x[l],r,c+1)) ++ct;
                // Outer nesting, if applicable...
                if (l > 0 && r == 0 && TEST_BIT(x[l-1],1,2)) ++ct;
                if (l > 0 && r == 4 && TEST_BIT(x[l-1],3,2)) ++ct;
                if (l > 0 && c == 0 && TEST_BIT(x[l-1],2,1)) ++ct;
                if (l > 0 && c == 4 && TEST_BIT(x[l-1],2,3)) ++ct;
                // Inner nesting, if applicable...
                if (l < lmax && r == 1 && c == 2) {
                    if (TEST_BIT(x[l+1],0,0)) ++ct;
                    if (TEST_BIT(x[l+1],0,1)) ++ct;
                    if (TEST_BIT(x[l+1],0,2)) ++ct;
                    if (TEST_BIT(x[l+1],0,3)) ++ct;
                    if (TEST_BIT(x[l+1],0,4)) ++ct;
                }
                if (l < lmax && r == 3 && c == 2) {
                    if (TEST_BIT(x[l+1],4,0)) ++ct;
                    if (TEST_BIT(x[l+1],4,1)) ++ct;
                    if (TEST_BIT(x[l+1],4,2)) ++ct;
                    if (TEST_BIT(x[l+1],4,3)) ++ct;
                    if (TEST_BIT(x[l+1],4,4)) ++ct;
                }
                if (l < lmax && r == 2 && c == 1) {
                    if (TEST_BIT(x[l+1],0,0)) ++ct;
                    if (TEST_BIT(x[l+1],1,0)) ++ct;
                    if (TEST_BIT(x[l+1],2,0)) ++ct;
                    if (TEST_BIT(x[l+1],3,0)) ++ct;
                    if (TEST_BIT(x[l+1],4,0)) ++ct;
                }
                if (l < lmax && r == 2 && c == 3) {
                    if (TEST_BIT(x[l+1],0,4)) ++ct;
                    if (TEST_BIT(x[l+1],1,4)) ++ct;
                    if (TEST_BIT(x[l+1],2,4)) ++ct;
                    if (TEST_BIT(x[l+1],3,4)) ++ct;
                    if (TEST_BIT(x[l+1],4,4)) ++ct;
                }
                // Update state for this cell.
                if (ct == 1) SET_BIT(y[l],r,c);
                if ((ct == 2) && !TEST_BIT(x[l],r,c)) SET_BIT(y[l],r,c);
            }
        }
    }
    return y;
}

// Iterate N times from a recursively-nested initial state.
std::vector<unsigned> iterate_nest(unsigned init, unsigned steps)
{
    // Maximum nesting level in each direction is D=(steps/2), rounded up.
    // Therefore the overall vector size is equal to 2*D+1.
    unsigned dd = (steps+1) / 2;
    unsigned nn = 2*dd + 1;

    // Middle element of this vector contains the initial state.
    std::vector<unsigned> x(nn, 0);
    x[dd] = init;

    // Iterate N times and return the result.
    for (unsigned a = 0 ; a < steps ; ++a)
        x = iterate_nest(x);
    return x;
}

// Count the number of bugs in a recursively-nested initial state.
unsigned count_nest(const std::vector<unsigned>& x)
{
    unsigned count = 0;
    for (unsigned l = 0 ; l < x.size() ; ++l) {
        for (unsigned r = 0 ; r < 5 ; ++r) {
            for (unsigned c = 0 ; c < 5 ; ++c) {
                if (TEST_BIT(x[l], r, c)) ++count;
            }
        }
    }
    return count;
}

int main()
{
    // Unit tests from the Part-A example scenario:
    const unsigned TEST_INIT = read_map("....##..#.#..##..#..#....");
    const unsigned TEST_REF1 = read_map("#..#.####.###.###.##.##..");
    const unsigned TEST_REF2 = read_map("#####....#....#...#.#.###");
    const unsigned TEST_REF3 = read_map("#....####....###.##..##.#");
    const unsigned TEST_REF4 = read_map("####.....###..#.....##...");
    assert (iterate_flat(TEST_INIT, 1) == TEST_REF1);
    assert (iterate_flat(TEST_INIT, 2) == TEST_REF2);
    assert (iterate_flat(TEST_INIT, 3) == TEST_REF3);
    assert (iterate_flat(TEST_INIT, 4) == TEST_REF4);

    // Part-A solution: Find the first repeated state.
    unsigned MAIN_INIT = read_map("##.#.#.#####......#.#.##.");
    std::cout << "Part-A: " << find_repeat(MAIN_INIT) << std::endl;

    // Unit test from the Part-B nested scenario:
    const unsigned NEST_REF[] = {
        read_map("..#...#.#...?.#.#.#...#.."),
        read_map("...#....##..?.....##...#."),
        read_map("#.#...#.....?...#...#.#.."),
        read_map(".#.##....#..?.#...##.###."),
        read_map("#..##...##..?.....#..####"),
        read_map(".#....#.##.#?............"),
        read_map(".##..#..##..?.###.#######"),
        read_map("###..##.#.#.?...#.###.#.."),
        read_map("..###.....#.?..#....#...#"),
        read_map(".###.#..#.#.?..##.#......"),
        read_map("####.#..#.#.?#.####......")};
    std::vector<unsigned> testb = iterate_nest(TEST_INIT, 10);
    for (unsigned a = 0 ; a < testb.size() ; ++a)
        assert(testb[a] == NEST_REF[a]);

    // Part-B solution: Iterate the nested state 200 times.
    std::vector<unsigned> partb = iterate_nest(MAIN_INIT, 200);
    std::cout << "Part-B: " << count_nest(partb) << std::endl;

    return 0;
}

