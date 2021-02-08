/// Day 20: https://adventofcode.com/2020/day/20
/// Copyright 2021 by Alex Utter
/// Note: It isn't mentioned in the problem statement, but every edge
///       in the provided tile-set has a unique complementary pairing.
///       This makes the problem *MUCH* easier to solve.

use std::collections::HashMap;
#[path = "common.rs"] mod common;

/// Generic "image" matrix
type Matrix = Vec<Vec<char>>;

/// A single image-tile.
#[derive(Clone)]
struct Tile {
    idx:    usize,          // ID# for this tile
    ncols:  usize,          // Number of columns
    nrows:  usize,          // Number of rows
    matrix: Matrix,         // Raw underlying image
    edges:  Vec<usize>,     // Hash for each fwd/rev edge
}

impl Tile {
    /// Parse a tile description.
    fn new(lines:&Vec<String>) -> Tile {
        // First line contains the index.
        let idx = lines[0][5..9].parse::<usize>().unwrap_or(0);
        // Convert from array-of-strings to matrix form.
        let nrows   = lines.len() - 1;
        let ncols   = lines[1].len();
        let matrix  = lines[1..].iter().map(|l| l.chars().collect()).collect();
        assert!(Tile::check_size(&matrix, nrows, ncols));
        // Extract the edges moving clockwise.
        let char_l  = |s:&String| s.chars().nth(0).unwrap();
        let char_r  = |s:&String| s.chars().nth(ncols-1).unwrap();
        let top:    Vec<char> = lines[1].chars().collect();
        let right:  Vec<char> = lines[1..].iter().map(char_r).collect();
        let bottom: Vec<char> = lines[nrows].chars().rev().collect();
        let left:   Vec<char> = lines[1..].iter().rev().map(char_l).collect();
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
        Tile {
            idx:    idx,
            ncols:  ncols,
            nrows:  nrows,
            matrix: matrix,
            edges:  edges,
        }
    }

