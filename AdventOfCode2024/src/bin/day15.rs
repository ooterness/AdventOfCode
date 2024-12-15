/// Advent of Code 2024, Day 15
/// Copyright 2024 by Alex Utter

use aocfetch;
use std::collections::HashSet;

type Rc = (usize, usize);
type Delta = (isize, isize);
type Moves = Vec<Delta>;

fn add(rc:&Rc, mv:&Delta, k:usize) -> Rc {
    (rc.0.saturating_add_signed(mv.0 * k as isize),
     rc.1.saturating_add_signed(mv.1 * k as isize))
}

struct Warehouse {
    robot: Rc,
    boxes: HashSet<Rc>,
    walls: HashSet<Rc>,
}

impl Warehouse {
    fn new(input: &str) -> (Warehouse, Moves) {
        let mut init = Warehouse {
            robot: (0,0),
            boxes: HashSet::new(),
            walls: HashSet::new(),
        };
        let mut moves = Vec::new();
        for (r,line) in input.trim().lines().enumerate() {
            for (c,ch) in line.trim().chars().enumerate() {
                match ch {
                    '@' => {init.robot = (r,c);},
                    'O' => {init.boxes.insert((r,c));},
                    '#' => {init.walls.insert((r,c));},
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

    fn push(&mut self, mv: &Delta) -> bool {
        // Can the move be completed?
        let mut boxes = 0usize;
        loop {
            let rc = add(&self.robot, mv, 1 + boxes);
            if self.boxes.contains(&rc) {
                // Push any number of consecutive boxes.
                boxes += 1;
            } else if self.walls.contains(&rc) {
                // If we hit a wall, the move cannot be applied.
                return false;
            } else {
                // Update the warehouse state
                let new_robot = add(&self.robot, mv, 1);
                let new_boxes = add(&self.robot, mv, 1 + boxes);
                if boxes > 0 {
                    self.boxes.remove(&new_robot);
                    self.boxes.insert(new_boxes);
                }
                self.robot = new_robot;
                return true;
            }
        }
    }
}

fn part1(input: &str) -> usize {
    let (mut state, moves) = Warehouse::new(input);
    for mv in moves.iter() {state.push(mv);}
    return state.gps();
}

fn part2(input: &str) -> usize {
    0 //???
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

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
