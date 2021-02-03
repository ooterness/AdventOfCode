/// Day 3: https://adventofcode.com/2020/day/3
/// Copyright 2021 by Alex Utter

use std::convert::TryFrom;
#[path = "common.rs"] mod common;

struct TreeMap {
    cols: usize,
    rows: usize,
    tree: Vec<bool>,
}

impl TryFrom<&Vec<String>> for TreeMap {
    type Error = ();

    fn try_from(item: &Vec<String>) -> Result<Self, Self::Error> {
        // One string for each row...
        let nrows = item.len();
        if nrows == 0 {return Err(())}

        // Confirm each row has the same width.
        let ncols = item[0].len();
        for row in item.iter() {
            if row.len() != ncols {return Err(())}
        }

        // Parse the map into trees and empty space.
        let map_size = nrows * ncols;
        let get_char = |n:usize| item[n/ncols].chars().nth(n%ncols).unwrap();
        let get_tree = |n:usize| get_char(n) == '#';
        let trees: Vec<bool> = (0..map_size).map(get_tree).collect();

        // Return a new TreeMap object.
        Ok(TreeMap {cols:ncols, rows:nrows, tree:trees})
    }
}

impl TreeMap {
    fn is_tree(&self, r:usize, c:usize) -> bool {
        self.tree[(self.cols*r) + (c%self.cols)]
    }

    fn count_trees(&self, dc:usize, dr:usize) -> usize {
        let iters = 1 + (self.rows-1) / dr;
        let route = (0..iters).map(|n| self.is_tree(n*dr, n*dc));
        common::count_true(route)
    }
}

fn solve_part2(lbl: &str, tm: TreeMap) {
    let a = tm.count_trees(1, 1);
    let b = tm.count_trees(3, 1);
    let c = tm.count_trees(5, 1);
    let d = tm.count_trees(7, 1);
    let e = tm.count_trees(1, 2);
    println!("{}: {} * {} * {} * {} * {} = {}",
        lbl, a, b, c, d, e, a*b*c*d*e);
}

pub fn solve() {
    // Define the test map from the problem statement.
    let test_str = vec![
        String::from("..##......."),
        String::from("#...#...#.."),
        String::from(".#....#..#."),
        String::from("..#.#...#.#"),
        String::from(".#...##..#."),
        String::from("..#.##....."),
        String::from(".#.#.#....#"),
        String::from(".#........#"),
        String::from("#.##...#..."),
        String::from("#...##....#"),
        String::from(".#..#...#.#"),
    ];

    // Parse the example map:
    if let Ok(test_map) = TreeMap::try_from(&test_str) {
        // Count trees at slope = 3 right, 1 down.
        println!("Test 1: {}", test_map.count_trees(1,3));
        // Count trees at each other slope.
        solve_part2("Test 2", test_map);
    } else {
        println!("Test parse error.");
    }

    // Now read the full-size input...
    let input_str = common::read_strings("input/input03.txt");
    if let Ok(input_map) = TreeMap::try_from(&input_str) {
        // Part 1: Number of trees at slope = 1 down, 3 right.
        println!("Part 1: {}", input_map.count_trees(3, 1));
        // Part 2: Product of the various slopes.
        solve_part2("Part 2", input_map);
    } else {
        println!("Input parse error.");
    }
}
