/// Advent of Code 2024, Day 9
/// Copyright 2024 by Alex Utter

use aocfetch;

struct Disk {
    blk: Vec<Option<usize>>,
}

impl Disk {
    fn new(input: &str) -> Self {
        let mut blk = Vec::new();
        for (n,ch) in input.trim().chars().enumerate() {
            let len = ch.to_digit(10).unwrap();
            if n % 2 == 0 {
                for _ in 0..len {blk.push(Some(n/2));}
            } else {
                for _ in 0..len {blk.push(None);}
            }
        }
        return Disk { blk:blk };
    }

    // Starting from specified index, scan right for number of matching entries.
    fn scan_right(&self, start:usize) -> usize {
        let mut len = 1usize;
        while start + len < self.blk.len() && self.blk[start] == self.blk[start+len] {len += 1;}
        return len;
    }

    // Starting from specified index, scan left for number of matching entries.
    fn scan_left(&self, start:usize) -> usize {
        let mut len = 1usize;
        while len < start && self.blk[start] == self.blk[start-len] {len += 1;}
        return len;
    }

    // Starting from specified index, find the leftmost empty block of a given size.
    fn find_empty(&self, wrpos:usize, rdpos:usize) -> Option<usize> {
        let rsize = self.scan_left(rdpos);  // Size of input block
        let mut idx = wrpos;                // Index of next empty block
        while idx + rsize <= rdpos {
            let empty = self.scan_right(idx);
            if empty >= rsize {return Some(idx);}
            idx += empty;
            while idx < rdpos && self.blk[idx] != None {idx += 1;}
        }
        return None;
    }

    fn compact(&self) -> Self {
        let mut blk = Vec::new();
        let mut rdpos = self.blk.len() - 1;
        while blk.len() <= rdpos {
            if let Some(idx) = self.blk[blk.len()] {
                // Copy input without moving.
                blk.push(Some(idx));
            } else {
                // Move the rightmost block of data.
                blk.push(self.blk[rdpos]);
                rdpos -= 1;
                // Before we continue, skip over empty blocks.
                while self.blk[rdpos] == None {rdpos -= 1;}
            }
        }
        return Disk { blk:blk };
    }

    fn defrag(&self) -> Self {
        let mut disk = Disk { blk: self.blk.clone() };
        let mut wrpos = disk.scan_right(0);
        let mut rdpos = disk.blk.len() - 1;
        while wrpos < rdpos {
            // Can we move the current file?
            let rsize = disk.scan_left(rdpos);
            if let Some(wrtmp) = disk.find_empty(wrpos, rdpos) {
                // Found empty space, move the file contents.
                for n in 0..rsize {disk.blk[wrtmp+n] = disk.blk[rdpos-n];}
                for n in 0..rsize {disk.blk[rdpos-n] = None;}
                // Update pointer to the first empty block?
                while disk.blk[wrpos] != None {wrpos += 1;}
            }
            // Find the next file of interest.
            rdpos -= rsize;
            while disk.blk[rdpos] == None {rdpos -= 1;}
        }
        return disk;
    }

    fn checksum(&self) -> usize {
        self.blk.iter().enumerate().map(|(n,&b)| n*b.unwrap_or(0)).sum()
    }
}

fn part1(input: &str) -> usize {
    Disk::new(input).compact().checksum()
}

fn part2(input: &str) -> usize {
    Disk::new(input).defrag().checksum()
}

const EXAMPLE: &'static str = "2333133121414131402";

fn main() {
    // Fetch input from server.
    let input = aocfetch::get_data(2024, 9).unwrap();

    assert_eq!(part1(EXAMPLE), 1928);
    assert_eq!(part2(EXAMPLE), 2858);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
