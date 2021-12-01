/// Commonly-used library functions for my Advent of Code solutions
/// Copyright 2021 by Alex Utter

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Result;
use std::str::FromStr;

/// Read a file with one String per line.
#[allow(dead_code)]
pub fn read_lines(filename: &str) -> Vec<String>
{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok).collect()
}

/// Read a file with one number per line.
#[allow(dead_code)]
pub fn read_lines_as<T: FromStr>(filename: &str) -> Vec<T>
{
    let file = File::open(filename).unwrap();
    let lines = BufReader::new(file).lines();
    lines.filter_map(Result::ok)
         .filter_map(|line| line.trim().parse::<T>().ok())
         .collect()
}

/// Parse character-delimited string as Vec<T>
#[allow(dead_code)]
pub fn split_str_as<T: FromStr>(line: &str, delim:char) -> Vec<T>
{
    line.split(delim)
        .filter_map(|x| x.parse::<T>().ok())
        .collect()
}

/// Print a labelled list of items.
#[allow(dead_code)]
pub fn print_list<T: std::fmt::Display>(lbl: &str, iter: impl Iterator<Item=T>) {
    print!("{}: [", lbl);
    for (n,x) in iter.enumerate() {
        if n == 0   {print!("{}", x);}
        else        {print!(", {}", x);}
    }
    println!("]");
}
