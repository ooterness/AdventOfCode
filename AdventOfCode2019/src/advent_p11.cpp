// Advent of Code 2019, Day 11
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/11

#include <map>
#include <sstream>
#include "intcode.h"

struct Robot
{
    explicit Robot(const Program& ref)
        : prog(ref)
        , x(0), y(0), dir(0)
    {
        // Nothing else to initialize.
    }

    // Run program, return the number of squares painted.
    unsigned run(int verbose=0) {
        unsigned steps = 0;
        std::stringstream strm;

        while (1) {
            // What's the current paint color?
            auto old_color = paint.find(std::make_pair(x,y));
            if (old_color != paint.end())
                strm << old_color->second << ",";
            else
                strm << "0,";   // Not yet painted
            // Run the program for one step.
            int64_t new_color;
            if (prog.run_next(&strm, new_color) == Program::STATUS_CONTINUE) {
                if (verbose > 0) std::cout << "CLR = " << new_color << std::endl;
                paint[std::make_pair(x,y)] = new_color;
                ++steps;
            } else break;
            // Which way to turn for the next movement step?
            int64_t new_turn;
            if (prog.run_next(&strm, new_turn) == Program::STATUS_CONTINUE) {
                if (new_turn > 0) { // Turn right
                    dir = (dir + 3) % 4;
                } else {            // Turn left
                    dir = (dir + 1) % 4;
                }
                if (dir == 0) --y;  // North
                if (dir == 1) --x;  // West
                if (dir == 2) ++y;  // South
                if (dir == 3) ++x;  // East
            } else break;
            // Debugging: Print current map state.
            if (verbose > 0) {
                std::cout << "DIR = " << new_turn << std::endl;
                print_map(true, verbose);
            }
        }

        return steps;
    }

    void print_map(bool numeric=false, int max_dim = 0) {
        // Unless specified, automatically find overall extents.
        int ymin=0, ymax=0, xmin=0, xmax=0;
        if (max_dim > 0) {
            ymin = ymax = xmin = xmax = max_dim;
        } else if (!paint.empty()) {
            xmin = xmax = paint.begin()->first.first;
            ymin = ymax = paint.begin()->first.second;
            for (auto it = paint.begin() ; it != paint.end() ; ++it) {
                int px = it->first.first, py = it->first.second;
                if (px < xmin) xmin = px;
                if (px > xmax) xmax = px;
                if (py < ymin) ymin = py;
                if (py > ymax) ymax = py;
            }
        }
        
        // Iterate over the specified bounds.
        for (int py = -ymin ; py <= ymax ; ++py) {
            for (int px = -xmin ; px <= xmax ; ++px) {
                if (px == x && py == y) {
                    if (dir == 0) std::cout << "^";
                    if (dir == 1) std::cout << "<";
                    if (dir == 2) std::cout << "v";
                    if (dir == 3) std::cout << ">";
                } else if (numeric) {
                    auto c = paint.find(std::make_pair(px,py));
                    if (c != paint.end())
                        std::cout << c->second; // Fresh paint (0/1)
                    else
                        std::cout << ".";       // No paint on this square
                } else {
                    auto c = paint.find(std::make_pair(px,py));
                    if (c != paint.end() && c->second)
                        std::cout << "#";       // White paint
                    else
                        std::cout << ".";       // No paint or black paint
                }
            }
            std::cout << std::endl;
        }
    }

    void print_paint() {
        for (auto it = paint.begin() ; it != paint.end() ; ++it) {
            std::cout << "(" << it->first.first
                      << "," << it->first.second
                      << "," << it->second << ")" << std::endl;
        }
    }

    Program prog;
    int x, y;
    unsigned dir;
    std::map<std::pair<int,int>, unsigned> paint;
};

int main()
{
    static const bool VERBOSE = false;

    // Test program ignores input and emits the example move sequence.
    Program test("104,1,104,0,104,0,104,0,104,1,104,0,104,1,104,0,"\
                 "104,0,104,1,104,1,104,0,104,1,104,0,99");
    Robot robot1(test); robot1.run(VERBOSE ? 2 : 0);
    if (VERBOSE) robot1.print_paint();
    assert (robot1.paint.size() == 6);

    // Part-1: Run the program and count painted squares.
    Program prog("advent_p11.txt", 1);
    Robot robot2(prog); robot2.run();
    std::cout << "P1: Painted squares: " << robot2.paint.size() << std::endl;

    // Part-2: Set different initial conditions and try again.
    Robot robot3(prog);
    robot3.paint[std::make_pair(0,0)]=1;
    robot3.run();
    std::cout << "P2: Painted squares: " << robot3.paint.size() << std::endl;
    robot3.print_map();

    return 0;
}

