// Advent of Code 2019, Day 6
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/6

#include <algorithm>
#include <cassert>
#include <fstream>
#include <iostream>
#include <sstream>
#include <map>
#include <vector>

typedef std::map<std::string, std::string> orbit_map;
typedef std::vector<std::string> orbit_list;

orbit_map read_orbits(std::istream& strm)
{
    // Read each line of the input stream.
    // Use a dictionary to store object-to-parent map.
    orbit_map orbits;
    std::string line;
    while (std::getline(strm, line)) {
        // Split line on the ')' token.
        unsigned split = line.find(')');
        std::string a(line.begin(), line.begin() + split);
        std::string b(line.begin() + split + 1, line.end());
        // Add the new entry to the dictionary.
        orbits[b] = a;
    }
    return orbits;
}

orbit_list get_parents(const orbit_map& orbits, const std::string& node)
{
    orbit_list result;
    orbit_map::const_iterator next = orbits.find(node);
    while (next != orbits.end()) {
        result.push_back(next->second);
        next = orbits.find(next->second);
    }
    return result;
}

unsigned count_orbits(const orbit_map& orbits)
{
    // Count number of nested orbits for each object in the map.
    unsigned total = 0;
    for (orbit_map::const_iterator it = orbits.begin() ; it != orbits.end() ; ++it) {
        total += get_parents(orbits, it->first).size();
    }
    return total;
}

unsigned count_transfer(const orbit_map& orbits, const std::string& from, const std::string& to)
{
    // Make a list of parents for each node, going up to the root.
    orbit_list list_a = get_parents(orbits, from);
    orbit_list list_b = get_parents(orbits, to);

    // Traverse the first list until we find a common element, then find distance.
    for (orbit_list::const_iterator a = list_a.begin() ; a != list_a.end() ; ++a) {
        orbit_list::const_iterator b = find(list_b.begin(), list_b.end(), *a);
        if (b != list_b.end()) {
            unsigned da = a - list_a.begin();
            unsigned db = b - list_b.begin();
            return da + db;
        }
    }

    // If we reach this point, there is no valid path. :(
    return -1;
}

int main()
{
    // Unit test from the Part 1 instructions.
    std::istringstream test1_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n");
    orbit_map test1_map = read_orbits(test1_str);
    assert (count_orbits(test1_map) == 42);

    // Count orbits in the full-size input.
    std::ifstream ref_file("advent_p6.txt");
    orbit_map ref_orbits = read_orbits(ref_file);
    std::cout << "Orbit count = " << count_orbits(ref_orbits) << std::endl;

    // Unit test from the Part 2 instructions.
    std::istringstream test2_str("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n");
    orbit_map test2_map = read_orbits(test2_str);
    assert (count_transfer(test2_map, "YOU", "SAN") == 4);

    // Count transfer from our location to Santa.
    std::cout << "Transfer count = " << count_transfer(ref_orbits, "YOU", "SAN") << std::endl;

    return 0;
}

