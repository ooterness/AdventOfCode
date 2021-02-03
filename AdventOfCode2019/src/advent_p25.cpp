// Advent of Code 2019, Day 25
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/25

#include <cstring>
#include <iostream>
#include <list>
#include <string>
#include "intcode.h"

// Print contents of an Intcode-ASCII stream.
// (e.g., "72,101,108,108,111" --> "Hello")
void print_ascii(std::stringstream& strm)
{
    // Read and print each character.
    int64_t next;
    while (read_next(strm, next)) {
        std::cout << (char)next;
    }

    // Clear EOF flag for next iteration.
    strm.clear();
}

// Parse command(s) from a given text source.
void write_command(std::ostream& strm, const char* str)
{
    while (*str) {
        unsigned next = (unsigned char)(*str++);
        strm << next << ",";
    }
    strm << 10 << ",";  // End-of-input token
}

// Read one line of user input and provide it as an Intcode-ASCII stream.
// (e.g., "Hello" --> "72,101,108,108,111,10")
// Return true if user enters "quit".
unsigned read_ascii(std::stringstream& strm)
{
    // Clear EOF flags on the input stream.
    strm.clear();

    // Read one line of user data.
    static const unsigned MAX_LINE = 256;
    char line[MAX_LINE];
    std::cin.getline(line, MAX_LINE);

    // Check for special commands...
    unsigned len = strlen(line);
    if (len >= 4 && line[0] == 'q' && line[1] == 'u' && line[2] == 'i' && line[3] == 't')
        return 1;   // Quit command
    if (len >= 4 && line[0] == 'e' && line[1] == 'x' && line[2] == 'i' && line[3] == 't')
        return 1;   // Quit command
    if (len >= 4 && line[0] == 's' && line[1] == 'a' && line[2] == 'v' && line[3] == 'e')
        return 2;   // Save-state
    if (len >= 4 && line[0] == 'l' && line[1] == 'o' && line[2] == 'a' && line[3] == 'd')
        return 3;   // Load-state
    if (len >= 5 && line[0] == 's' && line[1] == 't' && line[2] == 'a' && line[3] == 'r' && line[4] == 't')
        return 4;   // Auto-start
    if (len >= 5 && line[0] == 'b' && line[1] == 'r' && line[2] == 'u' && line[3] == 't' && line[4] == 'e')
        return 5;   // Brute-force search

    // Otherwise, write comma-delimited contents to the stream.
    write_command(strm, line);
    return 0;           // Continue / normal input
}

// Write commands required to pick up every safe item.
void quick_start(std::stringstream& strm)
{
    write_command(strm, "south");    // To the kitchen
    write_command(strm, "take monolith");
    write_command(strm, "east");
    write_command(strm, "take asterisk");
    write_command(strm, "west");
    write_command(strm, "north");    // Back at start
    write_command(strm, "west");
    write_command(strm, "take coin");
    write_command(strm, "north");
    write_command(strm, "east");
    write_command(strm, "take astronaut ice cream");
    write_command(strm, "west");
    write_command(strm, "south");
    write_command(strm, "east");     // Back at start
    write_command(strm, "north");
    write_command(strm, "north");
    write_command(strm, "take mutex");
    write_command(strm, "west");
    write_command(strm, "take astrolabe");
    write_command(strm, "west");
    write_command(strm, "take dehydrated water");
    write_command(strm, "west");
    write_command(strm, "take wreath");
    write_command(strm, "east");
    write_command(strm, "south");
    write_command(strm, "east");
    write_command(strm, "north");    // Stop at the security checkpoint
}

// Helper function looks for "lighter" or "heavier" in the given output stream.
bool find_words(const std::stringstream& ref)
{
    // Make a copy of the comma-delimited input stream, then convert.
    int64_t next;
    std::stringstream in, out;
    in << ref.rdbuf();
    while (read_next(in, next))
        out << (char)next;

    // Search for the two magic words.
    std::string str(out.str());
    if (str.find("lighter") < str.length()) return true;
    if (str.find("heavier") < str.length()) return true;
    return false;
}

// Brute-force search for the required item set.
bool quick_search(Program& prog, bool verbose=0)
{
    // Try each combination of the eight items.
    for (unsigned a = 0 ; a < 256 ; ++a) {
        // Make a copy of the initial state...
        Program guess(prog);
        // From that state, drop items according to current guess.
        // (Note: Bit assignments are arbitrary order; don't care about weights.)
        std::stringstream strm_in, strm_out;
        std::cout << "Trying combo: " << a << std::endl; //???
        if (verbose) write_command(strm_in, "inv");
        if (a & 1)   write_command(strm_in, "drop asterisk");
        if (a & 2)   write_command(strm_in, "drop astrolabe");
        if (a & 4)   write_command(strm_in, "drop astronaut ice cream");
        if (a & 8)   write_command(strm_in, "drop coin");
        if (a & 16)  write_command(strm_in, "drop dehydrated water");
        if (a & 32)  write_command(strm_in, "drop monolith");
        if (a & 64)  write_command(strm_in, "drop mutex");
        if (a & 128) write_command(strm_in, "drop wreath");
        // Walk north to trigger the weight check.
        write_command(strm_in, "north");
        // Run program until next input...
        guess.run(&strm_in, &strm_out);
        //print_ascii(strm_out); //???
        if (find_words(strm_out)) {
            // If we see "lighter" or "heavier" in the output stream, try again.
            if (verbose) {
                print_ascii(strm_out);
                std::cout << "Mask failed: " << a << std::endl;
            }
        } else {
            // Otherwise, print final output and make that the new state.
            print_ascii(strm_out);
            if (verbose) {
                std::cout << "Mask passed: " << a << std::endl;
            }
            prog = guess;
            return true;
        }
    }
    std::cout << "No solution :(" << std::endl;
    return false;
}

int main()
{
    // Part-A: Run program interactively.
    static const unsigned RUNMODE =
        Program::RUNMODE_LOADFILE |
        Program::RUNMODE_INTERACTIVE;
    Program prog("advent_p25.txt", RUNMODE);
    std::list<Program> saves;
    saves.push_back(prog);  // Stack of saved states
    std::stringstream strm_in, strm_out;
    while (1) {
        // Run program until it needs more input.
        prog.run(&strm_in, &strm_out);
        // Print results, and check if we should prompt user.
        print_ascii(strm_out);
        if (prog.m_status == Program::STATUS_BLOCK) {
            switch (read_ascii(strm_in)) {
            case 1: // Quit command
                return 0;
            case 2: // Save command
                saves.push_back(prog);
                std::cout << "Saved!" << std::endl;
                break;
            case 3: // Load command
                prog = *saves.rbegin();
                if (saves.size() > 1) saves.pop_back();
                std::cout << "Loaded!" << std::endl;
                break;
            case 4: // Auto-start (pick up all safe items)
                quick_start(strm_in);
                break;
            case 5: // Auto-search (brute-force)
                saves.push_back(prog);
                quick_search(prog);
                break;
            }
        } else {
            std::cout << "[Program terminated]" << std::endl;
            break;
        }
    }
    return -1;
}

