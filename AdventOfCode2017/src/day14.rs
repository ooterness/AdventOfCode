/// Advent of Code 2017, Day 14
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
#[path = "knot.rs"] mod knot;
use std::collections::HashSet;

type Tile = (i64, i64);
type Grid = HashSet<Tile>;

// Bytes are considered MSB-first.
const MSB_FIRST: [u8; 8] = [128, 64, 32, 16, 8, 4, 2, 1];

// Add '1' bits from the designated 128-bit row-hash.
fn add_row(grid: &mut Grid, key: &str, row: usize) {
    // Calculate salted hash as a byte-array.
    let input = format!("{}-{}", key, row);
    let bytes = knot::hash_raw(&input);
    // Convert byte-array to booleans, MSB-first.
    for (cc, byte) in bytes.iter().enumerate() {
        for (c, mask) in MSB_FIRST.iter().enumerate() {
            let rc = (row as i64, (8*cc+c) as i64);
            if (byte & mask) > 0 {grid.insert(rc);}
        }
    }
}

// Return a HashSet of all '1' bits in a 128 x 128 grid.
fn make_grid(key: &str) -> Grid {
    let mut grid = Grid::new();
    for row in 0..128 {add_row(&mut grid, key, row);}
    return grid;
}

// Find all tiles reachable from a given starting point.
fn reachable(grid: &Grid, from: &Tile) -> Grid {
    // Set initial state.
    let mut queue: Vec<Tile> = vec![from.clone()];
    let mut visit: HashSet<Tile> = HashSet::new();
    visit.insert(from.clone());
    // Breadth first search...
    while queue.len() > 0 {
        let node = queue.pop().unwrap();
        let adj = vec![(node.0-1, node.1),
                       (node.0+1, node.1),
                       (node.0, node.1-1),
                       (node.0, node.1+1)];
        for next in adj.into_iter() {
            if grid.contains(&next) && visit.insert(next.clone()) {
                queue.push(next);
            }
        }
    }
    return visit;
}

// Count the number of true squares.
fn part1(key: &str) -> usize {
    make_grid(key.trim()).len()
}

// Count the number of disconnected regions.
fn part2(key: &str) -> usize {
    // Create the grid of '1' values.
    let mut count = 0usize;
    let mut pending = make_grid(key.trim());
    // Pick a tile at random and remove all connected tiles.
    while pending.len() > 0 {
        count += 1; // Count the next region.
        let root = pending.iter().next().unwrap();
        let conn = reachable(&pending, &root);
        for c in conn.iter() {pending.remove(&c);}
    }
    return count;
}

fn main() {
    // Fetch problem input from server.
    let input = fetch::get_data(2017, 14).unwrap();

    // Unit tests on provided examples.
    assert_eq!(part1("flqrgnkx"), 8108);
    assert_eq!(part2("flqrgnkx"), 1242);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