    /// Create hash from a pseudo-string describing an edge.
    fn edge_hash<'a>(s: impl Iterator<Item=&'a char>) -> usize {
        let mut sum = 0usize;
        for (n,c) in s.enumerate() {
            if *c == '#' {sum += 2usize.pow(n as u32);}
        }
        sum
    }

    /// Mirror a tile horizontally.
    fn mirror(&self) -> Tile {
        // Create a flipped copy of the input matrix.
        let mut matrix = self.matrix.clone();
        for row in matrix.iter_mut() {row.reverse()};
        assert!(Tile::check_size(&matrix, self.nrows, self.ncols));
        // Construct the rest of the object.
        Tile {
            idx:    self.idx,
            ncols:  self.ncols,
            nrows:  self.nrows,
            matrix: matrix,
            edges:  vec![
                self.edges[4], self.edges[7], self.edges[6], self.edges[5],
                self.edges[0], self.edges[3], self.edges[2], self.edges[1]],
        }
    }

    /// Rotate a tile 1 step clockwise.
    fn rotate(&self) -> Tile {
        // Create a rotated copy of the input matrix.
        let out_cel = |r:usize,c:usize| self.matrix[self.nrows-1-c][r];
        let out_row = |r:usize| (0..self.nrows).map(|c| out_cel(r,c)).collect();
        let matrix = (0..self.ncols).map(out_row).collect();
        assert!(Tile::check_size(&matrix, self.ncols, self.nrows));
        // Construct the rest of the object.
        Tile {
            idx:    self.idx,
            ncols:  self.nrows,
            nrows:  self.ncols,
            matrix: matrix,
            edges:  vec![
                self.edges[3], self.edges[0], self.edges[1], self.edges[2],
                self.edges[7], self.edges[4], self.edges[5], self.edges[6]],
        }
    }

    /// Concatentate tiles horizontally (Matlab "[A,B]")
    fn hcat(&self, next:&Tile) -> Option<Tile> {
        // Confirm that the edges line up.
        if (self.nrows == next.nrows) &&
           (self.edges[1] == next.edges[7]) {
            // Concatentate the two matrices.
            let mut matrix = self.matrix.clone();
            for r in 0..self.nrows {
                for c in 1..next.ncols {
                    matrix[r].push(next.matrix[r][c]);
                }
            }
            assert!(Tile::check_size(&matrix, self.nrows, self.ncols + next.ncols - 1));
            // Construct the rest of the object.
            Some(Tile {
                idx:    self.idx,
                ncols:  self.ncols + next.ncols - 1,
                nrows:  self.nrows,
                matrix: matrix,
                edges:  vec![
                    self.edges[0], next.edges[1], self.edges[2], self.edges[3],
                    self.edges[4], next.edges[5], self.edges[6], self.edges[7]],
            })
        } else {
            None
        }
    }
    
    /// Concatentate tiles vertically (Matlab "[A;B]")
    fn vcat(&self, next:&Tile) -> Option<Tile> {
        // Confirm that the edges line up.
        if (self.ncols == next.ncols) &&
           (self.matrix[self.nrows-1] == next.matrix[0]) {
            // Concatentate the two matrices.
            let mut matrix = self.matrix.clone();
            for r in 1..next.nrows {
                matrix.push(next.matrix[r].clone());
            }
            assert!(Tile::check_size(&matrix, self.nrows + next.nrows - 1, self.ncols));
            // Construct the rest of the object.
            Some(Tile {
                idx:    next.idx,
                ncols:  self.ncols,
                nrows:  self.nrows + next.nrows - 1,
                matrix: matrix,
                edges:  vec![
                    self.edges[0], self.edges[1], next.edges[2], next.edges[3],
                    self.edges[4], self.edges[5], next.edges[6], next.edges[7]],
            })
        } else {
            None
        }
    }

    /// Try all possible permutations when building to the right.
    fn attach_col(&self, next:&Tile) -> Option<Tile> {
        let n0 = next.clone();
        let n1 = next.rotate();
        let n2 = n1.rotate();
        let n3 = n2.rotate();
        if let Some(tile) = self.hcat(&n0)              {return Some(tile);}
        if let Some(tile) = self.hcat(&n1)              {return Some(tile);}
        if let Some(tile) = self.hcat(&n2)              {return Some(tile);}
        if let Some(tile) = self.hcat(&n3)              {return Some(tile);}
        if let Some(tile) = self.hcat(&n0.mirror())     {return Some(tile);}
        if let Some(tile) = self.hcat(&n1.mirror())     {return Some(tile);}
        if let Some(tile) = self.hcat(&n2.mirror())     {return Some(tile);}
        if let Some(tile) = self.hcat(&n3.mirror())     {return Some(tile);}
        None
    }

    /// Try all four possible permutations when attaching two rows.
    fn attach_row(&self, next:&Tile) -> Option<Tile> {
        let n180 = next.rotate().rotate();
        if let Some(tile) = self.vcat(next)             {return Some(tile);}
        if let Some(tile) = self.vcat(&n180)            {return Some(tile);}
        if let Some(tile) = self.vcat(&next.mirror())   {return Some(tile);}
        if let Some(tile) = self.vcat(&n180.mirror())   {return Some(tile);}
        None
    }

    /// Matrix size integrity check.
    fn check_size(mat: &Matrix, nrows:usize, ncols:usize) -> bool {
        if mat.len() != nrows {return false;}
        for row in mat.iter() {
            if row.len() != ncols {return false;}
        }
        true
    }

    /// Print one row from a character-matrix.
    fn print_row(row: &Vec<char>) {
        let row:String = row.iter().collect();
        println!("{}", row);
    }

    /// Print this tile.
    fn print(&self) {
        println!("Tile {}:", self.idx);
        for row in self.matrix.iter() {Tile::print_row(row);}
    }

    /// Count total "#" symbols.
    fn count_symbols(mat: &Matrix) -> usize {
        let mut count = 0usize;
        for row in mat.iter() {
            for col in row.iter() {
                if *col == '#' {count += 1;}
            }
        }
        count
    }

    /// Count the number of dragons in the current orientation.
    fn count_dragons_simple(&self) -> usize {
        // Define the "dragon" template.
        let template:Matrix = vec![
            String::from("                  # ").chars().collect(),
            String::from("#    ##    ##    ###").chars().collect(),
            String::from(" #  #  #  #  #  #   ").chars().collect(),
        ];
        let trows:usize = template.len();
        let tcols:usize = template[0].len();
        // Sanity check on input sizes.
        assert!(Tile::check_size(&template, trows, tcols));
        assert!(Tile::check_size(&self.matrix, self.nrows, self.ncols));
        assert!((self.nrows >= trows) && (self.ncols >= tcols));
        // Iterate over the search space and count each match.
        let nrows = self.nrows + 1 - trows;
        let ncols = self.ncols + 1 - tcols;
        let mut dragons = 0usize;
        for rr in 0..nrows {
            for cc in 0..ncols {
                let mut found = true;
                for dr in 0..trows {
                    for dc in 0..tcols {
                        if (template[dr][dc] == '#') &&
                           (self.matrix[rr+dr][cc+dc] != '#') {
                            found = false;
                        }
                    }
                }
                if found {dragons += 1usize;}
            }
        }
        dragons
    }

    /// Count the number of dragons in any orientation.
    fn count_dragons_any(&self) -> usize {
        // Construct each possible orientation...
        let n0 = self.clone();
        let n1 = n0.rotate();
        let n2 = n1.rotate();
        let n3 = n2.rotate();
        let dcount = vec![
            n0.mirror(), n0,
            n1.mirror(), n1,
            n2.mirror(), n2,
            n3.mirror(), n3,
        ];
        dcount.iter().map(|n| n.count_dragons_simple()).max().unwrap()
    }

    /// Surface-roughness = Number of non-dragon "#" symbols.
    fn roughness(&self) -> usize {
        Tile::count_symbols(&self.matrix) - 15usize * self.count_dragons_any()
    }

    /// Remove borders from each NxN sub-tile.
    fn remove_borders(&self, grid:usize) -> Tile {
        // Iterate over the grid.
        let mut matrix:Matrix = Vec::new();
        let is_grid = |x:usize| ((x % grid) == 0);
        for r in 0..self.nrows {
            if is_grid(r) {continue;}
            let mut row:Vec<char> = Vec::new();
            for c in 0..self.ncols {
                if is_grid(c) {continue;}
                row.push(self.matrix[r][c]);
            }
            matrix.push(row);
        }
        // Construct the new Tile object.
        Tile {
            idx:    0usize,
            ncols:  matrix[0].len(),
            nrows:  matrix.len(),
            matrix: matrix,
            edges:  vec![0,0,0,0,0,0,0,0],
        }
    }
}

