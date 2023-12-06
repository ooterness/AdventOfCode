# Advent Of Code Fetch

This crate is used to fetch input data for the [Advent of Code](https://adventofcode.com/) puzzles.

The API is inspired by the Python ["advent-of-code-data"](https://pypi.org/project/advent-of-code-data/)
package, including the use of the "AOC_SESSION" environment variable for authentication.

To use this crate:
* Follow the [AoCD instructions](https://pypi.org/project/advent-of-code-data/) to set the AOC_SESSION environment variable.\
This key is used for authentication and should not be shared with anyone.
* Add the `aocfetch` crate to your Cargo.toml `[dependencies]` section:\
`aocfetch = { git = "https://github.com/ooterness/AdventOfCode.git" }`
* Import the crate and call `aocfetch::get_data(year, day)` to fetch your input data.

An example:
```
use aocfetch;

fn main() {
    let input = aocfetch::get_data(2023, 1).unwrap();
    println!("My input data: {}", input);
    println!("Part 1 solution: 42");    // TODO
    println!("Part 2 solution: 42");    // TODO
}
```

Copyright 2021-2023 by Alex Utter
