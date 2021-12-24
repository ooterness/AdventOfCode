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
const RM_HALL:      usize = 11;
const RM_SIZE_SM:   usize = 19;
const RM_SIZE_XL:   usize = 27;

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
//      11  12  13  14          11+ are the rooms
//      15  16  17  18          (Additional rows as needed)
type Rooms = Vec<u8>;

// Expected type for each side-room.
fn expect(rm:usize) -> u8 {
    if rm < RM_HALL {return RM_EMPTY;}
    match (rm-RM_HALL) % 4 {
        0 => RM_PODA,
        1 => RM_PODB,
        2 => RM_PODC,
        _ => RM_PODD,
    }
}

// Is a given room index a hallway?
fn is_hall(rm:usize) -> bool {
    rm < RM_HALL
}


// A game-state is the room vector plus a cumulative movement cost.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        let mut rm = vec![RM_EMPTY;RM_HALL];
        for ch in line.chars() {
            rm.push( match ch {
                'A' => RM_PODA,
                'B' => RM_PODB,
                'C' => RM_PODC,
                'D' => RM_PODD,
                _   => RM_EMPTY,
            } )
        }
        assert!(rm.len() == RM_SIZE_SM || rm.len() == RM_SIZE_XL);
        GameState { rm:rm, cost:0 }
    }

    // Is this game in a win condition?
    fn is_won(&self) -> bool {
        self.rm.iter().enumerate().all(|(n,r)| *r == expect(n))
    }

    // Is a given room empty?
    fn empty(&self, idx:usize) -> bool {
        self.rm[idx] == RM_EMPTY
    }

    // Is a given segment of hallway empty? (Inclusive bounds)
    fn hall_empty(&self, lo:usize, hi:usize) -> bool {
        assert!(is_hall(lo) && is_hall(hi) && lo <= hi);
        self.rm[lo..hi+1].iter().all(|&r| r == RM_EMPTY)
    }

    // Do all sections of a room match the template? (Inclusive bounds)
    fn room_empty(&self, lo:usize, hi:usize) -> bool {
        assert!(!is_hall(lo) && !is_hall(hi) && lo <= hi);
        let rows = 1 + (hi - lo) / 4;       // How many rows to check?
        assert_eq!(lo + 4*rows, hi + 4);    // Confirm same column
        for r in 0..rows {
            if self.rm[lo + 4*r] != RM_EMPTY {return false;}
        }
        return true
    }

    // Path length from room to designated spot in hallway,
    // confirming that intervening cells are free.
    fn path(&self, from:usize, to:usize) -> Option<usize> {
        // Get the hallway and room coordinates.
        let hh = std::cmp::min(from, to);
        let rr = std::cmp::max(from, to);
        assert!(is_hall(hh));
        assert!(!is_hall(rr));
        // Calculate room-related indices.
        let nn = (rr-RM_HALL) % 4;  // Room index?
        let uu = (rr-RM_HALL) / 4;  // Steps to entrance
        let cc = 2 * (nn+1);        // Entrance (hallway)
        let tt = RM_HALL + nn;      // Entrance (top of room)
        // Check that the intervening path is clear...
        if (rr > tt) && !self.room_empty(tt, rr-4) {
            None                    // Room is blocked
        } else if (hh < cc) && self.hall_empty(hh+1,cc) {
            Some(1 + uu + cc - hh)  // Left of entrance
        } else if (hh > cc) && self.hall_empty(cc,hh-1) {
            Some(1 + uu + hh - cc)  // Right of entrance
        } else {
            None                    // Hallway is blocked
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
            // Can't move to upper cell if any lower cell would be trapped.
            let mut lo = to + 4;
            while lo < self.rm.len() {
                if self.rm[lo] != expect(to) {return None;} lo += 4;
            }
        } else {
            // Moving from a room to a hallway.
            assert!(is_hall(to));
            // Never move to one of the entranceways.
            if to == 2 || to == 4 || to == 6 || to == 8 {return None;}
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
                for to in RM_HALL..self.rm.len() {
                    if let Some(st) = self.pmove(from,to) {next.push(st);}
                }
            } else {
                for to in 0..RM_HALL {
                    if let Some(st) = self.pmove(from,to) {next.push(st);}
                }
            }
        }
        next
    }
}

impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let rr:usize = (self.rm.len() - RM_HALL) / 4;    // How many rows?
        let ch:Vec<char> = self.rm.iter().map(|r| rm2char(*r)).collect();
        write!(f, "############# Cost: {}\n#", self.cost)?;
        for n in 0..RM_HALL {write!(f, "{}", ch[n])?;}
        writeln!(f, "#")?;  // First row (hallway)
        for r in 0..rr {    // Next rows (rooms)
            let n = RM_HALL + 4*r;
            writeln!(f, "###{}#{}#{}#{}###", ch[n+0], ch[n+1], ch[n+2], ch[n+3])?;
        }
        writeln!(f, "#############")
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

    // Insert the starting point.
    queue.push(start.clone());
    best.insert(start.rm.clone(), start.cost);

    // Keep popping from priority queue until we find the solution.
    while let Some(state) = queue.pop() {
        // Stop immediately if we've reached the win state.
        if state.is_won() {return Some(state.cost);}
        // Skip nodes that we've already visited.
        if !done.insert(state.rm.clone()) {continue;}
        // Optionally print current state if verbose.
        if VERBOSE {eprint!("{}", state)}
        // Otherwise, process each of the immediate neighbors.
        for next in state.next().into_iter() {
            let old = best.entry(next.rm.clone()).or_insert(u64::MAX);
            if next.cost < *old {
                *old = next.cost;
                queue.push(next);
            }
        }
    }
    return None
}

pub fn solve() {
    let test1 = GameState::new("BCBDADCA");
    let data1 = GameState::new("ADBDBCAC");
    let test2 = GameState::new("BCBDDCBADBACADCA");
    let data2 = GameState::new("ADBDDCBADBACBCAC");

    assert_eq!(dijkstra(&test1).unwrap(), 12521);
    println!("Part1: {}", dijkstra(&data1).unwrap());
    assert_eq!(dijkstra(&test2).unwrap(), 44169);
    println!("Part2: {}", dijkstra(&data2).unwrap());
}
