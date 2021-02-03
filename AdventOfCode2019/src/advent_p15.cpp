// Advent of Code 2019, Day 15
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/15

#include <deque>
#include <set>
#include <sstream>
#include "intcode.h"

static const bool VERBOSE = true;

struct SearchState {
    explicit SearchState(const Program& ref)
        : prog(ref)
        , x(0), y(0)
        , nmoves(0)
        , dir(0)
    {
        // No further initialization.
    }

    // Get ready to try moving...
    explicit SearchState(const SearchState& prev, unsigned _dir)
        : prog(prev.prog)
        , x(prev.x), y(prev.y)
        , nmoves(prev.nmoves+1)
        , dir(_dir)
    {
        // Calculate the new location.
        if (dir == 1) ++y;
        if (dir == 2) --y;
        if (dir == 3) --x;
        if (dir == 4) ++x;
    }

    // Issue a movement command and return result (0/1/2).
    unsigned move()
    {
        // Issue the specified movement command.
        std::stringstream strm; strm << dir;
        int64_t outval;
        if (prog.run_next(&strm, outval) == Program::STATUS_CONTINUE) {
            return (unsigned)outval;
        } else {
            return 0; // Program error or halted
        }
    }

    Program prog;
    int x, y;
    unsigned nmoves, dir;
};

typedef std::deque<SearchState> search_queue;

// Breadth first search to find shortest path through the maze.
SearchState count_moves(const Program& ref)
{
    // Add program initial state to the head of the search queue.
    search_queue queue;
    queue.push_back(SearchState(ref));

    // Keep branching from the head of the BFS queue...
    while (!queue.empty()) {
        for (unsigned dir = 1 ; dir <= 4 ; ++dir) {
            // Discard back-and-forth moves to save time.
            unsigned prev = queue.begin()->dir;
            if ((dir == 1 && prev == 2) ||
                (dir == 2 && prev == 1) ||
                (dir == 3 && prev == 4) ||
                (dir == 4 && prev == 3)) continue;
            // Otherwise, try the next move:
            SearchState next(*queue.begin(), dir);
            unsigned result = next.move();
            if (result == 2) {
                return next;            // Found goal!
            } else if (result == 1) {
                queue.push_back(next);  // Keep searching.
            }
        }
        queue.pop_front();  // Done with current state.
    }

    // Error, can't find goal.
    return SearchState(ref);
}

// Starting from the oxygen generator (see above),
// count maximum moves to reach any other part of the maze.
unsigned count_flood(const SearchState& start)
{
    // Keep track of visited locations.
    typedef std::pair<int,int> location;
    std::set<location> visited;
    visited.insert(std::make_pair(start.x, start.y));

    // Add program initial state to the head of the search queue.
    search_queue queue;
    queue.push_back(start);

    // Keep branching from the head of the BFS queue...
    unsigned max_flood = 0;
    while (!queue.empty()) {
        for (unsigned dir = 1 ; dir <= 4 ; ++dir) {
            // Has the new location already been visited?
            SearchState next(*queue.begin(), dir);
            location pos(std::make_pair(next.x, next.y));
            if (visited.find(pos) != visited.end()) continue;
            // Otherwise, try the next move:
            unsigned result = next.move();
            unsigned nflood = next.nmoves - start.nmoves;
            if (result != 0) {
                // Successful move, update search state.
                if (nflood > max_flood) max_flood = nflood;
                visited.insert(pos);
                queue.push_back(next);
            }
        }
        queue.pop_front();  // Done with current state.
    }

    return max_flood;
}

int main()
{
    // Load program from file and find shortest path.
    Program maze("advent_p15.txt", 1);
    SearchState goal = count_moves(maze);
    std::cout << "Oxygen on in " << goal.nmoves << " moves." << std::endl;
    std::cout << "Oxygen filled in " << count_flood(goal) << " minutes." << std::endl;

    return 0;
}

