// Advent of Code 2019, Day 13
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/13

#include <conio.h>
#include <deque>
#include <sstream>
#include <vector>
#include "intcode.h"

struct Board {
    // Internal state
    std::vector<std::string> board;
    unsigned max_x, max_y, blocks, score, paddle_x, ball_x;

    explicit Board()
        : max_x(0), max_y(0), blocks(0), score(0), paddle_x(0), ball_x(0)
    {}

    explicit Board(const Board& other)
        : board(other.board)
        , max_x(other.max_x)
        , max_y(other.max_y)
        , blocks(other.blocks)
        , score(other.score)
        , paddle_x(other.paddle_x)
        , ball_x(other.ball_x)
    {
    }

    // Render next frame of game board, including score and block count.
    void render(std::stringstream& strm)
    {
        // Define tokens for each tile (empy, wall, block, paddle, ball)
        static const char TILES[] = ".#X=o";

        // Convert string to X/Y/T integer list.
        // As we do so, count blocks and note max X/Y coordinates.
        int64_t tmp1, tmp2, tmp3;
        std::deque<unsigned> x, y, v;
        while (read_next(strm, tmp1) &&
               read_next(strm, tmp2) &&
               read_next(strm, tmp3)) {
            if (tmp1 < 0) {
                // Special case for score.
                score = tmp3;
            } else {
                // Normal X/Y/Type command.
                x.push_back(tmp1);
                y.push_back(tmp2);
                v.push_back(tmp3);
                if (tmp1 > max_x) max_x = tmp1;
                if (tmp2 > max_y) max_y = tmp2;
            }
        }

        // Do we need to resize the board?
        if (board.empty() || max_y >= board.size() || max_x >= board[0].length()) {
            board.clear();
            board.resize(max_y+1, std::string(max_x+1, TILES[0]));
        }

        // Apply each tile-update command.
        for (unsigned a = 0 ; a < x.size() ; ++a) {
            // Sanity-check parameters.
            unsigned r = y[a]; assert(r <= max_y);
            unsigned c = x[a]; assert(c <= max_x);
            unsigned t = v[a]; assert(t <= 4);
            // Update the remaining-blocks counter, ball position, etc.
            if (t == 2) ++blocks;
            if (t == 3) paddle_x = c;
            if (t == 4) ball_x = c;
            if ((board[r][c] == 2) && (t != 2)) --blocks;
            // Overwrite the character for this tile.
            board[r][c] = TILES[t];
        }

        // Finally, print the result.
        for (unsigned r = 0 ; r <= max_y ; ++r)
            std::cout << board[r] << std::endl;
        std::cout << "BLOCKS = " << blocks << std::endl;
        std::cout << "SCORE  = " << score << std::endl;
    }
};

int main()
{
    // Read program and run to completion.
    std::stringstream strm1;
    Program part1("advent_p13.txt", Program::RUNMODE_LOADFILE);
    part1.run(0, &strm1);
    Board board1; board1.render(strm1);

    // Run program interactively.
    static const unsigned RUNMODE =
        Program::RUNMODE_LOADFILE |
        Program::RUNMODE_INTERACTIVE;
    Program part2("advent_p13.txt", RUNMODE);
    Board board2;
    Program save_prog(part2);
    Board save_scrn(board2);
    part2.m_prog[0] = 2;    // Quarters hacked!
    while (1) {
        std::stringstream strm2_in, strm2_out;
        // Get user input.
        int next = _getch();
        if (next == 'q' || next == 'Q') {
            break;              // Quit game
        } else if (next == 's' ||  next == 'S') {
            std::cout << "State SAVED!" << std::endl;
            save_prog = part2;      // Save state
            save_scrn = board2;
            continue;
        } else if (next == 'r' ||  next == 'R') {
            std::cout << "State RESTORED!" << std::endl;
            part2 = save_prog;  // Restore state
            board2 = save_scrn;
            continue;
        } else if (next == 'a' || next == 'A') {
            // Autopilot mode
            if (board2.ball_x < board2.paddle_x)
                strm2_in << "-1,";  // Left
            else
                strm2_in << "1,";   // Right
        } else if (next == '4') {
            strm2_in << "-1,";  // Left
        } else if (next == '6') {
            strm2_in << "1,";   // Right
        } else {
            strm2_in << "0,";   // Neutral
        }
        // Render the next frame.
        part2.run(&strm2_in, &strm2_out);
        board2.render(strm2_out);
        // On halt, restore state instead.
        if (part2.m_status != Program::STATUS_BLOCK) {
            std::cout << "YOU ARE DEAD!" << std::endl;
            part2 = save_prog;
            board2 = save_scrn;
        }
    }

    return 0;
}

