/// Advent of Code 2016, Day 25
/// Copyright 2023 by Alex Utter

// From hand analysis of the input program, the equivalent pseudocode is:
//      loop {
//          a = init + 2550;
//          loop {
//              out(a % 2);
//              a = floor(a/2);
//          } while a > 0;
//      }
// Therefore we need to find an initial value that generates an
// even / odd / even / odd... sequence under repeated division.

fn check(guess: u64) -> bool {
    let mut next = guess;
    while next > 0 {
        if next % 2 != 0 {return false;}
        next /= 2;
        if next % 2 != 1 {return false;}
        next /= 2;
    }
    return true;
}

fn main() {
    let mut guess = 1u64;
    while !check(guess + 2550) { guess += 1; }
    println!("Part 1: {guess}");
}
