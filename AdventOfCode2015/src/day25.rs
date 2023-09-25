/// Advent of Code 2015, Day 25
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn lcg(x: u64) -> u64 {
    (x * 252533u64) % 33554393u64
}

fn lcg_idx(idx: usize) -> u32 {
    (0..idx).fold(20151125u64, |x,_| lcg(x)) as u32
}

fn lcg_rc(row: usize, col: usize) -> u32 {
    let dd = row + col - 2;
    let ds = dd * (dd+1);
    lcg_idx(ds/2 + col - 1)
}

fn part1(input: &str) -> u32 {
    // Parse the row and column from the input string.
    let rc: Vec<usize> = input.trim().split([' ', ',', '.'])
        .filter_map(|x| x.parse::<usize>().ok()).collect();
    assert_eq!(rc.len(), 2);
    lcg_rc(rc[0], rc[1])
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 25).unwrap();

    // Unit tests based on the provided examples:
    assert_eq!(lcg_rc(1, 1), 20151125);
    assert_eq!(lcg_rc(1, 2), 18749137);
    assert_eq!(lcg_rc(1, 3), 17289845);
    assert_eq!(lcg_rc(1, 4), 30943339);
    assert_eq!(lcg_rc(1, 5), 10071777);
    assert_eq!(lcg_rc(1, 6), 33511524);
    assert_eq!(lcg_rc(2, 1), 31916031);
    assert_eq!(lcg_rc(2, 2), 21629792);
    assert_eq!(lcg_rc(2, 3), 16929656);
    assert_eq!(lcg_rc(2, 4), 7726640);
    assert_eq!(lcg_rc(2, 5), 15514188);
    assert_eq!(lcg_rc(2, 6), 4041754);
    assert_eq!(lcg_rc(3, 1), 16080970);
    assert_eq!(lcg_rc(3, 2), 8057251);
    assert_eq!(lcg_rc(3, 3), 1601130);
    assert_eq!(lcg_rc(3, 4), 7981243);
    assert_eq!(lcg_rc(3, 5), 11661866);
    assert_eq!(lcg_rc(3, 6), 16474243);
    assert_eq!(lcg_rc(4, 1), 24592653);
    assert_eq!(lcg_rc(4, 2), 32451966);
    assert_eq!(lcg_rc(4, 3), 21345942);
    assert_eq!(lcg_rc(4, 4), 9380097);
    assert_eq!(lcg_rc(4, 5), 10600672);
    assert_eq!(lcg_rc(4, 6), 31527494);
    assert_eq!(lcg_rc(5, 1), 77061);
    assert_eq!(lcg_rc(5, 2), 17552253);
    assert_eq!(lcg_rc(5, 3), 28094349);
    assert_eq!(lcg_rc(5, 4), 6899651);
    assert_eq!(lcg_rc(5, 5), 9250759);
    assert_eq!(lcg_rc(5, 6), 31663883);
    assert_eq!(lcg_rc(6, 1), 33071741);
    assert_eq!(lcg_rc(6, 2), 6796745);
    assert_eq!(lcg_rc(6, 3), 25397450);
    assert_eq!(lcg_rc(6, 4), 24659492);
    assert_eq!(lcg_rc(6, 5), 1534922);
    assert_eq!(lcg_rc(6, 6), 27995004);

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
}
