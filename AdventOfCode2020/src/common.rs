/// Commonly-used library functions for my Advent of Code solutions
/// Copyright 2021 by Alex Utter

use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

/// Count the number of true items in a list/vector/etc.
#[allow(dead_code)]
pub fn count_true(iter: impl Iterator<Item=bool>) -> usize {
    let mut count = 0usize;
    for x in iter {
        if x {count += 1usize;}
    }
    count
}

/// Read a file with one integer per line.
#[allow(dead_code)]
pub fn read_integers(filename: &str) -> Vec<i64>
{
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.filter_map(io::Result::ok)
         .filter_map(|line| line.trim().parse().ok())
         .collect()
}

/// Read a file with one String per line.
#[allow(dead_code)]
pub fn read_strings(filename: &str) -> Vec<String>
{
    let file = File::open(filename).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.filter_map(io::Result::ok)
         .collect()
}

/// Read a file where blank lines delimit line-groups.
#[allow(dead_code)]
pub fn group_strings(lines:&Vec<String>) -> Vec<Vec<String>>
{
    let mut list = vec![];
    let mut next = Vec::<String>::new();
    for line in lines {                     // Read one line at a time.
        if !line.is_empty() {               // Content line?
            next.push(line.clone());        // Add next line to group
        } else if !next.is_empty() {        // Empty line / delimiter
            list.push(next);                // Add group to output
            next = Vec::<String>::new();    // New empty group
        }
    }
    list.push(next);                        // Add final group
    return list
}

/// Cumulative sum of a vector of integers.
#[allow(dead_code)]
pub fn cumsum(xvec: &Vec<i64>) -> Vec<i64> {
    let mut sum  = 0i64;
    let mut yvec = vec![0i64; xvec.len()];
    for (n,x) in xvec.iter().enumerate() {
        sum += x;
        yvec[n] = sum;
    }
    yvec
}

/// Parse character-delimited string as Vec<T>
#[allow(dead_code)]
pub fn parse_vec<T:FromStr>(line: &str, delim:char) -> Vec<T>
{
    line.split(delim).filter_map(|x| x.parse::<T>().ok()).collect()
}

/// Print a labelled list of items.
#[allow(dead_code)]
pub fn print_list(lbl: &str, iter: impl Iterator<Item=usize>) {
    print!("{}: [", lbl);
    for (n,x) in iter.enumerate() {
        if n == 0   {print!("{}", x);}
        else        {print!(", {}", x);}
    }
    println!("]");
}

/// Split a string in two based on delimiter.
#[allow(dead_code)]
pub fn split2<'a>(line: &'a str, delim: &str) -> Option<(&'a str, &'a str)>
{
    let vec:Vec<&str> = line.split(delim).collect();
    if vec.len() == 2 {
        Some((vec[0], vec[1]))
    } else {
        None
    }
}
