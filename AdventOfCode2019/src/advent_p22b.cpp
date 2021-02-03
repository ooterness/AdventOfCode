// Advent of Code 2019, Day 22, Part 2
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/22
// Based on the following modulo-arithmetic tutorials:
//  https://codeforces.com/blog/entry/72593
//  https://codeforces.com/blog/entry/72527

#include <cassert>
#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>

// Modulo-arithmetic wrapper.
struct ModInt {
    uint64_t x, m;

    explicit ModInt(uint64_t _x, uint64_t _m)
        : x(_x % _m), m(_m)
    {
        assert (m >= 2);
    }

    ModInt(const ModInt& b)
        : x(b.x), m(b.m) {}

    // Modulo multiplicative inverse.
    ModInt mmi() const {
        if (m == 10) {
            // Special case for M=10, since it's not prime.
            if (x == 1) return ModInt(1, m);
            if (x == 3) return ModInt(7, m);
            if (x == 7) return ModInt(3, m);
            if (x == 9) return ModInt(9, m);
            return ModInt(0, m);    // No inverse
        } else {
            // All other cases can use Fermat's theorem:
            return *this ^ (m-2);
        }
    }

    ModInt operator+(const ModInt& b) const {
        assert (m == b.m);
        return ModInt(x + b.x, m);
    }

    ModInt operator-(const ModInt& b) const {
        assert (m == b.m);
        return ModInt(m + x - b.x, m);
    }

    ModInt operator*(const ModInt& bb) const {
        assert (m == bb.m);
        // Use the iterated-doubling method.
        uint64_t b = bb.x;
        ModInt dbl(*this), acc(0, m);
        while (b > 0) {
            if (b & 1ULL)
                acc = acc + dbl;
            dbl = dbl + dbl;
            b /= 2;
        }
        return acc;
    }

    ModInt operator^(uint64_t b) const {
        // Use the iterated-squaring method.
        ModInt sqr(*this), acc(1, m);
        while (b > 0) {
            if (b & 1ULL)
                acc = acc * sqr;
            sqr = sqr * sqr;
            b /= 2;
        }
        return acc;
    }

    // "Divide" using modulo multiplicative inverse.
    // (a/b) mod m = (a * b') mod m, where (b*b') mod m = 1.
    ModInt operator/(const ModInt& b) const {
        assert (m == b.m);
        ModInt bp = b.mmi();
        return *this * b.mmi();
    }
};

// Each "shuffle" can be expressed as a linear congruential generator (LCG)
// of the form f(x) = (ax + b) mod M, where M is the deck size.
class Shuffle {
public:
    ModInt a, b;

    explicit Shuffle(ModInt _a, ModInt _b)
        : a(_a), b(_b) {}

    explicit Shuffle(uint64_t _a, uint64_t _b, uint64_t _m)
        : a(_a, _m), b(_b, _m) {}

    uint64_t ncards() const {
        return a.m;
    }

    // Execute the forward operation, i.e., Where does card N end up?
    uint64_t fwd(uint64_t x) const {
        ModInt y = ModInt(x, a.m) * a + b;
        return y.x;
    }

    // Return the inverse operation, i.e., Where did the Nth card come from?
    uint64_t inv(uint64_t y) const {
        ModInt x = (ModInt(y, a.m) - b) / a;
        return x.x;
    }

    // Compose with a second shuffle, i.e., f * g returns h(x) = g(f(x))
    Shuffle operator*(const Shuffle& s) const {
        return Shuffle(a * s.a, b * s.a + s.b);
    }
    
    // Execute a shuffle N times, i.e., h(x) = f^n(x) = f(f(...f(x)))
    Shuffle operator^(uint64_t k) const {
        ModInt ONE(1ULL, a.m);
        ModInt ak = a ^ k;
        return Shuffle(ak, (b * (ONE - ak)) / (ONE - a));
    }

    // Exhaustively print a given Shuffle result.
    void print(bool f=0) const {
        for (unsigned n = 0 ; n < ncards() ; ++n)
            std::cout << (f ? fwd(n) : inv(n)) << " ";
        std::cout << std::endl;
    }
};

