/// Day 23: https://adventofcode.com/2021/day/23
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

// Enable diagnostics?
const VERBOSE:      bool = false;

// Shortcuts for each room state:
const RM_EMPTY:     u8 = 0;
const RM_PODA:      u8 = 1;
const RM_PODB:      u8 = 2;
const RM_PODC:      u8 = 3;
const RM_PODD:      u8 = 4;
const RM_HALL_UP:   usize = 11;
const RM_HALL_LO:   usize = 15;
const RM_SIZE:      usize = 19;

// Movement costs for each amphipod type:
fn cost(typ:u8) -> u64 {
    match typ {
        RM_PODA => 1,
        RM_PODB => 10,
        RM_PODC => 100,
        RM_PODD => 1000,
        _       => u64::MAX,
    }
}

// Convert room contents to display character.
fn rm2char(typ:u8) -> char {
    match typ {
        RM_PODA => 'A',
        RM_PODB => 'B',
        RM_PODC => 'C',
        RM_PODD => 'D',
        _       => ' ',
    }
}

// A game board is a set of 19 locations:
//  0-1-2-3-4-5-6-7-8-9-10      0-10 are the hallway
//      11  12  13  14          11-18 are the rooms
//      15  16  17  18
type Rooms = [u8;RM_SIZE];

// Expected type for each side-room.
fn expect(rm:usize) -> u8 {
    match rm {
        11 | 15 => RM_PODA,
        12 | 16 => RM_PODB,
        13 | 17 => RM_PODC,
        14 | 18 => RM_PODD,
        _       => RM_EMPTY,
    }
}

// Is a given room index a hallway?
fn is_hall(rm:usize) -> bool {
    rm < RM_HALL_UP
}


// A game-state is the room vector plus a cumulative movement cost.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct GameState {
    rm: Rooms,      // Contents of each room
    cost: u64,      // Total cost to reach this game state
}

impl GameState {
    // Set initial state from a descriptor string:
    //  #############
    //  #...........#     Concatenate each line:
    //  ###B#C#B#D### --> BCBD
    //    #A#D#C#A#   --> ADCA --> "BCBDADCA"
    //    #########
    fn new(line: &str) -> GameState {
        let mut ch = line.chars();
        let mut rm:Rooms = [RM_EMPTY;RM_SIZE];
        for n in RM_HALL_UP..RM_SIZE {
            rm[n] = match ch.next().unwrap() {
                'A' => RM_PODA,
                'B' => RM_PODB,
                'C' => RM_PODC,
                'D' => RM_PODD,
                _   => RM_EMPTY,
            }
        }
        GameState { rm:rm, cost:0 }
    }

    // Is a given room empty?
    fn empty(&self, idx:usize) -> bool {
        self.rm[idx] == RM_EMPTY
    }

    // Is a given stretch of hallway empty? (Inclusive bounds)
    fn hall_empty(&self, lo:usize, hi:usize) -> bool {
        assert!(is_hall(lo) && is_hall(hi) && lo <= hi);
        self.rm[lo..hi+1].iter().all(|&r| r == RM_EMPTY)
    }

    // Path length from room to designated spot in hallway,
    // confirming that intervening cells are free.
    fn path(&self, from:usize, to:usize) -> Option<usize> {
        let hh = std::cmp::min(from, to);
        let rr = std::cmp::max(from, to);
        assert!(is_hall(hh));
        assert!(!is_hall(rr));
        let cc:usize = match rr {
            11 | 15 => 2,   // Entrance of 1st room
            12 | 16 => 4,   // Entrance of 2nd room
            13 | 17 => 6,   // Entrance of 3rd room
            14 | 18 => 8,   // Entrance of 4th room
            _       => 0,   // Invalid
        };
        // Upper half is one step to move out; lower is two.
        let uu = if rr < RM_HALL_LO {1} else {2};
        // Check any intervening cells...
        if (hh < cc) && self.hall_empty(hh+1,cc) {
            Some(uu + cc - hh)  // Left of entrance
        } else if (hh > cc) && self.hall_empty(cc,hh-1) {
            Some(uu + hh - cc)  // Right of entrance
        } else {
            None                // Move is blocked
        }
    }

