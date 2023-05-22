/// Advent of Code 2016, Day 13
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct XY(i64, i64);

fn count_bits(x: u64) -> u8 {
    let mut count = 0u8;
    for n in 0..64 {
        if x & (1u64<<n) > 0 {count += 1;}
    }
    return count;
}

fn adjacent(xy: &XY) -> [XY;4] {
    return [XY(xy.0+1, xy.1),
            XY(xy.0-1, xy.1),
            XY(xy.0, xy.1+1),
            XY(xy.0, xy.1-1)];
}

fn is_open(key: u64, xy: &XY) -> bool {
    if xy.0 < 0 || xy.1 < 0 {return false;}
    let (x, y) = (xy.0 as u64, xy.1 as u64);
    let tmp = x*x + 3*x + 2*x*y + y + y*y;
    return count_bits(tmp + key) % 2 == 0;
}

fn bfs(key: u64, init: &XY, goal: Option<&XY>, dist: Option<usize>) -> usize {
    let mut queue = VecDeque::<(XY,usize)>::new();
    let mut visit = HashSet::<XY>::new();
    queue.push_back((init.clone(), 0usize));
    visit.insert(init.clone());
    while let Some((xy,count)) = queue.pop_front() {
        if let Some(d) = dist {         // Limit search distance?
            if count >= d {continue;}
        }
        for adj in adjacent(&xy) {
            if let Some(g) = goal {     // Specific goal?
                if adj == *g {return count+1;}
            }
            if is_open(key, &adj) && visit.insert(adj) {
                queue.push_back((adj,count+1));
            }
        }
    }
    return visit.len();
}

fn part1(input: &str) -> usize {
    let key: u64 = input.trim().parse().unwrap();
    return bfs(key, &XY(1,1), Some(&XY(31,39)), None);
}

fn part2(input: &str) -> usize {
    let key: u64 = input.trim().parse().unwrap();
    return bfs(key, &XY(1,1), None, Some(50));
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 13).unwrap();

    // Unit tests on provided examples
    assert_eq!(bfs(10, &XY(1,1), Some(&XY(7,4)), None), 11);

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