// Read a single line and convert to Shuffle command.
Shuffle read_line(uint64_t ncards, std::istream& in)
{
    assert (ncards > 0);

    // Create command from next line of text.
    std::string word; int arg = 0;
    if (in >> word, in) {
        if (word[0] == 'd') {               // "deal..."
            in >> word;
            if (word[0] == 'w') {           // "deal with increment X"
                in >> word >> arg;
                return Shuffle(arg, 0, ncards);
            } else if (word[0] == 'i') {    // "deal into new stack"
                in >> word >> word;
                return Shuffle(ncards-1, ncards-1, ncards);
            } else {
                std::cerr << "Unknown deal: " << word << std::endl;
                in.setstate(std::ios_base::failbit);
            }
        } else if (word[0] == 'c') {        // "cut X"
            in >> arg;
            if (arg < 0)
                return Shuffle(1, -arg, ncards);
            else
                return Shuffle(1, ncards-arg, ncards);
        } else {
            std::cerr << "Unknown command: " << word << std::endl;
            in.setstate(std::ios_base::failbit);
        }
    }

    // Couldn't read command, return a placeholder.
    return Shuffle(0, 0, 10);
}

// Compose any number of steps into a single LCG operation.
Shuffle read_shuffle(uint64_t ncards, std::istream& in, bool verbose=1)
{
    assert (ncards > 0);

    Shuffle result = read_line(ncards, in);
    while (in) {
        Shuffle next = read_line(ncards, in);
        if (in) result = result * next;
    }
    if (verbose)
        std::cout << "Shuffle = " << result.a.x << "x + " << result.b.x << std::endl;
    return result;
}

// Does the shuffle operation result in the given deck?
bool match(const Shuffle& shuffle, const char* str, bool verbose=0)
{
    if (verbose) shuffle.print();

    std::stringstream ref(str);
    unsigned ncards = shuffle.ncards(), idx = 0, val = 0;
    while (ref >> val, ref) {
        if (idx >= ncards) return false;
        if (shuffle.inv(idx++) != val) return false;
    }
    return (idx == ncards);
}

int main()
{
    // Test individual operations.
    std::stringstream test1("deal into new stack");
    assert(match(read_shuffle(10, test1), "9 8 7 6 5 4 3 2 1 0"));

    std::stringstream test2("cut 3");
    assert(match(read_shuffle(10, test2), "3 4 5 6 7 8 9 0 1 2"));

    std::stringstream test3("cut -4");
    assert(match(read_shuffle(10, test3), "6 7 8 9 0 1 2 3 4 5"));

    std::stringstream test4("deal with increment 3");
    assert(match(read_shuffle(10, test4), "0 7 4 1 8 5 2 9 6 3"));

    // Try each of the short test sequences.
    std::stringstream test5(
        "deal with increment 7\n"\
        "deal into new stack\n"\
        "deal into new stack\n");
    assert(match(read_shuffle(10, test5), "0 3 6 9 2 5 8 1 4 7"));

    std::stringstream test6(
        "cut 6\n"\
        "deal with increment 7\n"\
        "deal into new stack\n");
    assert(match(read_shuffle(10, test6), "3 0 7 4 1 8 5 2 9 6"));

    std::stringstream test7(
        "deal with increment 7\n"\
        "deal with increment 9\n"\
        "cut -2\n");
    assert(match(read_shuffle(10, test7), "6 3 0 7 4 1 8 5 2 9"));

    std::stringstream test8(
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
    assert(match(read_shuffle(10, test8), "9 2 5 8 1 4 7 0 3 6"));

    // Read the actual part-1 sequence, then locate card #2019.
    std::ifstream seq1("advent_p22.txt");
    Shuffle part1 = read_shuffle(10007, seq1);
    std::cout << "Card #2019 is at index " << part1.fwd(2019) << std::endl;

    // Read the sequence again
    std::ifstream seq2("advent_p22.txt");
    Shuffle part2a = read_shuffle(119315717514047ULL, seq2);
    Shuffle part2b = part2a ^ 101741582076661ULL;
    std::cout << "Card index 2020 is #" << part2b.inv(2020) << std::endl;

    return 0;
}

