// Advent of Code 2019, Day 23
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/23

#include <sstream>
#include "intcode.h"

#define DEBUG_VERBOSE 1

// Define the network as a vector of Nodes.
class Node;
typedef std::vector<Node*> Network;

// A single network-attached IntCode computer.
class Node {
public:
    explicit Node(const Program& ref, unsigned _addr)
        : addr(_addr)
        , prog(ref)
        , pkt_count(0)
        , sent_flag(0)
    {
        in << _addr << " ";
    }
    
    bool is_halted() const
    {
        return (prog.m_status == Program::STATUS_HALT)
            || (prog.m_status == Program::STATUS_ERROR);
    }

    bool is_idle() const
    {
        return !in;
    }

    bool just_sent() const
    {
        return sent_flag;
    }

    void recv_pkt(int64_t x, int64_t y)
    {
        in.clear(); // Clear flags from previous EOF
        in << x << " " << y << " ";
    }

    bool send_pkt(Network& net)
    {
        // Sanity check that we have a packet to send...
        assert (pkt_count == 3);
        pkt_count = 0;
        sent_flag = 1;

        // Should we print the packet contents?
        unsigned dst = (unsigned)pkt_buff[0];
        if (DEBUG_VERBOSE || addr > net.size()) {
            std::cout << "Packet from [" << addr << "] to [" << dst
                      << "] = " << pkt_buff[1] << ", " << pkt_buff[2] << std::endl;
        }

        // Return true if packet goes outside the network.
        if (dst < net.size()) {
            net[dst]->recv_pkt(pkt_buff[1], pkt_buff[2]);
            return false;
        } else {
            return true;
        }
    }

    bool step(Network& net)
    {
        // Execute the next instruction...
        sent_flag = 0;
        int64_t tmp = 0;
        if (prog.run_instr(&in, tmp)) {
            // Store any output.  Every 3rd item, send packet.
            pkt_buff[pkt_count] = tmp;
            if (++pkt_count == 3)
                return send_pkt(net);
        }
        return false;
    }

    unsigned addr;
    Program prog;
    std::stringstream in;
    unsigned pkt_count;
    int64_t pkt_buff[3];
    bool sent_flag;
};

// Run simulated network until the first output packet.
void run_network(const char* lbl, const Program& ref, unsigned size, bool nat=0)
{
    std::cout << "*********************************" << std::endl;
    std::cout << "Running network: " << lbl << std::endl;

    // Create the network.
    Network net;
    for (unsigned a = 0 ; a < size ; ++a)
        net.push_back(new Node(ref, a));

    // Run each node in lockstep until we get an output
    // or every program has halted.
    bool done = false;
    unsigned idle_cycles = 0, consec_nat = 0;
    int64_t nat_x = 0, nat_y = 0;
    while (!done) {
        // Run the next instruction for each node.
        unsigned nhalt = 0, nsent = 0;
        for (auto it = net.begin() ; it != net.end() ; ++it) {
            Node* n = *it;
            if (n->is_halted()) {
                // If node is halted, don't run next instruction.
                ++nhalt;
            } else if(n->step(net)) {
                if (nat) {
                    // Update the NAT's last received packet.
                    nat_x = n->pkt_buff[1];
                    nat_y = n->pkt_buff[2];
                } else {
                    // No NAT (e.g., Part 1) -> halt after this iteration.
                    done = true;
                }
            }
            if (n->just_sent()) ++nsent;
        }
        // If every program has halted, stop immediately.
        if (nhalt == size) break;
        // Count the number of idle nodes.
        unsigned nidle = 0;
        for (auto it = net.begin() ; it != net.end() ; ++it) {
            Node* n = *it;
            if (n->is_idle()) ++nidle;
        }
        // Count consecutive cycles where the whole network is idle...
        if ((nsent == 0) && (nidle == size)) {
            ++idle_cycles;
        } else {
            idle_cycles = 0;
        }
        // If NAT is enabled, send packet whenever network is idle.
        if (nat && (nidle == size)) {
            // Send packet (and display contents).
            std::cout << "NAT activated: " << nat_x << ", " << nat_y << std::endl;
            net[0]->recv_pkt(nat_x, nat_y);
            // On the Nth consecutive NAT with no other activity, halt.
            if (++consec_nat >= 10) break;
        } else if (nsent) {
            consec_nat = 0; // Non-NAT packet, reset stop condition
        }
    }

    // Cleanup.
    for (auto it = net.begin() ; it != net.end() ; ++it)
        delete *it;
}

int main()
{
    // Unit test from Reddit thread:
    // https://www.reddit.com/r/adventofcode/comments/eel8y3/2019_day_23_intcode_program_to_test_part_one/
    Program test1(
        "3,60,1005,60,18,1101,0,1,61,4,61,104,1011,104,1,1105,1,22,1101,"\
        "0,0,61,3,62,1007,62,0,64,1005,64,22,3,63,1002,63,2,63,1007,63,256,"\
        "65,1005,65,48,1101,0,255,61,4,61,4,62,4,63,1105,1,22,99", Program::RUNMODE_NETWORK);
    run_network("TEST1", test1, 2);

    // Part-1 solution runs until first output packet.
    Program ref("advent_p23.txt", Program::RUNMODE_LOADFILE | Program::RUNMODE_NETWORK);
    run_network("PART1", ref, 50);

    // Part-2 solution runs until NAT sends twice in a row.
    run_network("PART2", ref, 50, 1);

    return 0;
}

