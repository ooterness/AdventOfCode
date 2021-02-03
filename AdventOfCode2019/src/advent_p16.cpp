// Advent of Code 2019, Day 16
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/16

#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

// Calculate "Flawed Frequency Transmission" of a full or partial vector.
// Partial vector takes advantage of the fact that the last N digits of
// an FFT only depend on the last N digits of the input.
std::string fft(const std::string& x, unsigned offset=0)
{
    unsigned LEN = x.length();

    // Pre-compute cumulative sums so the loops below are much faster.
    int* cumsum = new int[LEN+1];
    cumsum[0] = 0;
    for (unsigned a = 0 ; a < LEN ; ++a)
        cumsum[a+1] = cumsum[a] + (x[a] - '0');

    // For each output digit...
    std::string y(LEN, '0');
    for (unsigned a = 0 ; a < LEN ; ++a) {
        // Calculate the inner product with the repeating pattern.
        // Use nested counters rather than index-multiply-accumulate.
        int sum = 0;        // Running sum for the current output
        unsigned b = a;     // Input index (diagonal always starts a +1 block)
        while (b < LEN) {
            // Increment sum over the +1 block.
            if (b < LEN) {
                unsigned p1 = std::min(LEN, b+a+offset+1);
                sum += cumsum[p1] - cumsum[b];
            }
            // Skip over the zero block.
            b += 2*(a+offset+1);
            // Decrement sum over the -1 block.
            if (b < LEN) {
                unsigned p1 = std::min(LEN, b+a+offset+1);
                sum -= cumsum[p1] - cumsum[b];
            }
            // Skip over the zero block.
            b += 2*(a+offset+1);
        }
        // Extract and re-encode the final digit.
        y[a] = '0' + (char)(abs(sum) % 10);
    }

    delete[] cumsum;
    return y;
}

// Iterate offset-"FFT" 100 times, then return the first 8 digits.
std::string fft100(const std::string& x, unsigned offset=0, unsigned print_every=0)
{
    std::string y(x);
    for (unsigned a = 0 ; a < 100 ; ++a) {
        y = fft(y, offset);
        if (print_every && (a % print_every) == 0)
            std::cout << "Finished phase " << a+1 << std::endl;
    }
    return y.substr(0,8);
}

// Repeat input string 10,000 times, starting from designated offset.
std::string repeat10k(const std::string& x, unsigned offset)
{
    unsigned len = x.length() * 10000 - offset;
    std::string y(len, '0');
    for (unsigned a = 0 ; a < len ; ++a)
        y[a] = x[(a+offset) % x.length()];
    return y;
}

int main()
{
    // Unit tests from the problem statement:
    assert(fft("12345678") == std::string("48226158"));
    assert(fft("48226158") == std::string("34040438"));
    assert(fft("34040438") == std::string("03415518"));
    assert(fft("03415518") == std::string("01029498"));
    assert(fft100("80871224585914546619083218645595") == std::string("24176176"));
    assert(fft100("19617804207202209144916044189917") == std::string("73745418"));
    assert(fft100("69317163492948606335995924319873") == std::string("52432133"));
    std::cout << "Finished BIST 1" << std::endl;

    // Read input from file.
    std::ifstream infile("advent_p16.txt");
    std::string input;
    getline(infile, input);

    // Part-1 solution:
    std::string part1 = fft100(input);
    std::cout << "Part-1: " << part1.substr(0,8) << std::endl;

    // Unit-tests for the offset FFT.
    unsigned offset;
    offset = 303673;
    std::string test1 = repeat10k("03036732577212944063491565474664", offset);
    assert(fft100(test1, offset) == std::string("84462026"));
    std::cout << "Finished BIST 2a" << std::endl;
    offset = 293510;
    std::string test2 = repeat10k("02935109699940807407585447034323", offset);
    assert(fft100(test2, offset) == std::string("78725270"));
    std::cout << "Finished BIST 2b" << std::endl;
    offset = 308177;
    std::string test3 = repeat10k("03081770884921959731165446850517", offset);
    assert(fft100(test3, offset) == std::string("53553731"));
    std::cout << "Finished BIST 2c" << std::endl;

    // Part-2 solution uses offset for massive speedup.
    offset = 5972351;
    std::string part2_in = repeat10k(input, offset);
    std::cout << "Part-2: " << fft100(part2_in, offset) << std::endl;

    return 0;
}

