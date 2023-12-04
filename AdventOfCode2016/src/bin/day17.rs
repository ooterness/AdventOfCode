/// Advent of Code 2016, Day 17
/// Copyright 2023 by Alex Utter

use aocfetch;
use md5;
use std::collections::VecDeque;

fn door_open(x: u8) -> bool {(x & 0xF) >= 0xB}

struct Position {
    row: i8,
    col: i8,
    key: String,
    seq: String,
}

impl Position {
    fn new(input: &str) -> Self {
        Position { row:0, col:0, key:input.trim().to_string(), seq:String::new() }
    }

    fn solved(&self) -> bool {
        (self.row == 3) && (self.col == 3)
    }

    fn adj(&self) -> Vec<Self> {
        // MD5 hash indicates which doors are open.
        let salt = self.key.clone() + &self.seq;
        let hash = md5::compute(salt.as_bytes());
        let uu = door_open(hash[0] >> 4) && self.row > 0;
        let dd = door_open(hash[0] >> 0) && self.row < 3;
        let ll = door_open(hash[1] >> 4) && self.col > 0;
        let rr = door_open(hash[1] >> 0) && self.col < 3;
        // Create a vector of all valid moves.
        let mut tmp = Vec::with_capacity(4);
        if uu { tmp.push(self.mv(-1,  0, "U")); }
        if dd { tmp.push(self.mv( 1,  0, "D")); }
        if ll { tmp.push(self.mv( 0, -1, "L")); }
        if rr { tmp.push(self.mv( 0,  1, "R")); }
        return tmp;
    }

    fn mv(&self, dr:i8, dc:i8, ds:&str) -> Self {
        Position {
            row: self.row + dr,
            col: self.col + dc,
            key: self.key.clone(),
            seq: self.seq.clone() + ds,
        }
    }
}

// Search function is breadth-first-search, except there's no "visited"
// check because all movement sequences are inherently unique.
fn solve(input: &str, shortest: bool) -> String {
    let mut max_path = String::new();
    let mut queue: VecDeque<Position> = VecDeque::new();
    queue.push_back(Position::new(input));
    while let Some(posn) = queue.pop_front() {
        for next in posn.adj().into_iter() {
            if !next.solved() {
                queue.push_back(next);
            } else if shortest {
                return next.seq;
            } else if next.seq.len() > max_path.len() {
                max_path = next.seq;
            }
        }
    }
    return max_path;
}

fn part1(input: &str) -> String {
    solve(input, true)
}

fn part2(input: &str) -> usize {
    solve(input, false).len()
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 17).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1("ihgpwlah"), "DDRRRD");
    assert_eq!(part1("kglvqrro"), "DDUDRLRRUDRD");
    assert_eq!(part1("ulqzkmiv"), "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
    assert_eq!(part2("ihgpwlah"), 370);
    assert_eq!(part2("kglvqrro"), 492);
    assert_eq!(part2("ulqzkmiv"), 830);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
