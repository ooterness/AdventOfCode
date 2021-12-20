// General-purpose Intcode interpreter
// Copyright 2020 by Alex Utter
// (Used in Day 2, 5, 7, 9, ...)

#include <cassert>
#include <cinttypes>
#include <cstring>
#include <fstream>
#include <iostream>
#include <sstream>
#include <vector>

// Parse command(s) from a given text source.
void write_command(std::ostream& strm, const char* str)
{
    while (*str) {
        unsigned next = (unsigned char)(*str++);
        strm << next << ",";
    }
    strm << 10 << ",";  // End-of-input token
}

// Read next item from a comma-delited stream.
bool read_next(std::istream& strm, int64_t& val)
{
    // Consume any preceeding comma(s).
    while (strm && strm.peek() == ',') strm.get();

    // Read the next item.
    strm >> val;
    return (strm ? 1 : 0);
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

// The main Intcode interpreter object.
class Program
{
public:
    // Internal constants
    static const unsigned MAX_ALLOC = (1<<20);
    static const unsigned MODE_INDEX = 0;   // Normal indexing mode
    static const unsigned MODE_VALUE = 1;   // Immediate mode
    static const unsigned MODE_RBASE = 2;   // Relative base mode

    // Runtime mode flags
    static const unsigned RUNMODE_LOADFILE      = (1U << 0);
    static const unsigned RUNMODE_INTERACTIVE   = (1U << 1);
    static const unsigned RUNMODE_NETWORK       = (1U << 2);

    // Create program from a string.
    explicit Program(const char* str, unsigned runmode=0)
        : m_exec(0)
        , m_runmode(runmode)
        , m_base(0)
        , m_status(STATUS_CONTINUE)
    {
        if (runmode & RUNMODE_LOADFILE) {
            std::ifstream strm(str);
            read_program(strm);
        } else {
            std::istringstream strm(str);
            read_program(strm);
        }
    }

    // Create program from an input stream.
    explicit Program(std::istream& strm)
        : m_exec(0)
        , m_runmode(0)
        , m_base(0)
        , m_status(STATUS_CONTINUE)
    {
        read_program(strm);
    }

    // Copy another program.
    explicit Program(const Program& other)
        : m_prog(other.m_prog.begin(), other.m_prog.end())
        , m_exec(other.m_exec)
        , m_runmode(other.m_runmode)
        , m_base(other.m_base)
        , m_status(other.m_status)
    {
        // Nothing else to initialize.
    }

    // Define return codes for run_next.
    enum Status {
        STATUS_CONTINUE = 0,    // Ready to continue
        STATUS_HALT,            // End of program (opcode 99)
        STATUS_BLOCK,           // Waiting for new input
        STATUS_ERROR,           // Error, unable to continue
    };

    // Run exactly one instruction, return true on output.
    bool run_instr(std::istream* istrm, int64_t& outval, bool verbose=0)
    {
        // Sanity check: Are we ready to continue?
        if (m_status != STATUS_CONTINUE) {
            std::cerr << "Invalid program state." << std::endl;
            return false;
        }

        // Sanity check: did we reach end of program without halt?
        if (m_exec >= m_prog.size()) {
            std::cerr << "Missing halt instruction." << std::endl;
            m_status = STATUS_ERROR;
            return false;
        }

        // Read the next opcode and determine mode parameters.
        int64_t opcode = m_prog[m_exec];
        unsigned cmd = (unsigned)((opcode / 1) % 100);
        unsigned md1 = (unsigned)((opcode / 100) % 10);
        unsigned md2 = (unsigned)((opcode / 1000) % 10);
        unsigned md3 = (unsigned)((opcode / 10000) % 10);

        if (verbose)
            std::cout << "@" << m_exec << ", Opcode = " << opcode << std::endl;

        // Execute the operation and read parameters.
        unsigned oidx;
        int64_t in1, in2;
        if (cmd == 1) {         // Add
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2) &&
                get_index(m_exec+3, md3, oidx)) {
                m_prog[oidx]= in1 + in2;
                m_exec += 4;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 2) {  // Multiply
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2) &&
                get_index(m_exec+3, md3, oidx)) {
                m_prog[oidx]= in1 * in2;
                m_exec += 4;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 3) {  // Read input
            if (istrm && get_index(m_exec+1, md1, oidx)) {
                if (read_next(*istrm, m_prog[oidx])) {
                    if (verbose)
                        std::cout << "Read input = " << m_prog[oidx] << std::endl;
                    m_exec += 2;
                } else if (m_runmode & RUNMODE_NETWORK) {
                    if (verbose)
                        std::cout << "Read input = EMPTY (-1)" << std::endl;
                    m_prog[oidx] = -1;
                    m_exec += 2;
                } else if (m_runmode & RUNMODE_INTERACTIVE) {
                    m_status = STATUS_BLOCK;
                } else {
                    std::cerr << "Missing input." << std::endl;
                    m_status = STATUS_ERROR;
                }
            } else m_status = STATUS_ERROR;
        } else if (cmd == 4) {  // Write output
            if (get_value(m_exec+1, md1, in1)) {
                outval = in1;
                m_exec += 2;
                return true;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 5) {  // Jump if true
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2)) {
                if (in1)
                    m_exec = in2;
                else
                    m_exec += 3;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 6) {  // Jump if false
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2)) {
                if (!in1)
                    m_exec = in2;
                else
                    m_exec += 3;
            } else m_status = STATUS_ERROR;;
        } else if (cmd == 7) {  // Less than
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2) &&
                get_index(m_exec+3, md3, oidx)) {
                m_prog[oidx] = (in1 < in2) ? 1 : 0;
                m_exec += 4;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 8) {  // Equals
            if (get_value(m_exec+1, md1, in1) &&
                get_value(m_exec+2, md2, in2) &&
                get_index(m_exec+3, md3, oidx)) {
                m_prog[oidx] = (in1 == in2) ? 1 : 0;
                m_exec += 4;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 9) {  // Adjust base ptr
            if (get_value(m_exec+1, md1, in1)) {
                m_base += in1;
                m_exec += 2;
            } else m_status = STATUS_ERROR;
        } else if (cmd == 99) { // Halt
            m_status = STATUS_HALT;
        } else {                // Unknown instruction
            std::cerr << "Unknown instruction @" << m_exec << std::endl;
            m_status = STATUS_ERROR;
        }
        return false;   // No output
    }

    // Run program until it outputs or halts.
    // Returns true on successful termination, false on error.
    Status run_next(std::istream* istrm, int64_t& outval, bool verbose=0)
    {
        while (m_status == STATUS_CONTINUE) {
            if (run_instr(istrm, outval, verbose)) break;
        }
        return m_status;
    }

    // Run the program until halted, with optional input/output streams.
    // Returns true on successful termination, false on error.
    bool run(std::istream* istrm = 0, std::ostream* ostrm = 0, bool verbose=0)
    {
        // If we're in interactive mode, clear the BLOCK status.
        // Otherwise, reset execution from beginning.
        // Clear the "block" status if we're in interactive mode.
        if (m_status == STATUS_BLOCK) {
            if (m_runmode & RUNMODE_INTERACTIVE)
                m_status = STATUS_CONTINUE;
        } else {
            m_exec = 0;
            m_base = 0;
            m_status = STATUS_CONTINUE;
        }

        // Keep running program until error or halt instruction.
        while (m_status == STATUS_CONTINUE) {
            int64_t outval = 0;
            if (run_next(istrm, outval, verbose) == STATUS_CONTINUE) {
                if (ostrm) *ostrm << outval << ",";
                if (verbose) std::cout << "Wrote output = " << outval << std::endl;
            }
        }

        return (m_status != STATUS_ERROR);
    }

    // Wrapper for run(...) with a single argument and return value.
    int64_t run_simple(int64_t input, bool verbose=0)
    {
        // Construct streams and run the program.
        std::stringstream strm_in, strm_out;
        strm_in << input;
        bool ok = run(&strm_in, &strm_out, verbose);

        // Attempt to read the output.
        int64_t result = -1;
        strm_out >> result;
        if (ok && strm_out)
            return result;
        else
            return -1;
    }

    // Print current program state.
    void print()
    {
        for (unsigned a = 0 ; a < m_prog.size() ; ++a)
            std::cout << m_prog[a] << ", ";
        std::cout << std::endl;
    }

    // Main program instruction/state array.
    std::vector<int64_t> m_prog;

    // Interpreter state:
    unsigned m_exec;    // Execution pointer
    unsigned m_runmode; // Execution mode
    int64_t m_base;     // Relative base pointer
    Status m_status;    // Execution status

