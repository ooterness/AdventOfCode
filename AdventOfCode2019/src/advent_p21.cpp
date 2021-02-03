// Advent of Code 2019, Day 21
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/21

#include "intcode.h"

// Run the given SpringScript program, return score (0 on failure)
int64_t spring_run(const Program& ref, const std::string& str, bool verbose=0)
{
    // Convert input program to psuedo-ASCII.
    std::stringstream in, out;
    for (unsigned a = 0 ; a < str.length() ; ++a)
        in << (int)str[a] << ",";

    // Run program to termination.
    Program prog(ref);
    prog.run(&in, &out);

    // Find the first non-ASCII output value.
    int64_t tmp, result = 0;
    while (read_next(out, tmp)) {
        if (tmp > 255) {
            result = tmp;
        } else if (verbose) {
            std::cout << (char)tmp;
        }
    }
    return result;
}

int main()
{
    // One non-obvious factor in the WALK problem is that there is no universal
    // solution.  In WALK mode, jump distance and lookahead are both four spaces.
    // Consider the following situations:
    //   1) ###~#~~~### (Must jump now; walking forward is a dead-end)
    //   2) ###~#~#~~~# (Must walk forward; jumping now leads to a dead-end)
    // These two situations are indistinguishable using the four-space lookahead.
    // As a result, we cannot solve the problem without knowing the map a-priori.
    //
    // Given the limited lookahead, any given SpringScript program can be
    // viewed as a sort of truth table. Naively, there are 2^4 = 16 entries
    // in this truth table, but may are locked down.  Most obviously:
    //   1) If !D, then we must never jump.
    //   2) If !A, then we must always jump.
    // This leaves only 2^2 = 4 unlocked rows in the truth table:
    //      ABCD
    //   1) 1001 = Jump now (no lockout possible, but walking forward may trap)
    //   2) 1011 = Jump now (no lockout possible, but walking forward may trap)
    //   3) 1101 = Undecidable (see trap, above)
    //   4) 1111 = Walk (may see obstacles later)
    //
    // Since the rows 3 and 4 can be made the same, this means there are
    // really only two possible programs to consider:
    //   walk0 = (!A + !B) & D          (Walk in undecideable case.)
    //   walk1 = (!A + !B + !C) & D     (Jump in undecideable case.)
    //
    // Hand-written programs matching constraints above.
    std::string walk0("NOT A T\nNOT B J\nOR T J\nAND D J\nWALK\n");
    std::string walk1("NOT A T\nNOT B J\nOR T J\nNOT C T\nOR T J\nAND D J\nWALK\n");

    // Try each one against the provided interpreter.
    Program ref("advent_p21.txt", 1);
    std::cout << "Walk0 Result = " << spring_run(ref, walk0) << std::endl;
    std::cout << "Walk1 Result = " << spring_run(ref, walk1) << std::endl;

    // With the RUN command, lookahead increases to nine spaces, but jump
    // distance remains at four spaces.  However, some situations remain
    // undecidable.  In fact, any fixed-length lookahead can be trapped
    // with a sufficiently long A/B parity sequence, e.g.:
    //   ###~#~#~#~#~#...#---###
    //
    // With additional lookahead, it's difficult to enumerate rules.
    // A full list of jump/walk/don't-care cases is available in a
    // separate spreadsheet, "advent_p21.ods".
    // The simplest known full-coverage rule is:
    //   D & (!A + H!B + E!C + H!C)
    std::string run0("OR E J\nOR H J\nNOT C T\nAND T J\nNOT B T\nAND H T\nOR T J\nNOT A T\nOR T J\nAND D J\nRUN\n");
    std::cout << "Run0 Result = " << spring_run(ref, run0, 1) << std::endl;

    return 0;
}

