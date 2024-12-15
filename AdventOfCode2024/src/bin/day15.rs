/// Advent of Code 2024, Day 15
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Rc = (usize, usize);
type Delta = (isize, isize);
type Moves = Vec<Delta>;

fn add(rc:&Rc, mv:&Delta) -> Rc {
    (rc.0.saturating_add_signed(mv.0),
     rc.1.saturating_add_signed(mv.1))
}

struct Warehouse {
    part2: bool,
    robot: Rc,
    boxes: HashSet<Rc>,
    walls: HashSet<Rc>,
}

impl Warehouse {
    fn new(input: &str, part2: bool) -> (Warehouse, Moves) {
        let mut init = Warehouse {
            part2: part2,
            robot: (0,0),
            boxes: HashSet::new(),
            walls: HashSet::new(),
        };
        let mut moves = Vec::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                let c2 = if part2 {2*c} else {c};
                match ch {
                    '@' => {init.robot = (r,c2);},
                    'O' => {init.boxes.insert((r,c2));},
                    '#' => {init.walls.insert((r,c2));
                            if part2 {init.walls.insert((r,c2+1));}},
                    '^' => {moves.push((-1, 0));},
                    '>' => {moves.push(( 0, 1));},
                    'v' => {moves.push(( 1, 0));},
                    '<' => {moves.push(( 0,-1));},
                    _   => {},
                }
            }
        }
        return (init, moves);
    }

    fn gps(&self) -> usize {
        self.boxes.iter().map(|(r,c)| 100*r + c).sum()
    }

    fn get_box(&self, rc: &Rc) -> Option<Rc> {
        let ll = add(rc, &(0,-1));
        let rr = rc.clone();
        if self.part2 && self.boxes.contains(&ll) {
            return Some(ll);
        } else if self.boxes.contains(&rr) {
            return Some(rr);
        } else {
            return None;
        }
    }

    fn push(&mut self, mv: &Delta) -> bool {
        // Identify all boxes affected by this push.
        let mut boxes = HashSet::new();     // List of affected boxes
        let mut queue = Vec::new();         // Squares being pushed
        queue.push(add(&self.robot, mv));
        while let Some(rc) = queue.pop() {
            if let Some(bx) = self.get_box(&rc) {
                // Push all square(s) affected by this box.
                let left  = add(&bx, mv);
                let right = add(&left, &(0,1));
                boxes.insert(bx);
                if left != rc {queue.push(left);}
                if self.part2 && right != rc {queue.push(right);}
            } else if self.walls.contains(&rc) {
                // If we hit a wall, the move cannot be applied.
                return false;
            }
        }

        // Move successful, update the warehouse state.
        self.robot = add(&self.robot, mv);
        for bx in boxes.iter() {self.boxes.remove(bx);}
        for bx in boxes.iter() {self.boxes.insert(add(bx, mv));}
        return true;
    }
}

fn part1(input: &str) -> usize {
    let (mut state, moves) = Warehouse::new(input, false);
    for mv in moves.iter() {state.push(mv);}
    return state.gps();
}

fn part2(input: &str) -> usize {
    let (mut state, moves) = Warehouse::new(input, true);
    for mv in moves.iter() {state.push(mv);}
    return state.gps();
}

const EXAMPLE1: &'static str = "\
    ########
    #..O.O.#
    ##@.O..#
    #...O..#
    #.#.O..#
    #...O..#
    #......#
    ########

    <^^>>>vv<v>>v<<";

const EXAMPLE2: &'static str = "\
    ##########
    #..O..O.O#
    #......O.#
    #.OO..O.O#
    #..O@..O.#
    #O#..O...#
    #O..O..O.#
    #.OO.O.OO#
    #....O...#
    ##########

    <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
    vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
    ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
    <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
    ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
    ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
    >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
    <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
    ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
    v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 15).unwrap();

    assert_eq!(part1(EXAMPLE1), 2028);
    assert_eq!(part1(EXAMPLE2), 10092);
    assert_eq!(part2(EXAMPLE2), 9021);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
