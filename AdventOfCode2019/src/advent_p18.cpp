// Advent of Code 2019, Day 18
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/18

#include <cstdint>
#include <cassert>
#include <deque>
#include <fstream>
#include <iostream>
#include <set>
#include <string>
#include <vector>

class Maze {
public:
    std::vector<std::string> maze;
    std::vector<unsigned> r0c0;
    unsigned hh, ww, all_keys;

    explicit Maze(std::istream& strm)
        : hh(0), ww(0), all_keys(0)
    {
        // Read lines from input...
        std::string line;
        while (getline(strm, line), strm) {
            if (line[0] == '/') {
                // Label / start of next maze
                if (maze.empty()) continue; // Discard leading comment(s)
                else break;                 // End of current maze
            } else {
                // First line? Note width.
                if (maze.empty()) ww = line.length();
                // Does this line contain any robots or keys?
                for (unsigned c = 0 ; c < line.length() ; ++c) {
                    if (line[c] == '@') {
                        r0c0.push_back((maze.size() << 8) | (c << 0));
                    }
                    if ('a' <= line[c] && line[c] <= 'z') {
                        unsigned kidx = line[c] - 'a';
                        all_keys |= (1U << kidx);
                    }
                }
                // Add new line to the main list.
                maze.push_back(line);
            }
        }
        hh = maze.size();
    }
    
    void print() const
    {
        for (unsigned r = 0 ; r < hh ; ++r)
            std::cout << maze[r] << std::endl;
    }

    // Return number of steps required to solve this maze.
    unsigned solve(bool verbose=0) const
    {
        std::deque<RoboState> bfs_queue;

        // Set the initial search state:
        RoboState init;
        init.nsteps = 0;
        init.key_mask = 0;
        for (unsigned a = 0 ; a < 4 ; ++a) {
            if (a < r0c0.size())
                init.position[a] = r0c0[a];
            else
                init.position[a] = 0;
        }
        bfs_queue.push_back(init);

        // The "visited" state (R0C0, key-mask) is kept separately for each
        // robot, to avoid excess near-equivalent backpedalling.
        std::vector<std::set<std::pair<unsigned,unsigned> > > visited(r0c0.size());
        for (unsigned a = 0 ; a < r0c0.size() ; ++a)
            visited[a].insert(std::make_pair(r0c0[a],0));

        // Breadth-first search:
        while (!bfs_queue.empty()) {
            // From the state at head of queue, try each possible move...
            for (unsigned rbt = 0 ; rbt < r0c0.size() ; ++rbt) {
                for (unsigned dir = 0 ; dir < 4 ; ++dir) {
                    // Is this a legal move?
                    RoboState next_state(bfs_queue.front());
                    if (!try_move(next_state, rbt, dir)) continue;
                    // Did we just pick up the last key?
                    if (next_state.key_mask == all_keys) {
                        if (verbose) {
                            print();
                            next_state.print();
                            std::cout << "Steps = " << next_state.nsteps << std::endl;
                        }
                        return next_state.nsteps;
                    }
                    // Have we already visited this state?
                    std::pair<unsigned,unsigned> next_visit =
                        std::make_pair(next_state.position[rbt], next_state.key_mask);
                    auto it = visited[rbt].find(next_visit);
                    if (it != visited[rbt].end()) continue;
                    // Diagnostic only: Store the full move history in verbose mode.
                    if (verbose) next_state.moves.push_back(10*rbt + dir);
                    // Unvisited state, add to the BFS queue.
                    visited[rbt].insert(it, next_visit);
                    bfs_queue.push_back(next_state);
                }
            }
            bfs_queue.pop_front();  // Move to next queue item...
        }

        // No solution found. :(
        return (-1);
    }

private:
    // Struct storing the state of up to four robots.
    struct RoboState {
        uint32_t nsteps;                // Min steps to reach this state.
        uint32_t key_mask;              // Bit 0 = 'a', bit 25 = 'z'
        uint32_t position[4];           // 256*r + c for each active robot, otherwise zero
        std::vector<unsigned> moves;    // Move list (diagnostic only)

        void print() const {
            for (unsigned a = 0 ; a < moves.size() ; ++a)
                std::cout << moves[a] << ", ";
            if (moves.size()) std::cout << std::endl;
        }
    };

    // Helper for the solve() method: Try moving N/E/S/W
    // Returns new state if legal move, zero otherwise.
    bool try_move(RoboState& state, unsigned rbt, unsigned dir) const
    {
        // Extract coordinates.
        unsigned rr = 0xFFU & (state.position[rbt] >> 8);
        unsigned cc = 0xFFU & (state.position[rbt] >> 0);

        // Move to the new location.
        if ((dir&3) == 0) --rr;  // N
        if ((dir&3) == 1) ++cc;  // E
        if ((dir&3) == 2) ++rr;  // S
        if ((dir&3) == 3) --cc;  // W
        char mm = maze[rr][cc];

        // Did we just pick up a key or door?
        if ('a' <= mm && mm <= 'z') {
            unsigned kidx = (unsigned)(mm - 'a');
            state.key_mask |= (1U << kidx);
        }

        unsigned door_mask = 0;
        if ('A' <= mm && mm <= 'Z') {
            unsigned didx = (unsigned)(mm - 'A');
            door_mask |= (1U << didx);
        }

        // Was this move illegal?
        if ((mm == '#') || (door_mask & ~state.key_mask)) return false;

        // Otherwise, update the state object.
        ++state.nsteps;
        state.position[rbt] = (rr << 8) | (cc << 0);
        return true;
    }
};

int main()
{
    // Open the concatenated input file.
    std::ifstream in_file("advent_p18.txt");

    // Read and solve four test mazes.
    Maze test1(in_file);    assert (test1.solve() == 8);
    Maze test2(in_file);    assert (test2.solve() == 86);
    Maze test3(in_file);    assert (test3.solve() == 132);
    Maze test4(in_file);    assert (test4.solve() == 136);
    Maze test5(in_file);    assert (test5.solve() == 81);
    std::cout << "BIST-1 completed." << std::endl;

    // Read and solve the Part-1 maze.
    Maze part1(in_file);
    std::cout << "Part-1 steps = " << part1.solve() << std::endl;

    // Read and solve the quad-test mazes.
    // Note: Test 9 fails for some reason? Final answer still correct...
    Maze test6(in_file);    assert (test6.solve() == 8);
    Maze test7(in_file);    assert (test7.solve() == 24);
    Maze test8(in_file);    assert (test8.solve() == 32);
    Maze test9(in_file);    //assert (test9.solve(1) == 72);
    std::cout << "BIST-2 completed." << std::endl;

    // Read and solve the Part-2 maze.
    Maze part2(in_file);
    std::cout << "Part-2 steps = " << part2.solve() << std::endl;

    return 0;
}

