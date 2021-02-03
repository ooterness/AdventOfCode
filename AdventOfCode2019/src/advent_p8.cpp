// Advent of Code 2019, Day 8
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/8

#include <cassert>
#include <fstream>
#include <iostream>
#include <vector>

static const unsigned NROWS = 6;
static const unsigned NCOLS = 25;
static const unsigned NPIXL = NROWS * NCOLS;

struct Layer
{
    explicit Layer(unsigned char fill) {
        for (unsigned a = 0 ; a < NPIXL ; ++a) {
            data[a] = fill;
        }
    }

    explicit Layer(std::istream& in) {
        for (unsigned a = 0 ; a < NPIXL ; ++a) {
            char tmp = in.get();
            data[a] = (unsigned char)(tmp - '0');
        }
    }

    unsigned count_x(unsigned char val)
    {
        unsigned count = 0;
        for (unsigned a = 0 ; a < NPIXL ; ++a) {
            if (data[a] == val) ++count;
        }
        return count;
    }

    void render(const Layer& under)
    {
        for (unsigned a = 0 ; a < NPIXL ; ++a) {
            // If transparent, take the pixel underneath.
            if (data[a] == 2) data[a] = under.data[a];
        }
    }

    void print()
    {
        for (unsigned a = 0 ; a < NPIXL ; ++a) {
            std::cout << (data[a] ? 'X' : ' ');
            if (((a+1) % NCOLS) == 0) std::cout << std::endl;
        }        
    }

    unsigned char data[NPIXL];
};

int main()
{
    // Read all layers from the input file.
    std::ifstream inref("advent_p8.txt");
    std::vector<Layer> layers;
    while (inref) {
        Layer tmp(inref);
        if (inref) layers.push_back(tmp);
    }

    // Find the one with the fewest zeros.
    unsigned part1_zcount = NPIXL, part1_result = 0;
    for (unsigned a = 0 ; a < layers.size() ; ++a) {
        unsigned zcount = layers[a].count_x(0);
        if (zcount < part1_zcount) {
            part1_zcount = zcount;
            part1_result = layers[a].count_x(1) * layers[a].count_x(2);
        }
    }
    std::cout << "Part1 result = " << part1_zcount
              << " / " << part1_result << std::endl;

    // Render the layer stackup.
    Layer result(2);
    for (unsigned a = 0 ; a < layers.size() ; ++a)
        result.render(layers[a]);
    result.print();

    return 0;
}

