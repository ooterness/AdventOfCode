// Advent of Code 2019, Day 14
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/14

#include <cassert>
#include <climits>
#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <vector>

typedef std::map<std::string, unsigned> ChemList;
typedef std::vector<int64_t> ChemStocks;

// Prototypes
struct Reagent;
struct Reaction;
struct ReactionList;
std::ostream& operator<<(std::ostream& strm, const Reagent& r);
std::ostream& operator<<(std::ostream& strm, const Reaction& r);
std::ostream& operator<<(std::ostream& strm, const ReactionList& rl);

// Helper functions for reading string data.
bool is_character(std::istream& strm)
{
    int c = strm.peek();
    if ('a' <= c && c <= 'z') return true;
    if ('A' <= c && c <= 'Z') return true;
    return false;
}

bool is_delimiter(std::istream& strm, bool arrow=false)
{
    int c = strm.peek();
    if (c < 0) return false;
    if ((c == ' ') || (c == ',') || (c == '\r') || (c == '\n')) return true;
    if (arrow && ((c == '=') || (c == '>'))) return true;
    return false;
}

// A single reagent from a multi-compound reaction.
struct Reagent {
    std::string name;
    unsigned id;
    int64_t qty;

    explicit Reagent() : id(-1), qty(-1) {}

    explicit Reagent(ChemList& chems, std::istream& strm)
    {
        // Consume any leading format characters.
        while (is_delimiter(strm,1)) strm.get();

        // Read quantity, consume any trailing spaces.
        strm >> qty;
        while (is_delimiter(strm)) strm.get();

        // Read name, consume any trailing spaces / commas.
        while (is_character(strm)) name += strm.get();
        while (is_delimiter(strm)) strm.get();

        // Map name to ID, or create a new ID.
        auto it = chems.find(name);
        if (it != chems.end()) {
            id = it->second;
        } else {
            id = chems.size();
            chems[name] = id;
        }
    }

};

// A reaction with one or more inputs and a single output.
struct Reaction {
    std::deque<Reagent> in;
    Reagent out;

    explicit Reaction(ChemList& chems, const std::string& line)
    {
        // Read input reagents up to the "=>" token, then read the output.
        std::stringstream strm(line);
        while (strm.peek() != '=')
            in.push_back(Reagent(chems, strm));
        out = Reagent(chems, strm);
    }

    void execute(ChemStocks& stocks, int64_t mul, bool verbose=0) const
    {
        if (verbose) std::cout << "React = " << *this;
        stocks[out.id] += out.qty * mul;
        for (auto it = in.begin() ; it != in.end() ; ++it)
            stocks[it->id] -= it->qty * mul;
    }
};

// A list of reactions, organized by output type.
struct ReactionList {
    static const unsigned ID_ORE = 0;
    static const unsigned ID_FUEL = 1;

    ChemList chems;
    std::vector<std::deque<Reaction> > reactions;

    explicit ReactionList(std::istream& strm)
    {
        // Seed dictionary with "ORE" at index 0 and "FUEL" at index 1.
        chems["ORE"]  = ID_ORE;
        chems["FUEL"] = ID_FUEL;

        // Read each line from the reaction list.
        std::string line;
        while (getline(strm, line), strm) {
            // Is this a comment line?
            if (line[0] == '/') {
                // Comment lines mark boundaries, otherwise ignored.
                if (reactions.empty())
                    continue;   // Discard leading comments
                else
                    break;      // Stop at beginning of next block
            } else {
                // Parse the current line of text.
                Reaction temp(chems, line);
                // File this reaction according to its output.
                if (temp.out.id >= reactions.size())
                    reactions.resize(temp.out.id+1);
                reactions[temp.out.id].push_back(temp);
            }
        }
    }

