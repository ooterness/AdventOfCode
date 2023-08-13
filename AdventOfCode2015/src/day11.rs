/// Advent of Code 2015, Day 11
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

// Commonly used characters:
const CH_A: u32 = 'a' as u32;
const CH_I: u32 = 'i' as u32;
const CH_L: u32 = 'l' as u32;
const CH_O: u32 = 'o' as u32;
const CH_Z: u32 = 'z' as u32;
const BAD_CHARS: [u32;3] = [CH_I, CH_L, CH_O];

struct Password {
    chars: Vec<u32>,
}

impl Password {
    fn new(input: &str) -> Self {
        // One-to-one conversion unless we see a forbidden character.
        // If so, later characters replaced with 'z' for faster rollover.
        let mut skip_flag = false;
        let mut result = Vec::new();
        for ch in input.chars().map(|c| c as u32) {
            if skip_flag {
                result.push(CH_Z);
            } else {
                result.push(ch);
                skip_flag = BAD_CHARS.contains(&ch);
            }
        }
        return Password { chars:result }
    }

    fn valid(&self) -> bool {
        let mut flag_double = false;
        let mut flag_triple = false;
        let mut p2 = 0u32;
        let mut p1 = 0u32;
        let mut d0 = 0u32;
        for &ch in self.chars.iter() {
            if ch == p1 && d0 == 0 {
                d0 = ch;                // First pair
            } else if ch == p1 && ch != d0 {
                flag_double = true;     // Second pair
            }
            if ch == p1+1 && ch == p2+2 {
                flag_triple = true;     // Triplet
            }
            p2 = p1; p1 = ch;           // Update history
        }
        return flag_double && flag_triple;
    }

    fn incr(&mut self) {
        for ch in self.chars.iter_mut().rev() {
            if *ch == CH_Z {
                *ch = CH_A; continue;   // Wraparound
            } else if BAD_CHARS.contains(&(*ch+1)) {
                *ch = *ch + 2; break;   // Skip I/L/O
            } else {
                *ch = *ch + 1; break;   // Increment
            }
        }
    }

    fn to_string(&self) -> String {
        self.chars.iter().map(|&x| char::from_u32(x).unwrap()).collect()
    }
}

fn part1(input: &str) -> String
{
    let mut pass = Password::new(input);
    pass.incr();                            // Always increment at least once...
    while !pass.valid() {pass.incr();}      // Stop at the next valid password.
    return pass.to_string();
}

fn part2(input: &str) -> String
{
    return part1(&part1(input));
}

fn main() {
    // Fetch input from server.
    let input = fetch::get_data(2015, 11).unwrap();

    // Unit tests on provided examples.
    assert!(!Password::new("hijklmmn").valid());
    assert!(!Password::new("abbceffg").valid());
    assert!(!Password::new("abbcegjk").valid());
    assert!(Password::new("abcdffaa").valid());
    assert!(Password::new("ghjaabcc").valid());
    assert_eq!(part1("abcdefgh"), "abcdffaa");
    assert_eq!(part1("ghijklmn"), "ghjaabcc");

    // Solve for real input.
    println!("Part 1: {}", part1(input.trim()));
    println!("Part 2: {}", part2(input.trim()));
}