private:
    // Read items comma-delimited string until end of stream.
    void read_program(std::istream& strm)
    {
        int64_t tmp;
        while (read_next(strm, tmp)) {
            m_prog.push_back(tmp);
        }
    }

    // Get input parameter value, dereferencing if appropriate.
    // Returns true if successful, false otherwise.
    bool get_value(unsigned param_idx, unsigned mode, int64_t& out)
    {
        // Sanity check the parameter location.
        if (param_idx < 0 || param_idx >= m_prog.size()) {
            std::cerr << "Invalid parameter index @" << param_idx << std::endl;
            return false;
        }

        // If we're in immediate-value mode, return the specified value.
        if (mode == MODE_VALUE) {
            out = m_prog[param_idx];
            return true;
        }

        // Otherwise, figure out the index we should be reading...
        int64_t rd_idx = -1;
        if (mode == MODE_INDEX) {
            rd_idx = m_prog[param_idx];
        } else if (mode == MODE_RBASE) {
            rd_idx = m_base + m_prog[param_idx];
        } else {
            std::cerr << "Unknown parameter mode @" << param_idx << std::endl;
            return false;
        }

        // Check if that index is valid before returning the referenced value.
        if (0 <= rd_idx && rd_idx < (int64_t)m_prog.size()) {
            out = m_prog[rd_idx];
            return true;    // Normal read
        } else if (rd_idx < MAX_ALLOC) {
            out = 0;
            return true;    // Past end of allocated memory, still valid
        } else {
            std::cerr << "Invalid read index @" << param_idx << std::endl;
            return false;   // Error
        }
    }

    // Get index parameter value, error if immediate.
    bool get_index(unsigned param_idx, unsigned mode, unsigned& out)
    {
        // Sanity check the parameter location and mode.
        if (mode == MODE_VALUE || param_idx < 0 || param_idx >= m_prog.size()) {
            // Invalid mode or read index.
            std::cerr << "Invalid parameter index @" << param_idx << std::endl;
            return false;
        }

        // Determine the indexing mode.
        int64_t wr_idx = -1;
        if (mode == MODE_INDEX) {
            wr_idx = m_prog[param_idx];
        } else if (mode == MODE_RBASE) {
            wr_idx = m_base + m_prog[param_idx];
        } else {
            std::cerr << "Unknown parameter mode @" << param_idx << std::endl;
            return false;
        }

        // Check if that index is valid before returning the pointer.
        if (wr_idx < 0 || wr_idx >= MAX_ALLOC) {
            // Reference value is not a valid index.
            std::cerr << "Invalid write index @" << param_idx << std::endl;
            return false;
        } else {
            // Index is valid, allocate additional memory as needed.
            out = (unsigned)wr_idx;
            if (out >= m_prog.size()) m_prog.resize(out+1, 0);
            return true;
        }
    }
};

