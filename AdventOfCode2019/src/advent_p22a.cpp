// Advent of Code 2019, Day 22, Part 1
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/22
// This file solves Part 1 only.  Due to the nature of
// Part 2, it shares no code in common. :(

#include <cassert>
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

#define VERBOSE 0

struct Deck {
    std::vector<unsigned> cards;

    explicit Deck(unsigned ncards)
        : cards(ncards)
    {
        for (unsigned a = 0 ; a < ncards ; ++a)
            cards[a] = a;
    }

    explicit Deck(std::vector<unsigned>::const_iterator& beg,
                  std::vector<unsigned>::const_iterator& end)
        : cards(beg, end)
    {}

    unsigned size() const {
        return cards.size();
    }

    Deck deal_stack() const {
        if (VERBOSE) std::cout << "Deal-stack " << std::endl;
        Deck result(cards.size());
        for (unsigned a = 0 ; a < cards.size() ; ++a)
            result.cards[a] = *(cards.crbegin() + a);
        return result;
    }

    Deck deal_incr(unsigned incr) const {
        if (VERBOSE) std::cout << "Deal-incr " << incr << std::endl;
        Deck result(cards.size());
        for (unsigned a = 0 ; a < cards.size() ; ++a)
            result.cards[(a*incr) % cards.size()] = cards[a];
        return result;
    }

    Deck cut(int n) const {
        if (VERBOSE) std::cout << "Cut " << n << std::endl;
        Deck result(cards.size());
        unsigned u = (n >= 0) ? n : cards.size() + (unsigned)n;
        for (unsigned a = 0 ; a < cards.size() ; ++a)
            result.cards[a] = cards[(a + u) % cards.size()];
        return result;
    }

    unsigned find(unsigned val)
    {
        for (unsigned a = 0 ; a < cards.size() ; ++a)
            if (cards[a] == val) return a;
        return cards.size();
    }
};

Deck execute(const Deck& init, std::istream& in)
{
    // Start with a copy of the designated deck.
    Deck deck(init);

    // Read and execute each command...
    std::string word; int arg = 0;
    while (in >> word, in) {
        if (word[0] == 'd') {               // "deal with" or "deal into"
            in >> word;
            if (word[0] == 'w') {           // "deal with"
                in >> word >> arg;
                deck = deck.deal_incr(arg);
            } else if (word[0] == 'i') {    // "deal into"
                in >> word >> word;
                deck = deck.deal_stack();
            } else {
                std::cerr << "Unknown deal: " << word << std::endl;
                return deck;
            }
        } else if (word[0] == 'c') {        // "cut"
            in >> arg;
            deck = deck.cut(arg);
        } else {
            std::cerr << "Unknown command: " << word << std::endl;
            return deck;
        }
    }
    return deck;
}

bool match(const Deck& deck, const char* str)
{
    std::stringstream ref(str);
    unsigned ncards = deck.size(), idx = 0, val = 0;
    while (ref >> val, ref) {
        if (idx >= ncards) return false;
        if (deck.cards[idx++] != val) return false;
    }
    return (idx == ncards);
}

int main()
{
    // Test individual operations.
    Deck deck10(10);
    assert(match(deck10.deal_stack(), "9 8 7 6 5 4 3 2 1 0"));
    assert(match(deck10.cut(3), "3 4 5 6 7 8 9 0 1 2"));
    assert(match(deck10.cut(-4), "6 7 8 9 0 1 2 3 4 5"));
    assert(match(deck10.deal_incr(3), "0 7 4 1 8 5 2 9 6 3"));

    // Try each of the short test sequences.
    std::stringstream test1(
        "deal with increment 7\n"\
        "deal into new stack\n"\
        "deal into new stack\n");
    assert(match(execute(deck10, test1), "0 3 6 9 2 5 8 1 4 7"));

    std::stringstream test2(
        "cut 6\n"\
        "deal with increment 7\n"\
        "deal into new stack\n");
    assert(match(execute(deck10, test2), "3 0 7 4 1 8 5 2 9 6"));

    std::stringstream test3(
        "deal with increment 7\n"\
        "deal with increment 9\n"\
        "cut -2\n");
    assert(match(execute(deck10, test3), "6 3 0 7 4 1 8 5 2 9"));

    std::stringstream test4(
        "deal into new stack\n"\
        "cut -2\n"\
        "deal with increment 7\n"\
        "cut 8\n"\
        "cut -4\n"\
        "deal with increment 7\n"\
        "cut 3\n"\
        "deal with increment 9\n"\
        "deal with increment 3\n"\
        "cut -1");
    assert(match(execute(deck10, test4), "9 2 5 8 1 4 7 0 3 6"));

    // Run the actual part-1 sequence, then locate card #2019.
    std::ifstream seq1("advent_p22.txt");
    Deck part1 = execute(Deck(10007), seq1);
    std::cout << "Card #2019 is at index " << part1.find(2019) << std::endl;

    return 0;
}

