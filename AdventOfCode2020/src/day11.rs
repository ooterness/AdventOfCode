/// Day 10: https://adventofcode.com/2020/day/10
/// Copyright 2021 by Alex Utter

#[path = "common.rs"] mod common;

/// Raster-scan numbered seat-map.
struct RasterMap {
    nrows:  usize,
    ncols:  usize,
    seats:  Vec<Option<usize>>,
}

impl RasterMap {
    fn new(input: &Vec<String>) -> Option<RasterMap> {
        // Measure number of rows and columns.
        let nrows:usize = input.len();
        let ncols:usize = input[0].len();

        // Number each seat 0 through N-1
        let mut raster:Vec<Option<usize>> = vec![None; ncols*nrows];
        let mut next = 0usize;
        for r in 0..nrows {
            let row:Vec<char> = input[r].chars().collect();
            if row.len() != ncols {return None;}
            for c in 0..ncols {
                if row[c] == 'L' {
                    raster[c + r*ncols] = Some(next);
                    next += 1;
                }
            }
        }

        Some(RasterMap {nrows:nrows, ncols:ncols, seats:raster})
    }

    fn get(&self, r:usize, c:usize) -> Option<usize> {
        assert!(r < self.nrows);
        assert!(c < self.ncols);
        self.seats[c + r*self.ncols]
    }

    fn scan(&self, rpt:bool, r:usize, c:usize, dr:usize, dc:usize) -> Option<usize> {
        let r2 = r.overflowing_add(dr).0;               // Modulo-add
        let c2 = c.overflowing_add(dc).0;               // Modulo-add
        if      r2 >= self.nrows {None}                 // Out of bounds
        else if c2 >= self.ncols {None}                 // Out of bounds
        else if let Some(n) = self.get(r2,c2) {Some(n)} // Match
        else if rpt {self.scan(rpt,r2,c2,dr,dc)}        // Recurse
        else {None}                                     // Adjacent only
    }
}

/// Adjacency map: For each seat, list of adjacent seat indices.
struct SeatMap {
    sight: bool,
    seats: Vec<Vec<usize>>,
}

impl SeatMap {
    fn new(map: &RasterMap, sight:bool) -> SeatMap {
        let mut graph:Vec<Vec<usize>> = Vec::new();
        for r in 0..map.nrows {
            for c in 0..map.ncols {
                if let Some(_) = map.get(r,c) {
                    // Check each of eight directions:
                    let mut adj:Vec<usize> = Vec::new();
                    let scan = |dr,dc| map.scan(sight, r, c, dr as usize, dc as usize);
                    let mut add_if = |dr,dc| if let Some(n) = scan(dr,dc) {adj.push(n)};
                    add_if(-1,-1);
                    add_if(-1, 0);
                    add_if(-1, 1);
                    add_if( 0,-1);
                    add_if( 0, 1);
                    add_if( 1,-1);
                    add_if( 1, 0);
                    add_if( 1, 1);
                    graph.push(adj);
                }
            }
        }
        SeatMap {sight:sight, seats:graph}
    }
}

/// Current state of a SeatMap.
struct SeatState<'a> {
    map:  &'a SeatMap,      // Map of adjacent seats
    step: u64,              // Simulation step index
    seat: Vec<bool>,        // Occupied seats
}

impl<'a> SeatState<'a> {
    /// Given adjacency-map, create initial state with no occupied seats.
    fn init(map: &'a SeatMap) -> SeatState<'a> {
        SeatState {
            map:    map,
            step:   0u64,
            seat:   vec![false; map.seats.len()],
        }
    }

    /// Count total occupied seats.
    fn count(&self) -> usize {
        let mut count = 0usize;
        for s in self.seat.iter() {
            if *s {count += 1usize;}
        }
        count
    }

    /// Count occupied seats adjacent to given seat-index.
    fn count_adj(&self, n:usize) -> usize {
        let mut count = 0usize;
        for adj in self.map.seats[n].iter() {
            if self.seat[*adj] {count += 1usize;}
        }
        count
    }

    /// Calculate next simulation timestep.
    fn iterate(&self) -> SeatState<'a> {
        let mut next:Vec<bool> = self.seat.clone();
        for n in 0..next.len() {
            let adj = self.count_adj(n);
            if (adj == 0) && (!self.seat[n]) {
                next[n] = true;
            } else if (adj >= 4) && (!self.map.sight) && (self.seat[n]) {
                next[n] = false;
            } else if (adj >= 5) && (self.seat[n]) {
                next[n] = false;
            }
        }
        SeatState {
            map:    self.map,
            step:   self.step + 1,
            seat:   next,
        }
    }
}

/// Find terminal state from a given seating map.
fn terminal_state(map: &SeatMap) -> SeatState {
    let mut curr = SeatState::init(&map);
    loop {
        let next = curr.iterate();
        if curr.seat == next.seat {return curr;}
        curr = next;
    }
}

pub fn solve() {
    let example = vec![
        String::from("L.LL.LL.LL"),
        String::from("LLLLLLL.LL"),
        String::from("L.L.L..L.."),
        String::from("LLLL.LL.LL"),
        String::from("L.LL.LL.LL"),
        String::from("L.LLLLL.LL"),
        String::from("..L.L....."),
        String::from("LLLLLLLLLL"),
        String::from("L.LLLLLL.L"),
        String::from("L.LLLLL.LL"),
    ];
    let input = common::read_strings("input/input11.txt");

    if let Some(map) = RasterMap::new(&example) {
        let part1 = SeatMap::new(&map, false);
        let part2 = SeatMap::new(&map, true);
        assert_eq!(terminal_state(&part1).count(), 37usize);
        assert_eq!(terminal_state(&part2).count(), 26usize);
    } else {
        eprintln!("Error compiling example.");
    }

    if let Some(map) = RasterMap::new(&input) {
        let part1 = SeatMap::new(&map, false);
        let part2 = SeatMap::new(&map, true);
        println!("Part1: {} occupied seats", terminal_state(&part1).count());
        println!("Part2: {} occupied seats", terminal_state(&part2).count());
    } else {
        eprintln!("Error compiling input.");
    }
}
