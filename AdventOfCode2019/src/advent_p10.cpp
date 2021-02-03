// Advent of Code 2019, Day 10
// Copyright 2020 by Alex Utter
// https://adventofcode.com/2019/day/10

#include <cassert>
#include <iostream>
#include <fstream>
#include <map>
#include <set>
#include <vector>

// Simple struct for an X/Y pair.
struct Asteroid {
    int px, py;

    explicit Asteroid(int x, int y)
        : px(x), py(y) {}

    operator==(const Asteroid& other) const {
        return (px == other.px) && (py == other.py);
    }

};

std::ostream& operator<<(std::ostream& out, const Asteroid& val) {
    return out << "(" << val.px << "," << val.py << ")";
}

typedef std::vector<Asteroid> AsteroidList;

// Sortable object denoting a direction from origin.
struct Ray {
    int dx, dy;

    // Construct Ray from A to B.
    // (Note: Negate Y coordinate to use usual math conventions.)
    explicit Ray(const Asteroid& a, const Asteroid& b)
        : dx(b.px - a.px), dy(a.py - b.py) {}

    // Calculate square-magnitude.
    unsigned magsq() const {
        return dx*dx + dy*dy;
    }

    // Quadrant map is clockwise:
    //  44111
    //  44111
    //  44022
    //  33322
    //  33322
    unsigned quadrant() const {
        if (dx >= 0 && dy > 0) return 1;
        if (dx > 0 && dy <= 0) return 2;
        if (dx <= 0 && dy < 0) return 3;
        if (dx < 0 && dy >= 0) return 4;
        return 0;   // Origin
    }

    bool operator<(const Ray& other) const {
        // Sort by clockwise order: quadrant first, then x/y or y/x ratio.
        // (Ratio alone could be equal even on the opposite side of origin.)
        unsigned q1 = quadrant();
        unsigned q2 = other.quadrant();
        int delta = abs(dx * other.dy) - abs(other.dx * dy);
        if (q1 < q2) return true;   // Sort by quadrant first.
        if (q1 > q2) return false;
        if (q1 == 1 || q1 == 3)     // Same quadrant, so sort ratio by...
            return (delta < 0);     // ...ascending x/y (descending y/x)
        else
            return (delta > 0);     // ...ascending y/x (descending x/y)
    }

    bool operator==(const Ray& other) const {
        // Quadrant and x/y ratio must both be equal.
        // (Ratio alone could be equal even on the opposite side of origin.)
        unsigned q1 = quadrant();
        unsigned q2 = other.quadrant();
        int delta = dx * other.dy - other.dx * dy;
        return (q1 == q2 && delta == 0);
    }
};

// What's the maximum number of visible asteroids for a given map?
unsigned max_visible(const AsteroidList& list, Asteroid& center)
{
    // Put a "base" on each starting point and count visible asteroids.
    unsigned max_count = 0;
    for (AsteroidList::const_iterator base = list.begin() ; base != list.end() ; ++base) {
        // Create a list of visible asteroids from this central base.
        std::set<Ray> visible;
        for (AsteroidList::const_iterator it = list.begin() ; it != list.end() ; ++it) {
            if (it == base) continue;   // Skip self
            visible.insert(Ray(*base, *it));
        }
        // How many unique lines-of-sight do we have?
        unsigned count = visible.size();
        if (count > max_count) {
            max_count = count;
            center = *base;
        }
    }
    return max_count;
}

