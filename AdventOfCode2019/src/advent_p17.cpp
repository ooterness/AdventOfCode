// Advent of Code 2019, Day 17
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/17

#include <cassert>
#include <sstream>
#include "intcode.h"

struct AsciiMap {
    const std::string map;
    unsigned w, h;

    explicit AsciiMap(const std::string& str)
        : map(str)
    {
        w = map.find_first_of('\n');
        h = map.length() / (w+1);
    }

    // Get character at a certain R,C coordinate.
    char get(unsigned r, unsigned c) const
    {
        if (r < h && c < w)
            return map[r*(w+1) + c];
        else
            return '.';
    }

    // Is the given character a girder?
    // (Robot markers also count, since it must be on top of a girder.)
    bool has_girder(unsigned r, unsigned c) const
    {
        char x = get(r,c);
        return (x == '#') || (x == '^') || (x == '>') || (x == 'v') || (x == '<');
    }

};

// Run program to extract the ASCII map, returned as a string.
AsciiMap get_map(const Program& prog, bool print=0)
{
    std::string result; int64_t tmp;
    Program copy(prog);
    while (copy.run_next(0, tmp) == Program::STATUS_CONTINUE)
        result += (char)tmp;

    if (print) std::cout << result << std::endl;
    return AsciiMap(result);
}

// Run the specified program and compressed move sequence, return output value.
unsigned get_dust(Program& prog, const std::string& moves, bool print=0)
{
    // Write the ASCII input stream.
    std::stringstream in_strm;
    for (unsigned a = 0 ; a < moves.length() ; ++a)
        in_strm << (int)moves[a] << ",";

    // Turn down the video-feed option.
    in_strm << (int)'n' << "," << (int)'\n';

    // Run program and return the first output > 255.
    int64_t dust = 0;
    while (prog.m_status == Program::STATUS_CONTINUE && dust < 256) {
        prog.run_next(&in_strm, dust);
        if (print && dust < 256) std::cout << (char)dust;
    }

    return (unsigned)dust;
}

// Given a map, calculate the part-1 solution.
// The "alignment score" is a function derived from intersection coordinates.
unsigned get_alignment(const AsciiMap& map)
{
    // Find all intersections:
    unsigned result = 0;
    for (unsigned r = 1 ; r < map.h-1 ; ++r) {
        for (unsigned c = 1 ; c < map.w-1 ; ++c) {
            if (map.has_girder(r,c) &&
                map.has_girder(r-1,c) &&
                map.has_girder(r+1,c) &&
                map.has_girder(r,c-1) &&
                map.has_girder(r,c+1)) {
                result += r * c;
            }
        }
    }
    return result;
}

// Given a map, find the turn sequence to explore the whole path.
std::string plan_movement(const AsciiMap& map)
{
    std::stringstream moves;

    // First, find the robot's starting position.
    unsigned start_idx = map.map.find_first_of("^>v<");
    unsigned cc = start_idx % (map.w + 1);
    unsigned rr = start_idx / (map.w + 1);

    // First move is always a right turn to face east.
    moves << "R";
    unsigned dir = 1;   // 0/1/2/3 = N/E/S/W

    // Drive until we're at the end of the path.
    while (1) {
        // Turn robot direction into delta-coordinates.
        int dx = 0, dy = 0;
        if (dir == 0) dy = -1;
        if (dir == 1) dx = +1;
        if (dir == 2) dy = +1;
        if (dir == 3) dx = -1;
        // How far can we go to the next corner?
        unsigned d = 1;
        while (map.has_girder(rr+d*dy, cc+d*dx))
            ++d;
        moves << "," << --d;
        // Sanity-check the updated coordinates.
        rr += d * dy;
        cc += d * dx;
        assert ((cc < map.w) && (rr < map.h));
        // Turn left or right?
        if (map.has_girder(rr-dx, cc+dy)) {
            dir = (dir + 3) % 4;
            moves << ",L";
        } else if (map.has_girder(rr+dx, cc-dy)) {
            dir = (dir + 1) % 4;
            moves << ",R";
        } else {
            moves << ",";       // Final comma, if desired
            return moves.str(); // End of path
        }
    }
}

