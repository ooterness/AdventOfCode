// Advent of Code 2019, Day 7
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/7

#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>
#include "intcode.h"

struct Amplifier
{
    Amplifier(const Program& ref, int phase)
        : m_prog(ref)
        , m_phase(phase)
        , m_first(1)
    {
        // Nothing else to initialize.
    }

    // Continue program until next output.
    int iterate(int input)
    {
        // Configure the input pipe.
        std::stringstream strm_in;
        if (m_first) {
            strm_in << m_phase << ",";
            m_first = 0;
        }
        strm_in << input;
        
        // Continue program until next output.
        int64_t output = -1;
        m_prog.run_next(&strm_in, output);
        return (int)output;
    }

    Program m_prog;
    int m_phase;
    int m_first;
};

typedef std::vector<int> PhaseConfig;

struct AmplifierChain
{
    // Create a chain of amplifiers and set initial conditions.
    AmplifierChain(const Program& ref, const PhaseConfig& phase)
    {
        for (PhaseConfig::const_iterator it = phase.begin() ; it != phase.end() ; ++it)
            m_chain.push_back(Amplifier(ref, *it));
    }

    // Run the entire chain once.
    int iterate(int next)
    {
        for (unsigned a = 0 ; a < m_chain.size() ; ++a)
            next = m_chain[a].iterate(next);
        return next;
    }

    std::vector<Amplifier> m_chain;
};

int max_thrust_part1(const Program& ref)
{
    // Set baseline phase configuration = 0,1,2,3,4
    PhaseConfig cfg(5);
    for (unsigned a = 0 ; a < 5 ; ++a) cfg[a] = a;

    // Lexicographic search of all possible permutations.
    int max_thrust = 0;
    do {
        // Create a new chain and run each one once.
        AmplifierChain chain(ref, cfg);
        int thrust = chain.iterate(0);
        if (thrust > max_thrust)
            max_thrust = thrust;
    } while (next_permutation(cfg.begin(), cfg.end()));

    return max_thrust;
}

int max_thrust_part2(const Program& ref)
{
    // Set baseline phase configuration = 5,6,7,8,9
    PhaseConfig cfg(5);
    for (unsigned a = 0 ; a < 5 ; ++a) cfg[a] = a+5;

    // Lexicographic search of all possible permutations.
    int max_thrust = 0;
    do {
        // Create a new chain and iterate until finished.
        AmplifierChain chain(ref, cfg);
        int thrust = 0;
        while (thrust >= 0) {
            thrust = chain.iterate(thrust);
            if (thrust > max_thrust)
                max_thrust = thrust;
        }
    } while (next_permutation(cfg.begin(), cfg.end()));

    return max_thrust;
}

int main()
{
    // Define each of the programs.
    Program test1("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0");
    Program test2("3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0");
    Program test3("3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0");
    Program test4("3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5");
    Program test5("3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,"\
                  "54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10");
    Program thruster("advent_p7.txt", 1);  // Load from file

    // Unit tests for the forward search function.
    assert (max_thrust_part1(test1) == 43210);
    assert (max_thrust_part1(test2) == 54321);
    assert (max_thrust_part1(test3) == 65210);

    // Find the part-1 solution.
    std::cout << "Max forward thrust = " << max_thrust_part1(thruster) << std::endl;

    // Unit tests for the feedback search function.
    assert (max_thrust_part2(test4) == 139629729);
    assert (max_thrust_part2(test5) == 18216);

    // Find the part-1 solution.
    std::cout << "Max feedback thrust = " << max_thrust_part2(thruster) << std::endl;

    return 0;
}

