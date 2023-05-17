/// Advent of Code 2016, Day 6
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn solve(input: &str, part1: bool) -> String
{
    // Measure the length of the target word.
    let word_len = input.trim().lines().nth(0).unwrap().len();

    // Count occurences of each letter in each position.
    let mut counts = vec![0u64; 26*word_len];
    for line in input.trim().lines() {
        for (n,ch) in line.trim().chars().enumerate() {
            let idx = n*26 + (ch as u32 - 'a' as u32) as usize;
            counts[idx] += 1;
        }
    }

    // Find max (part1) or min (part2) for each position.
    let mut result = String::new();
    for w in 0..word_len {
        let range = counts[w*26 .. (w+1)*26].iter()
            .enumerate().map(|(idx,count)| (count,idx as u32));
        let idx = if part1 {range.max()} else {range.min()};
        let chr = char::from_u32(idx.unwrap().1 + 'a' as u32);
        result.push(chr.unwrap());
    }
    return result;
}

fn part1(input: &str) -> String
{
    solve(input, true)
}

fn part2(input: &str) -> String
{
    solve(input, false)
}

const TEST: &str = "\
eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2016, 6).unwrap();

    // Unit tests on provided examples
    assert_eq!(part1(TEST), "easter");

    // Solve for real input.
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