    // Attempt to make a move:
    fn pmove(&self, from:usize, to:usize) -> Option<GameState> {
        // Sanity check: Source if full and destination is empty.
        if self.empty(from) {return None;}
        if !self.empty(to) {return None;}
        // Is the origin a room or a hallway?
        if is_hall(from) {
            // Moving from hallway to a room.
            assert!(!is_hall(to));
            // Only enter a room if it's our final destination.
            if self.rm[from] != expect(to) {return None;}
            // Can't move to upper cell if lower cell would be trapped.
            if to < RM_HALL_LO && self.rm[to+4] != expect(to) {return None;}
            // Can't move to lower cell if upper cell is occupied.
            if to >= RM_HALL_LO && self.rm[to-4] != RM_EMPTY {return None;}
        } else {
            // Moving from a room to a hallway.
            assert!(is_hall(to));
            // Never move to one of the entranceways.
            if to == 2 || to == 4 || to == 6 || to == 8 {return None;}
            // Can't leave lower cell if upper cell is occupied.
            if from >= RM_HALL_LO && self.rm[from-4] != RM_EMPTY {return None;}
        }
        // Check if the required stretch of hallway is clear.
        if let Some(n) = self.path(from, to) {
            // Movement cost for this amphipod type?
            let cost = self.cost + (n as u64) * cost(self.rm[from]);
            // Clone and update the room vector.
            let mut rm = self.rm.clone();
            rm[to] = self.rm[from];
            rm[from] = RM_EMPTY;
            Some ( GameState { rm:rm, cost:cost } )
        } else {None}
    }

    // Find all legal moves from this state.
    fn next(&self) -> Vec<GameState> {
        // Scan rooms for each amphipod...
        let mut next = Vec::new();
        for (from,&r) in self.rm.iter().enumerate() {
            // Skip empty rooms.
            if r == RM_EMPTY {continue;}
            // Otherwise, try every possible destination.
            if is_hall(from) {
                for to in RM_HALL_UP..RM_SIZE {
                    if let Some(st) = self.pmove(from,to) {next.push(st);}
                }
            } else {
                for to in 0..RM_HALL_UP {
                    if let Some(st) = self.pmove(from,to) {next.push(st);}
                }
            }
        }
        next
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ch:Vec<char> = self.rm.iter().map(|r| rm2char(*r)).collect();
        write!(f, "############# Cost: {}\n#", self.cost)?;
        for n in 0..RM_HALL_UP {write!(f, "{}", ch[n])?;}
        writeln!(f, "#")?;
        writeln!(f, "###{}#{}#{}#{}###", ch[11], ch[12], ch[13], ch[14])?;
        writeln!(f, "  #{}#{}#{}#{}#",   ch[15], ch[16], ch[17], ch[18])?;
        writeln!(f, "  #########")
    }
}

impl Ord for GameState {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)  // Reversed so we get a min-heap
    }
}

impl PartialOrd for GameState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Find lowest-cost path to the winning state.
// https://en.wikipedia.org/wiki/Dijkstra%27s_algorithm#Pseudocode
fn dijkstra(start: &GameState) -> Option<u64> {
    let mut best:   HashMap<Rooms,u64> = HashMap::new();
    let mut done:   HashSet<Rooms> = HashSet::new();
    let mut queue:  BinaryHeap<GameState> = BinaryHeap::new();

    // Define the winning state.
    let win = GameState::new("ABCDABCD");

    // Insert the starting point.
    queue.push(start.clone());
    best.insert(start.rm, start.cost);

    // Keep popping from priority queue until we find the solution.
    while let Some(state) = queue.pop() {
        // Stop immediately if we've reached the end node.
        if state.rm == win.rm {return Some(state.cost);}
        // Skip nodes that we've already visited.
        if !done.insert(state.rm) {continue;}
        // Optionally print current state if verbose.
        if VERBOSE {eprint!("{}", state)}
        // Otherwise, process each of the immediate neighbors.
        for next in state.next().into_iter() {
            let old = best.entry(next.rm).or_insert(u64::MAX);
            if next.cost < *old {
                *old = next.cost;
                queue.push(next);
            }
        }
    }
    return None
}

pub fn solve() {
    let test = GameState::new("BCBDADCA");
    let data = GameState::new("ADBDBCAC");

    assert_eq!(dijkstra(&test).unwrap(), 12521);
    println!("Part1: {}", dijkstra(&data).unwrap());
}
