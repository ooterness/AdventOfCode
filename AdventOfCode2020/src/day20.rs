/// Day 20: https://adventofcode.com/2020/day/20
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

/// A single image-tile.
struct Tile {
    idx:    usize,          // ID# for this tile
    edges:  Vec<usize>,     // Hash for each fwd/rev edge
}

impl Tile {
    /// Parse a tile description.
    fn new(lines:&Vec<String>) -> Tile {
        // First line contains the index.
        let idx = lines[0][5..9].parse::<usize>().unwrap_or(0);
        // Extract the edges moving clockwise.
        let size    = lines[1].len();
        let char0   = |s:&String| s.chars().nth(0).unwrap();
        let charn   = |s:&String| s.chars().nth(size-1).unwrap();
        let top:Vec<char>     = lines[1].chars().collect();
        let right:Vec<char>   = lines[1..].iter().map(char0).collect();
        let bottom:Vec<char>  = lines[size].chars().rev().collect();
        let left:Vec<char>    = lines[1..].iter().map(charn).rev().collect();
        // Save numeric hashes for each edge.
        let edges = vec![
            Tile::edge_hash(top.iter()),
            Tile::edge_hash(right.iter()),
            Tile::edge_hash(bottom.iter()),
            Tile::edge_hash(left.iter()),
            Tile::edge_hash(top.iter().rev()),
            Tile::edge_hash(right.iter().rev()),
            Tile::edge_hash(bottom.iter().rev()),
            Tile::edge_hash(left.iter().rev()),
        ];
        // Return the new object.
        Tile {idx:idx, edges:edges}
    }

    /// Create hash from a pseudo-string describing an edge.
    fn edge_hash<'a>(s: impl Iterator<Item=&'a char>) -> usize {
        let mut sum = 0usize;
        for (n,c) in s.enumerate() {
            if *c == '#' {sum += 2usize.pow(n as u32);}
        }
        sum
    }
}

/// A set of image-tiles.
struct TileSet {
    tiles: Vec<Tile>,               // Array of all tiles
    edges: HashMap<usize,usize>,    // Count edges matching a given hash
}

impl TileSet {
    fn new(lines:&Vec<String>) -> TileSet {
        // Break input into individual tiles.
        let group = common::group_strings(lines);
        let tiles:Vec<Tile> = group.iter().map(|g| Tile::new(g)).collect();
        // Compile a list of valid edges.
        let mut edges:HashMap<usize,usize> = HashMap::new();
        for tile in tiles.iter() {
            for edge in tile.edges.iter() {
                let count:usize = *edges.get(edge).unwrap_or(&0);
                edges.insert(*edge, count+1);
            }
        }
        // Return the new object.
        TileSet {tiles:tiles, edges:edges}
    }

    fn edge_is_unique(&self, edge:&usize) -> bool {
        let count = self.edges.get(edge).unwrap_or(&0);
        *count == 1
    }

    // Find a list of possible corners.
    fn corners(&self) -> Vec<usize> {
        let mut list:Vec<usize> = Vec::new();
        for tile in self.tiles.iter() {
            let mut count = 0usize;
            for n in 0..4 {
                if self.edge_is_unique(&tile.edges[n+0]) &&
                   self.edge_is_unique(&tile.edges[n+4]) {
                    count += 1;
                }
            }
            if count == 2 {list.push(tile.idx);}
        }
        list
    }

    // Find product of IDs for possible corners.
    fn cproduct(&self) -> u64 {
        let corners = self.corners();
        if corners.len() == 4 {
            corners.iter().map(|x| *x as u64).product()
        } else {
            0u64
        }
    }
}

pub fn solve() {
    let test1 = TileSet::new(&common::read_strings("input/test20.txt"));
    let input = TileSet::new(&common::read_strings("input/input20.txt"));

    println!("Test1: {}", test1.cproduct());
    println!("Part1: {}", input.cproduct());
}