// Can the raw string be constructed by repeating the three subroutines?
std::string compress_helper(
    const std::string& raw,
    const std::string& sub1,
    const std::string& sub2,
    const std::string& sub3)
{
    // Greedy matching, one string at a time.
    std::stringstream result;
    unsigned m1, pos = 0;
    while (pos < raw.length()) {
        m1 = raw.find(sub1, pos);
        if (m1 == pos) {
            result << "A,";
            pos = m1 + sub1.length();
            continue;
        }

        m1 = raw.find(sub2, pos);
        if (m1 == pos) {
            result << "B,";
            pos = m1 + sub2.length();
            continue;
        }

        m1 = raw.find(sub3, pos);
        if (m1 == pos) {
            result << "C,";
            pos = m1 + sub3.length();
            continue;
        }

        // No match found, abort.
        return "";
    }

    return result.str();
}

// Given a movement sequence, split it into subroutines
// so that it fits within the designated size constraints.
std::string compress_movement(const std::string& raw)
{
    // Raw lengths include comma, which is stripped, so max is 21.
    static const unsigned SUB_MAX = 21;
    // Brute-force search across five indices, each larger than the previous:
    //   0 = Start of 1st subroutine
    //   a = End of 1st subroutine (must be a comma)
    //   b = Start of 2nd subroutine (must follow a comma)
    //   c = End of 2nd subroutine (must be a comma)
    //   d = Start of 3rd subroutine (must follow a comma)
    //   e = End of 3rd subroutine (must be a comma)
    for (unsigned a = 1 ; a <= SUB_MAX && a < raw.length() ; ++a) {
        if (raw[a] != ',') continue;
        for (unsigned b = a+1 ; b < raw.length() ; ++b) {
            if (raw[b-1] != ',') continue;
            for (unsigned c = b+1 ; c - b <= SUB_MAX && c < raw.length() ; ++c) {
                if (raw[c] != ',') continue;
                for (unsigned d = c+1 ; d < raw.length() ; ++d) {
                    if (raw[d-1] != ',') continue;
                    for (unsigned e = d+1 ; e - d <= SUB_MAX && e < raw.length() ; ++e) {
                        if (raw[e] != ',') continue;
                        // Can we construct the sequence using the three subroutines?
                        std::string s1(raw.begin()+0, raw.begin()+a+1);
                        std::string s2(raw.begin()+b, raw.begin()+c+1);
                        std::string s3(raw.begin()+d, raw.begin()+e+1);
                        std::string abc = compress_helper(raw, s1, s2, s3);
                        if (abc.length() > 0 && abc.length() <= SUB_MAX) {
                            // Match found, construct the program.
                            // (Replace final comma in each substring with newline.)
                            s1[s1.length()-1] = '\n';
                            s2[s2.length()-1] = '\n';
                            s3[s3.length()-1] = '\n';
                            abc[abc.length()-1] = '\n';
                            return abc + s1 + s2 + s3;
                        }
                    }
                }
            }
        }
    }
    return "FAILED";
}

int main()
{
    // Unit test using the example map:
    AsciiMap test_map(
        "..#..........\n"\
        "..#..........\n"\
        "#######...###\n"\
        "#.#...#...#.#\n"\
        "#############\n"\
        "..#...#...#..\n"\
        "..#####...^..\n");
    assert(get_alignment(test_map) == 76);

    // Unit test of the compressor using the example moves:
    std::string test_seq("R,8,R,8,R,4,R,4,R,8,L,6,L,2,R,4,R,4,R,8,R,8,R,8,L,6,L,2,");
    std::string sub1("R,8,R,8,"), sub2("R,4,R,4,R,8,"), sub3("L,6,L,2,");
    assert(compress_helper(test_seq, sub1, sub2, sub3) == "A,B,C,B,A,C,");
    assert(compress_movement(test_seq) != "FAILED");

    // Generate the Part-1 map and calculate alignment.
    Program prog("advent_p17.txt", 1);
    AsciiMap part1_map = get_map(prog, 1);
    std::cout << "Alignment parameter = " << get_alignment(part1_map) << std::endl;

    // Find the sequence to navigate the maze.
    std::string moves1 = plan_movement(part1_map);
    std::cout << "Raw path:" << std::endl << moves1 << std::endl;

    // Break that sequence into smaller subroutines.
    std::string moves2 = compress_movement(moves1);
    std::cout << "Compressed path:" << std::endl << moves2 << std::endl;

    // Wake the robot and execute the compressed path.
    if (moves2 != "FAILED") {
        prog.m_prog[0] = 2;
        std::cout << "Dust count = " << get_dust(prog, moves2) << std::endl;
    }

    return 0;
}

