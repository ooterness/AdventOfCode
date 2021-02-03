// Advent of Code 2019, Day 12
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/12

#include <cassert>
#include <iostream>
#include <set>
#include <sstream>
#include <vector>

std::vector<int> str2vec(const char* str)
{
    std::stringstream strm(str);
    int next; std::vector<int> result;

    while (strm >> next, strm) {
        result.push_back(next);
    }
    return result;
}

int sign(int x)
{
    if (x < 0) return -1;
    if (x > 0) return +1;
    return 0;
}

struct OneAxis {
    std::vector<int> p, v;

    explicit OneAxis(const std::vector<int>& ref)
        : p(ref)            // Copy input positions
        , v(ref.size(), 0)  // Zero velocity for each
    {
        // Nothing else to initialize.
    }

    void step()
    {
        // Apply acceleration first.
        for (unsigned a = 0 ; a < p.size() ; ++a) {
            int dv = 0;
            for (unsigned b = 0 ; b < p.size() ; ++b)
                dv += sign(p[b] - p[a]);
            v[a] += dv;
        }
        // Now update positions
        for (unsigned a = 0 ; a < p.size() ; ++a) {
            p[a] += v[a];
        }
    }

    bool check_pos(const std::vector<int>& ref) const
    {
        if (ref.size() != p.size()) return false;
        for (unsigned a = 0 ; a < ref.size() ; ++a)
            if (ref[a] != p[a]) return false;
        return true;
    }

    bool operator<(const OneAxis& other) const
    {
        for (unsigned a = 0 ; a < p.size() ; ++a) {
            if (p[a] < other.p[a]) return true;
            if (p[a] > other.p[a]) return false;
            if (v[a] < other.v[a]) return true;
            if (v[a] > other.v[a]) return false;
        }
        return false;   // Exactly equal
    }
};

struct ThreeAxis {
    OneAxis x, y, z;

    explicit ThreeAxis(const char* _x, const char* _y, const char* _z)
        : x(str2vec(_x))
        , y(str2vec(_y))
        , z(str2vec(_z))
    {
        // Nothing else to initialize.
    }

    void step(unsigned n=1)
    {
        while (n--) {
            x.step();
            y.step();
            z.step();
        }
    }

    bool check_pos(const char* _x, const char* _y, const char* _z) const
    {
        return x.check_pos(str2vec(_x))
            && y.check_pos(str2vec(_y))
            && z.check_pos(str2vec(_z));
    }

    unsigned energy() const
    {
        unsigned nmoons = x.p.size(), total = 0;
        for (unsigned a = 0 ; a < nmoons ; ++a) {
            unsigned kin = (unsigned)(abs(x.v[a]) + abs(y.v[a]) + abs(z.v[a]));
            unsigned pot = (unsigned)(abs(x.p[a]) + abs(y.p[a]) + abs(z.p[a]));
            total += kin * pot;
        }
        return total;
    }
};

// Find single-axis period.
unsigned one_period(const char* str)
{
    // Calculate the initial state and add it to the list.
    OneAxis state(str2vec(str));
    std::set<OneAxis> visited;
    visited.insert(state);

    // Keep advancing state to see when we repeat.
    unsigned nsteps = 0;
    while (1) {
        state.step(); ++nsteps;
        auto it = visited.find(state);
        if (it != visited.end()) return nsteps;
        visited.insert(state);
    }
}

// Three-axis period is just the least common multiple of each axis.
// (I don't feel like doing that here, so just print it and find an online calculator.)
void three_period(const char* x, const char* y, const char* z)
{
    std::cout << one_period(x) << ", "
              << one_period(y) << ", "
              << one_period(z) << std::endl;
}

// Test constructors: Separate strings for X, Y, Z
#define XYZ_TEST1       "-1 2 4 3", "0 -10 -8 5", "2 -7 8 -1"
#define XYZ_TEST2       "-8 5 2 9", "-10 5 -7 -8", "0 10 3 -3"
#define XYZ_TEST2_ST10  "-9 4 8 5", "-10 10 -10 -10", "1 9 -3 3"
#define XYZ_TEST2_ST100 "8 13 -29 16", "-12 16 -11 -13", "-9 -3 -1 23"
#define XYZ_INPUT       "16 0 6 -3", "-11 -4 4 -2", "2 7 -10 -4"

int main()
{
    // Use the second example test problem, check simulation after 10 and 100 steps.
    ThreeAxis test(XYZ_TEST2);
    test.step(10); assert(test.check_pos(XYZ_TEST2_ST10));
    test.step(90); assert(test.check_pos(XYZ_TEST2_ST100));
    assert(test.energy() == 1940);

    // Part-1: Find energy in given system after 1000 steps.
    ThreeAxis part1(XYZ_INPUT);
    part1.step(1000);
    std::cout << "Part1 energy = " << part1.energy() << std::endl;

    // Test the repeat-finder function.
    three_period(XYZ_TEST1);    // LCM should be 2772
    three_period(XYZ_TEST2);    // LCM should be 4686774924
    three_period(XYZ_INPUT);    // LCM is the Part-2 answer

    return 0;
}

