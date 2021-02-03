// Advent of Code 2019, Day 20
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/20

#include <deque>
#include <fstream>
#include <iostream>
#include <map>
#include <set>
#include <sstream>
#include <vector>
#include "intcode.h"

// Each Point is a row/column pair
typedef std::pair<unsigned, unsigned> Point;
#define MkPt(x,y) std::make_pair(x, y)

unsigned distance(const Point& a, const Point& b)
{
    unsigned d = 0;
    if (a.first < b.first)
        d += b.first - a.first;
    else
        d += a.first - b.first;
    if (a.second < b.second)
        d += b.second - a.second;
    else
        d += a.second - b.second;
    return d;
}

// Miscellaneous string utility functions.
bool is_alpha(char c)
{
    return ('a' <= c && c <= 'z')
        || ('A' <= c && c <= 'Z');
}

bool is_open(char c)
{
    return (c == '.');
}

std::string make_str(char c1, char c2)
{
    std::string str(2, 'X');
    str[0] = c1;
    str[1] = c2;
    return str;
}

// Read next line, return true if successful; false if it's EOF or a header line.
typedef std::vector<std::string> MazeText;
bool read_line(std::istream& strm, MazeText& maze)
{
    // Attempt to read next line.
    std::string line;
    getline(strm, line);
    if (!strm) return false;    // EOF

    // Ignore header(s) on first line only (start of new maze)
    while (maze.empty() && line[0] == '/')
        getline(strm, line);

    // Any subsequent header lines mark end of input.
    // Otherwise, add it to the output vector.
    if (line[0] == '/') {
        return false;
    } else {
        maze.push_back(line);
        return true;
    }
}

struct Maze {
    std::map<Point, std::vector<Point> > graph;
    Point start, finish;
    unsigned nrows, ncols;

    static const unsigned NO_SOLUTION = -1;

    explicit Maze(std::istream& strm)
    {
        // Read lines up to the next header row.
        MazeText maze;
        while (read_line(strm, maze)) {}
        nrows = maze.size();
        ncols = maze[0].length();

        // Find all ordinary passages and two-letter links in a single pass.
        // (Two-letter links are to/from the point any letter with a "." on
        //  one side and another letter on the other. Find each pair.)
        std::map<std::string, std::vector<Point> > links;
        for (unsigned r = 1 ; r < nrows-1 ; ++r) {
            for (unsigned c = 1 ; c < ncols-1 ; ++c) {
                // Find all the ordinary adjacent linkages.
                if (is_open(maze[r][c])) {
                    if (is_open(maze[r-1][c]))
                        graph[MkPt(r,c)].push_back(MkPt(r-1,c));
                    if (is_open(maze[r][c-1]))
                        graph[MkPt(r,c)].push_back(MkPt(r,c-1));
                    if (is_open(maze[r+1][c]))
                        graph[MkPt(r,c)].push_back(MkPt(r+1,c));
                    if (is_open(maze[r][c+1]))
                        graph[MkPt(r,c)].push_back(MkPt(r,c+1));
                }
                // Find any two-letter linkages.
                if (is_alpha(maze[r][c])) {
                    if (is_alpha(maze[r-1][c]) && is_open(maze[r+1][c]))
                        links[make_str(maze[r-1][c], maze[r][c])].push_back(MkPt(r+1,c));
                    if (is_alpha(maze[r+1][c]) && is_open(maze[r-1][c]))
                        links[make_str(maze[r][c], maze[r+1][c])].push_back(MkPt(r-1,c));
                    if (is_alpha(maze[r][c-1]) && is_open(maze[r][c+1]))
                        links[make_str(maze[r][c-1], maze[r][c])].push_back(MkPt(r,c+1));
                    if (is_alpha(maze[r][c+1]) && is_open(maze[r][c-1]))
                        links[make_str(maze[r][c], maze[r][c+1])].push_back(MkPt(r,c-1));
                }
            }
        }

        // Connect each pair of two-letter linkages.
        for (auto it = links.begin() ; it != links.end() ; ++it) {
            if (it->first == "AA") {
                start = it->second[0];
            } else if (it->first == "ZZ") {
                finish = it->second[0];
            } else if (it->second.size() == 2) {
                graph[it->second[0]].push_back(it->second[1]);
                graph[it->second[1]].push_back(it->second[0]);
            } else {
                std::cout << "Error: Orphan link " << it->first << std::endl;
            }
        }
    }