/// A set of image-tiles.
struct TileSet {
    tiles: HashMap<usize,Tile>,         // Map of all tiles by index
    edges: HashMap<usize,Vec<usize>>,   // Indices matching a given edge-hash
}

impl TileSet {
    /// Parse problem description into a set of tiles.
    fn new(lines:&Vec<String>) -> TileSet {
        // Break input into individual tiles.
        let input = common::group_strings(lines);
        let mut tiles: HashMap<usize,Tile> = HashMap::new();
        let mut edges: HashMap<usize,Vec<usize>> = HashMap::new();
        for grp in input.iter() {
            // Parse this tile and add its edges to the combined list.
            let tile = Tile::new(grp);
            for edge in tile.edges.iter() {
                if let Some(v) = edges.get_mut(edge) {
                    v.push(tile.idx);           // Add to list
                } else {
                    let v = vec![tile.idx];
                    edges.insert(*edge, v);     // New list
                }
            }
            tiles.insert(tile.idx, tile);
        }
        // Return the new object.
        TileSet {tiles:tiles, edges:edges}
    }

    /// Count the number of edges matching a given hash.
    fn count_edges(&self, edge:&usize) -> usize {
        if let Some(v) = self.edges.get(edge) {
            v.len()     // Number of matching edges
        } else {
            0usize      // No such edge-hash
        }
    }

