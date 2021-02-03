// Advent of Code 2019, Day 3
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/3

#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>

// A generic X/Y grid, with offset from origin.
typedef unsigned short cell_type;
class XyGrid {
    public:
    XyGrid (unsigned size)
        : m_grid(4*size*size)
        , m_offset(size)
        , m_size(2*size)
    {
        // Nothing else to initialize.
    }

    unsigned get_idx(int x, int y) const {
        unsigned ux = (unsigned)(x + m_offset);
        unsigned uy = (unsigned)(y + m_offset);
        assert ((ux < m_size) && (uy < m_size));
        return uy*m_size + ux;
    }

    std::vector<cell_type> m_grid;
    const int m_offset;
    const unsigned m_size;
};

// Apply the given wire path to a grid object.
// Set a constant mask, or set it to the step counter.
void apply_wire(XyGrid& grid, const std::string& instr, cell_type mask)
{
    // Create stream object from instruction string.
    std::istringstream strm(instr);

    // Execute each instruction until we reach the end.
    int px = 0, py = 0;
    cell_type ps = 0;
    while (strm) {
        // Read the next instruction.
        char dir, comma; unsigned len;
        strm >> dir >> len; // Attempt to read instruction
        strm >> comma;      // Consume comma, if present

        int dx = 0, dy = 0;
        if (dir == 'u' || dir == 'U') {
            dy = +1;
        } else if (dir == 'd' || dir == 'D') {
            dy = -1;
        } else if (dir == 'l' || dir == 'L') {
            dx = -1;
        } else if (dir == 'r' || dir == 'R') {
            dx = +1;
        } else {
            assert (0);     // Invalid instruction!?
        }

        // Apply mask for N steps in specified direction.
        for (unsigned n = 0 ; n < len ; ++n) {
            px += dx; py += dy; ++ps;
            unsigned idx = grid.get_idx(px, py);
            if (mask)
                grid.m_grid[idx] |= mask;
            else if (!grid.m_grid[idx])
                grid.m_grid[idx] = ps;
        }
    }
}

// Simulate two wires and return the Manhattan distance to the crossing point.
unsigned nearest_crossing(std::istream& in, int max_size = 1000)
{
    // Read the next two lines.
    std::string wire1, wire2;
    std::getline(in, wire1);
    std::getline(in, wire2);

    // Write out the path for each wire using a different bit mask.
    XyGrid grid(max_size);
    apply_wire(grid, wire1, 0x01);
    apply_wire(grid, wire2, 0x02);

    // Breadth-first search by increasing Manhattan distance.
    for (int d = 1 ; d < max_size ; ++d) {
        for (int p = 0 ; p < d ; ++p) {
            unsigned idx1 = grid.get_idx(-d+p, p);  // W sloping NE
            unsigned idx2 = grid.get_idx(p, d-p);   // N sloping SE
            unsigned idx3 = grid.get_idx(d-p, -p);  // E sloping SW
            unsigned idx4 = grid.get_idx(-p, -d-p); // S sloping NW
            if ((grid.m_grid[idx1] == 0x03) ||
                (grid.m_grid[idx2] == 0x03) ||
                (grid.m_grid[idx3] == 0x03) ||
                (grid.m_grid[idx4] == 0x03)) {
                return (unsigned)d;
            }
        }
    }
    return 0;
}

// As nearest_crossing, but find the shortest combined distance.
unsigned fastest_crossing(std::istream& in, int max_size = 1000)
{
    // Read the next two lines.
    std::string wire1, wire2;
    std::getline(in, wire1);
    std::getline(in, wire2);

    // Write out a separate grid for each wire.
    XyGrid grid1(max_size), grid2(max_size);
    apply_wire(grid1, wire1, 0);
    apply_wire(grid2, wire2, 0);

    // Brute-force search for the smallest sum.
    unsigned min_sum = -1;
    unsigned max_idx = (unsigned)(4*max_size*max_size);
    for (unsigned idx = 0 ; idx < max_idx ; ++idx) {
        if (grid1.m_grid[idx] && grid2.m_grid[idx]) {
            unsigned sum = grid1.m_grid[idx] + grid2.m_grid[idx];
            if (sum < min_sum) min_sum = sum;
        }
    }
    return min_sum;
}

int main()
{
    unsigned d1, d2, d3, d4;
    std::ifstream in("advent_p3.txt");

    // First 6 lines are the tests from the instructions.
    d1 = nearest_crossing(in);
    d2 = nearest_crossing(in);
    d3 = nearest_crossing(in);
    assert (d1 == 6);
    assert (d2 == 159);
    assert (d3 == 135);
    std::cout << "BIST passed." << std::endl;

    // Calculate the nearest crossing (Phase 1).
    d4 = nearest_crossing(in, 12000);
    std::cout << "Nearest crossing distance = " << d4 << std::endl;

    // Reset to the beginning of the file and repeat the tests.
    in.seekg(0, std::ios::beg); in.clear();
    d1 = fastest_crossing(in);
    d2 = fastest_crossing(in);
    d3 = fastest_crossing(in);
    assert (d1 == 30);
    assert (d2 == 610);
    assert (d3 == 410);

    // Calculate the fastest crossing (Phase 2)
    d4 = fastest_crossing(in, 15000);
    std::cout << "Nearest crossing distance = " << d4 << std::endl;

    return 0;
}

