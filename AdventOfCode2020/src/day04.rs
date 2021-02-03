/// Day 4: https://adventofcode.com/2020/day/4
/// Copyright 2021 by Alex Utter

use std::collections::HashMap;
#[path = "common.rs"] mod common;

struct Passport {
    fields: HashMap<String,String>,
}

impl Passport {
    /// Create a new, empty passport.
    fn new() -> Passport {
        Passport {fields: HashMap::new()}
    }

    fn from_group(group: &Vec<String>) -> Passport {
        let mut pp = Passport::new();
        for line in group {pp.add_line(line);}
        pp
    }

    /// Create a vector of Passports from an input file.
    fn read(lines: &Vec<String>) -> Vec<Passport> {
        // Parse strings into delimited groups.
        let groups = common::group_strings(lines);
        // Create a passport for each group.
        groups.iter().map(Passport::from_group).collect()
    }    

    /// Add one line of key-value pairs to a passport.
    fn add_line(&mut self, line: &str) {
        // Split on spaces to get key:value pairs.
        for kv_str in line.split(' ') {
            let kv:Vec<&str> = kv_str.split(':').collect();
            if kv.len() == 2 {
                self.fields.insert(String::from(kv[0]), String::from(kv[1]));
            } else {
                eprintln!("Bad key/value: {}", kv_str);
            }
        }
    }

    /// Test if this passport has all required fields.
    fn is_valid(&self) -> bool {
        return self.fields.contains_key("byr")
            && self.fields.contains_key("iyr")
            && self.fields.contains_key("eyr")
            && self.fields.contains_key("hgt")
            && self.fields.contains_key("hcl")
            && self.fields.contains_key("ecl")
            && self.fields.contains_key("pid");
    }

    /// Also check the range on selected fields.
    fn really_valid(&self) -> bool {
        // Check all fields are present before we start.
        if !self.is_valid() {return false;}

        // Otherwise, extract and check each field:
        let byr = self.fields.get("byr").unwrap().parse::<i32>();
        if let Ok(byr) = byr {  // Integer parse OK
            if byr < 1920 || byr > 2002 {return false;}
        } else {return false;}  // Integer parse failed

        let iyr = self.fields.get("iyr").unwrap().parse::<i32>();
        if let Ok(iyr) = iyr {  // Integer parse OK
            if iyr < 2010 || iyr > 2020 {return false;}
        } else {return false;}  // Integer parse failed

        let eyr = self.fields.get("eyr").unwrap().parse::<i32>();
        if let Ok(eyr) = eyr {  // Integer parse OK
            if eyr < 2020 || eyr > 2030 {return false;}
        } else {return false;}  // Integer parse failed

        let hgt = self.fields.get("hgt").unwrap();
        if hgt.ends_with("cm") {
            let num = height_helper(&hgt);
            if num < 150 || num > 193 {return false;}
        } else if hgt.ends_with("in") {
            let num = height_helper(&hgt);
            if num < 59 || num > 76 {return false;}
        } else {return false;}

        let hcl = self.fields.get("hcl").unwrap();
        if hcl.len() == 7 {
            let mut cc = hcl.chars();
            if cc.next() != Some('#') {return false;}
            if !cc.all(valid_hex) {return false;}
        } else {return false;}

        let ecl_allowed = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        let ecl:&str = self.fields.get("ecl").unwrap();
        if !ecl_allowed.contains(&ecl) {return false;}

        let pid = self.fields.get("pid").unwrap();
        if (pid.len() != 9) || (!pid.chars().all(char::is_numeric)) {return false;}

        // Passed all tests.
        true
    }
}

/// Helper function for parsing numeric portion of height.
fn height_helper(x: &String) -> i32 {
    let y = x[0..x.len()-2].parse::<i32>();
    if let Ok(z) = y {z} else {0}
}

/// Check if a character is a hexadecimal digit.
fn valid_hex(c: char) -> bool {
    return ('0' <= c && c <= '9')
        || ('a' <= c && c <= 'f')
        || ('A' <= c && c <= 'F');
}

/// Count number of valid passports (containing all fields).
fn count_valid(vec: &Vec<Passport>) -> usize {
    common::count_true(vec.iter().map(|x| x.is_valid()))
}

/// Count number of valid passports (containing all fields AND valid data).
fn count_really_valid(vec: &Vec<Passport>) -> usize {
    common::count_true(vec.iter().map(|x| x.really_valid()))
}

pub fn solve() {
    // Define the test strings from the problem statement.
    let test1_str = vec![
        String::from("ecl:gry pid:860033327 eyr:2020 hcl:#fffffd"),
        String::from("byr:1937 iyr:2017 cid:147 hgt:183cm"),
        String::from(""),
        String::from("iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884"),
        String::from("hcl:#cfa07d byr:1929"),
        String::from(""),
        String::from("hcl:#ae17e1 iyr:2013"),
        String::from("eyr:2024"),
        String::from("ecl:brn pid:760753108 byr:1931"),
        String::from("hgt:179cm"),
        String::from(""),
        String::from("hcl:#cfa07d eyr:2025 pid:166559648"),
        String::from("iyr:2011 ecl:brn hgt:59in"),
    ];
    let test2_str = vec![
        String::from("eyr:1972 cid:100"),
        String::from("hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926"),
        String::from(""),
        String::from("iyr:2019"),
        String::from("hcl:#602927 eyr:1967 hgt:170cm"),
        String::from("ecl:grn pid:012533040 byr:1946"),
        String::from(""),
        String::from("hcl:dab227 iyr:2012"),
        String::from("ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277"),
        String::from(""),
        String::from("hgt:59cm ecl:zzz"),
        String::from("eyr:2038 hcl:74454a iyr:2023"),
        String::from("pid:3556412378 byr:2007"),
    ];
    let test3_str = vec![
        String::from("pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980"),
        String::from("hcl:#623a2f"),
        String::from(""),
        String::from("eyr:2029 ecl:blu cid:129 byr:1989"),
        String::from("iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm"),
        String::from(""),
        String::from("hcl:#888785"),
        String::from("hgt:164cm byr:2001 iyr:2015 cid:88"),
        String::from("pid:545766238 ecl:hzl"),
        String::from("eyr:2022"),
        String::from(""),
        String::from("iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719"),
    ];

    // Parse the example and check results.
    let test1 = Passport::read(&test1_str);
    let test2 = Passport::read(&test2_str);
    let test3 = Passport::read(&test3_str);
    assert_eq!(2, count_valid(&test1));
    assert_eq!(2, count_really_valid(&test1));
    assert_eq!(0, count_really_valid(&test2));
    assert_eq!(4, count_really_valid(&test3));

    // Load and analyze the main input.
    let input = Passport::read(&common::read_strings("input/input04.txt"));
    println!("Part1: {} valid passports.", count_valid(&input));
    println!("Part2: {} valid passports.", count_really_valid(&input));
}
