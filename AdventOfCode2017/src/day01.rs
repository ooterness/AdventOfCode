/// Advent of Code 2017, Day 1
/// Copyright 2023 by Alex Utter

#[path = "fetch.rs"] mod fetch;

fn main() {
    let data = fetch::get_data(2017, 1).unwrap();
    println!("Hello, world!\n{}", &data); //???
}