    // Structure holding a point, recursion level, and step-count
    struct RPoint {
        Point pt;
        unsigned lvl;
        unsigned nsteps;

        explicit RPoint(Point _pt, unsigned _lvl, unsigned _steps)
            : pt(_pt), lvl(_lvl), nsteps(_steps) {}

        explicit RPoint(const RPoint& other)
            : pt(other.pt), lvl(other.lvl), nsteps(other.nsteps) {}

        // Comparison operator is for the "visited" set, ignore step count.
        bool operator<(const RPoint& other) const {
            if (lvl < other.lvl) return true;
            if (lvl > other.lvl) return false;
            if (pt < other.pt) return true;
            return false;
        }
    };

    // Solve maze and return required number of steps.
    unsigned solve(bool recursive, bool verbose=0) const
    {
        // Set initial state.
        std::deque<RPoint> queue;
        std::set<RPoint> visited;
        RPoint init(start, 0, 1);
        queue.push_back(init);
        visited.insert(init);

        // Breadth-first search.
        while (!queue.empty()) {
            // For the point at the head of the queue...
            const RPoint& from = queue.front();
            if (verbose) {
                std::cout << "@N = " << from.nsteps << ": "
                    << "R=" << from.pt.first << ", "
                    << "C=" << from.pt.second << ", "
                    << "L=" << from.lvl << std::endl;
            }
            // Lookup adjacency graph for this point.
            auto adj = graph.find(from.pt);
            if (adj == graph.end()) {
                std::cout << "Error: No such point." << std::endl;
                return NO_SOLUTION;
            }
            // For each adjacent point...
            for (auto to = adj->second.begin() ; to != adj->second.end() ; ++to) {
                // Did we just reach the exit?
                if ((from.lvl == 0) && (*to == finish)) {
                    if (verbose) std::cout << "Solved in " << from.nsteps << std::endl;
                    return from.nsteps;
                }
                // Are we recursing?  Is this legal?
                unsigned next_lvl = from.lvl;
                unsigned d = distance(from.pt, *to);
                if (recursive && (d > 1)) {
                    bool outer = (from.pt.first == 2)
                              || (from.pt.second == 2)
                              || (from.pt.first == nrows-3)
                              || (from.pt.second == ncols-3);
                    if (outer && from.lvl == 0) {
                        continue;   // Illegal move
                    } else if (outer && from.lvl >= 100) {
                        continue;   // Recursion limit
                    } else if (outer) {
                        --next_lvl; // Recurse out
                    } else {
                        ++next_lvl; // Recurse in
                    }
                }
                // Have we already visited this location?
                RPoint next(*to, next_lvl, from.nsteps+1);
                auto v = visited.find(next);
                if (v != visited.end()) continue;
                // Print recursion-change status messages.
                if (verbose && next_lvl < from.lvl) std::cout << "  (Recurse out)" << std::endl;
                if (verbose && next_lvl > from.lvl) std::cout << "  (Recurse in)" << std::endl;
                // Add new points to the end of the search queue.
                queue.push_back(next);
                visited.insert(next);
            }
            // Pop head of queue before continuing.
            queue.pop_front();
        }
        return NO_SOLUTION;
    }
};

int main()
{
    // Read in the three mazes from the file.
    std::ifstream in_file("advent_p20.txt");
    Maze test1(in_file);
    Maze test2(in_file);
    Maze test3(in_file);
    Maze part1(in_file);

    // Unit tests for the first two mazes, then solve the main input.
    assert (test1.solve(0) == 23);
    assert (test2.solve(0) == 58);
    std::cout << "Part-1 BIST finished." << std::endl;
    std::cout << "Part-1 maze solution in " << part1.solve(0) << " steps." << std::endl;

    // Unit tests for each recursive maze, then solve the main input.
    assert (test1.solve(1) == 26);
    assert (test2.solve(1) == Maze::NO_SOLUTION);
    assert (test3.solve(1) == 396);
    std::cout << "Part-2 BIST finished." << std::endl;
    std::cout << "Part-2 maze solution in " << part1.solve(1) << " steps." << std::endl;

    return 0;
}

