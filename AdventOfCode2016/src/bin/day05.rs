/// Advent of Code 2016, Day 5
/// Copyright 2023 by Alex Utter

use aocfetch;

use md5;

// Return the first four bytes of the MD5 hash.
fn md5_prefix(base: &str, idx: u64) -> u64 {
    let salt = format!("{}{}", base, idx);
    let hash = md5::compute(salt.as_bytes());
    return 16777216 * hash[0] as u64
         +    65536 * hash[1] as u64
         +      256 * hash[2] as u64
         +            hash[3] as u64;
}

fn part1(input: &str) -> u64
{
    let mut count = 0usize;
    let mut index = 0u64;
    let mut password = 0u64;
    while count < 8 {
        let hash = md5_prefix(input, index) / 256;
        if hash/16 == 0 {
            count += 1;
            password = 16*password + (hash%16);
            println!("{:08X}", password);
        }
        index += 1;
    }
    return password;
}

fn part2(input: &str) -> u64
{
    let mut pmask = 0u8;
    let mut index = 0u64;
    let mut password = 0u64;
    while pmask < 255 {
        let hash = md5_prefix(input, index);
        let pfix = hash & 0xFFFFF000;
        let posn = (hash & 0x00000F00) / 256;
        let pval = (hash & 0x000000F0) / 16;
        if (pfix == 0) && (posn < 8) {
            let tmask = 1u8 << posn;
            if pmask & tmask == 0 {
                pmask |= tmask;
                password |= pval << (28 - 4*posn);
                println!("{:08X}", password);
            }
        }
        index += 1;
    }
    return password;
}

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2016, 5).unwrap();

    // Unit tests on provided examples
    assert_eq!(md5_prefix("abc", 3231929)/256, 1);
    assert_eq!(md5_prefix("abc", 5017308)/256, 8);
    assert_eq!(md5_prefix("abc", 5278568)/256, 15);
    assert_eq!(part1("abc"), 0x18f47a30);
    assert_eq!(part2("abc"), 0x05ace8e3);

    // Solve for real input.
    println!("Part 1: {:08X}", part1(input.trim()));
    println!("Part 2: {:08X}", part2(input.trim()));
}