// Given a map and a laser-base location, what's the Nth destroyed asteroid?
Asteroid nth_destroyed(const AsteroidList& list, const Asteroid& base, unsigned nreq, bool verbose=0)
{
    // Sanity check on the requested index.
    // (Must be nreq < size because the base object is never destroyed.)
    assert(nreq < list.size());

    // Construct a list of all objects, grouped by angle.
    std::map<Ray, std::map<unsigned, Asteroid> > sweep;
    for (AsteroidList::const_iterator it = list.begin() ; it != list.end() ; ++it) {
        if (*it == base) continue;   // Skip self
        Ray tmp(base, *it);
        sweep[tmp].insert(std::make_pair(tmp.magsq(), *it));
    }

    // If verbose-mode is enabled, print sets in order:
    if (verbose) {
        for (auto it = sweep.begin() ; it != sweep.end() ; ++it) {
            for (auto a = it->second.begin() ; a != it->second.end() ; ++a)
                std::cout << a->second << ", ";
            std::cout << std::endl;
        }
    }

    // Execute each laser sweep...
    unsigned nboom = 0; // Number destroyed so far.
    while (1) {
        // For each unique angle...
        for (auto it = sweep.begin() ; it != sweep.end() ; ++it) {
            // Skip subsets that are already empty.
            if (it->second.empty()) continue;
            // Is this the Nth destroyed asteroid?
            if (++nboom == nreq) {
                return it->second.begin()->second;
            } else {
                it->second.erase(it->second.begin());
            }
        }
    }
}

// Read ASCII-art map from text input stream.
AsteroidList read_map(std::istream& in)
{
    AsteroidList result;

    // Skip any comment lines before the map itself.
    std::string(line);
    while (in.peek() == '/') getline(in, line);

    // Now read each actual line...
    // (Stop at EOF or next comment line.)
    for (int y = 0 ; in.peek() != '/' ; ++y) {
        getline(in, line);  // Attempt to read next line?
        if (!in) break;     // Reached EOF?
        for (int x = 0 ; x < (int)line.size() ; ++x) {
            if (line[x] == '#') result.push_back(Asteroid(x,y));
        }
    }

    return result;
}

// Print ASCII-art map
void print_map(const AsteroidList& list, int max_dim = 32)
{
    for (int y = 0 ; y < max_dim ; ++y) {
        for (int x = 0 ; x < max_dim ; ++x) {
            bool found = false;
            for (AsteroidList::const_iterator it = list.begin() ; !found && it != list.end() ; ++it) {
                if ((it->px == x) && (it->py == y)) found = true;
            }
            std::cout << (found ? '#' : '.');
        }
        std::cout << std::endl;
    }
}

int main()
{
    static const bool VERBOSE = false;
    Asteroid center(0,0);

    // Open the input file with all the maps...
    std::ifstream in_file("advent_p10.txt");

    // First N maps are unit tests:
    AsteroidList map1 = read_map(in_file);
    assert(max_visible(map1, center) == 8);
    assert(center == Asteroid(3,4));

    AsteroidList map2 = read_map(in_file);
    assert(max_visible(map2, center) == 33);
    assert(center == Asteroid(5,8));

    AsteroidList map3 = read_map(in_file);
    assert(max_visible(map3, center) == 35);
    assert(center == Asteroid(1,2));

    AsteroidList map4 = read_map(in_file);
    assert(max_visible(map4, center) == 41);
    assert(center == Asteroid(6,3));

    AsteroidList map5 = read_map(in_file);
    assert(max_visible(map5, center) == 210);
    assert(center == Asteroid(11,13));

    assert(nth_destroyed(map5, center, 1, VERBOSE) == Asteroid(11,12));
    assert(nth_destroyed(map5, center, 2) == Asteroid(12,1));
    assert(nth_destroyed(map5, center, 3) == Asteroid(12,2));
    assert(nth_destroyed(map5, center, 10) == Asteroid(12,8));
    assert(nth_destroyed(map5, center, 20) == Asteroid(16,0));
    assert(nth_destroyed(map5, center, 50) == Asteroid(16,9));
    assert(nth_destroyed(map5, center, 100) == Asteroid(10,16));
    assert(nth_destroyed(map5, center, 199) == Asteroid(9,6));
    assert(nth_destroyed(map5, center, 200) == Asteroid(8,2));
    assert(nth_destroyed(map5, center, 201) == Asteroid(10,9));
    assert(nth_destroyed(map5, center, 299) == Asteroid(11,1));

    // The last map is the problem input:
    AsteroidList map6 = read_map(in_file);
    std::cout << "Visible: " << max_visible(map6, center)
        << " from " << center << std::endl;
    std::cout << "200th destroyed = "
        << nth_destroyed(map6, center, 200) << std::endl;

    return 0;
}