    // Solve for minimum ore required to obtain X units of fuel.
    int64_t solve1(int64_t fuel=1, bool verbose=0) const
    {
        if (verbose) std::cout << *this;
    
        // Initial state has a debit of N units FUEL.
        ChemStocks init(chems.size(), 0);
        init[1] = -fuel;

        // Set initial conditions for depth-first search.
        std::deque<ChemStocks> queue;
        queue.push_back(init);

        // Execute the search queue.
        int64_t min_ore = INT64_MAX;
        while (!queue.empty()) {
            // Find the first non-ORE reagent with negative quantity,
            // then attempt each action that could produce that item.
            bool pending = true;
            for (unsigned need = ID_FUEL ; pending && need < chems.size() ; ++need) {
                if (queue.front()[need] >= 0) continue;
                pending = false;
                for (auto r = reactions[need].begin() ; r != reactions[need].end() ; ++r) {
                    // How many times can we run reaction? (Round up)
                    // TODO: Is greedy method universally correct? How to prove this?
                    int64_t mul = (-queue.front()[need]+r->out.qty-1) / r->out.qty;
                    // Create the new state...
                    ChemStocks next(queue.front());
                    r->execute(next, mul, verbose);
                    queue.push_back(next);
                }
            }
            // If we found a solution, update minimum ORE count.
            if (pending) {
                int64_t ore = -queue.front()[0];
                if (verbose) std::cout << "Found = " << ore << std::endl;
                if (ore < min_ore) min_ore = ore;
            }
            // Done with current state, proceed to next.
            queue.pop_front();
        }

        if (verbose) std::cout << "Final = " << min_ore << std::endl;
        return min_ore;
    }

    // Solve for maximum fuel given X units of ore.
    int64_t solve2(int64_t ore, bool verbose=0) const
    {
        // Use the one-fuel case to find a lower bound.
        int64_t ore1 = solve1(1);
        int64_t fuel_min = ore / ore1;
        if (verbose) std::cout << "Initial " << fuel_min << std::endl;

        // Keep doubling until we find an upper bound.
        int64_t fuel_max = fuel_min;
        while (solve1(fuel_max) < ore) {
            fuel_max *= 2;
            if (verbose) std::cout << "Probing " << fuel_max << std::endl;
        }

        // Binary search to find the exact limit.
        // Note: Fuel-min is the largest that we know works.
        //       Fuel-max is the smallest that we know doesn't work.
        while (fuel_min + 1 < fuel_max) {
            if (verbose)
                std::cout << "Testing " << fuel_min << " - " << fuel_max << std::endl;
            int64_t guess = (fuel_min + fuel_max + 1) / 2;
            if (solve1(guess) < ore) {
                fuel_min = guess;
            } else {
                fuel_max = guess;
            }
        }
        return fuel_min;
    }
};

// Print formatting helpers
std::ostream& operator<<(std::ostream& strm, const Reagent& r) {
    return strm << r.name << "[" << r.id << "]x" << r.qty;
}
std::ostream& operator<<(std::ostream& strm, const Reaction& r) {
    for (auto it = r.in.begin() ; it != r.in.end() ; ++it)
        strm << (*it) << " ";
    return strm << "=> " << r.out << std::endl;
}
std::ostream& operator<<(std::ostream& strm, const ReactionList& rl) {
    unsigned rcount = 0;
    for (auto a = rl.reactions.begin() ; a != rl.reactions.end() ; ++a) {
        for (auto b = a->begin() ; b != a->end() ; ++b) {
            strm << *b;
            ++rcount;
        }
    }
    return strm << "Found " << rl.chems.size() << " compounds in "
                << rcount << " reactions." << std::endl;
}

int main()
{
    // Open the main input file.
    std::ifstream in_file("advent_p14.txt");

    // Unit tests using each of the Part-1 examples.
    ReactionList test1(in_file);    assert(test1.solve1() == 31);
    ReactionList test2(in_file);    assert(test2.solve1() == 165);
    ReactionList test3(in_file);    assert(test3.solve1() == 13312);
    ReactionList test4(in_file);    assert(test4.solve1() == 180697);
    ReactionList test5(in_file);    assert(test5.solve1() == 2210736);

    // Part-1 solution.
    ReactionList part1(in_file);
    std::cout << "Part-1 ore = " << part1.solve1() << std::endl;

    // Unit tests using each of the Part-2 examples.
    static const int64_t ORE_COUNT = 1000000000000LL;
    assert(test3.solve2(ORE_COUNT) == 82892753LL);
    assert(test4.solve2(ORE_COUNT) == 5586022LL);
    assert(test5.solve2(ORE_COUNT) == 460664LL);

    // Part-2 solution.
    std::cout << "Part-2 fuel = " << part1.solve2(ORE_COUNT) << std::endl;

    return 0;
}