    /// Search for all possible corner tiles.
    /// (i.e., Tiles where only two edges have matching counterparts.)
    fn corners(&self) -> Vec<usize> {
        let mut list:Vec<usize> = Vec::new();
        for tile in self.tiles.values() {
            let mut count = 0usize;
            for n in 0..4 {
                if self.count_edges(&tile.edges[n+0]) == 1 &&
                   self.count_edges(&tile.edges[n+4]) == 1 {
                    count += 1;
                }
            }
            if count == 2 {list.push(tile.idx);}
        }
        list
    }

    /// Find product of IDs for the four possible corners.
    fn cproduct(&self) -> u64 {
        let corners = self.corners();
        if corners.len() == 4 {
            corners.iter().map(|x| *x as u64).product()
        } else {
            0u64
        }
    }

    /// Helper function finds tile-index by matching edge.
    /// (Rotate the tile until the matching edge is on the left.)
    fn matching_tile(&self, edge:usize, prev:usize) -> Option<Tile> {
        if let Some(list) = self.edges.get(&edge) {
            for idx in list.iter() {
                if *idx != prev {
                    let mut tile = self.tiles.get(idx).unwrap().clone();
                    while (tile.edges[3] != edge) && (tile.edges[7] != edge)
                        {tile = tile.rotate();}
                    return Some(tile);
                }
            }
        }
        None
    }

    /// Helper function solves one row given the leftmost piece.
    fn solve_row(&self, left:&Tile) -> Tile {
        let mut prev:usize  = left.idx;
        let mut tile:Tile   = left.clone();
        // Keep adding pieces to the right...
        while let Some(next) = self.matching_tile(tile.edges[1], prev) {
            prev = next.idx;
            tile = tile.attach_col(&next).unwrap();
        }
        tile
    }

    /// Solve the jigsaw puzzle and return image as a single tile.
    fn solve(&self) -> Tile {
        // Start from one of the corners.
        // Rotate the seed until the top and left edges are unmatchable.
        let mut pidx   = self.corners()[0];
        let mut corner = self.tiles.get(&pidx).unwrap().clone();
        while (self.count_edges(&corner.edges[0]) > 1) ||
              (self.count_edges(&corner.edges[3]) > 1) {
            corner = corner.rotate();
        }
        // Solve the first row, then add each neighboring row.
        let mut puzzle = self.solve_row(&corner);
        loop {
            if let Some(left) = self.matching_tile(puzzle.edges[2], pidx) {
                let seed1 = left.rotate();
                let seed2 = seed1.mirror();
                let row1 = self.solve_row(&seed1);
                let row2 = self.solve_row(&seed2);
                if let Some(attach) = puzzle.attach_row(&row1) {
                    pidx   = left.idx;
                    puzzle = attach;    // Add next row
                } else if let Some(attach) = puzzle.attach_row(&row2) {
                    pidx   = left.idx;
                    puzzle = attach;    // Add next row
                } else {
                    eprintln!("Couldn't attach rows:");
                    puzzle.print();
                    row1.print();
                    return puzzle;      // Unable to proceed
                }
            } else {
                return puzzle;          // Reached last row
            }
        }
    }
}

pub fn solve() {
    // Part 1: Identify corners.
    let test1 = TileSet::new(&common::read_strings("input/test20.txt"));
    let input = TileSet::new(&common::read_strings("input/input20.txt"));

    println!("Test1: {}", test1.cproduct());
    println!("Part1: {}", input.cproduct());

    // Part 2: Solve each puzzle and look for dragons.
    let solve1 = test1.solve().remove_borders(9);
    let solve2 = input.solve().remove_borders(9);

    println!("Test2: {} -> {}", solve1.count_dragons_any(), solve1.roughness());
    println!("Part2: {} -> {}", solve2.count_dragons_any(), solve2.roughness());
}
