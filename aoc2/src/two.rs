use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Policy {
    required_char: char,
    first_pos: i32,
    second_pos: i32,
}

impl Policy {
    pub fn apply(&self, password: &str) -> bool {
        let first_match =  password
            .chars().nth((self.first_pos - 1) as usize)
            .unwrap_or('\0') == self.required_char;
        let second_match =  password
            .chars().nth((self.second_pos- 1) as usize)
            .unwrap_or('\0') == self.required_char;

        !(first_match == second_match)
    }
}

fn parse_line(line: &str) -> (Policy, String) {
    let re = Regex::new(r"(\d+)-(\d+) (\w{1}): (\w+)").unwrap();
    let caps = re.captures(line).unwrap();

    let raw_first_pos = caps.get(1).map_or("", |m| m.as_str());
    let raw_second_pos = caps.get(2).map_or("", |m| m.as_str());
    let raw_required_char = caps.get(3).map_or("", |m| m.as_str());
    let password = caps.get(4).map_or("", |m| m.as_str());

    let policy = Policy {
        required_char: raw_required_char.chars().nth(0).unwrap(),
        first_pos: raw_first_pos.parse::<i32>().unwrap(),
        second_pos: raw_second_pos.parse::<i32>().unwrap(),
    };

    (policy, password.into())
}

fn main() {
    // Uses the same input file as the first part of the challenge.
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut valid_count = 0;
    for line in content.lines() {
        let (policy, password) = parse_line(line);
        let is_valid = policy.apply(&password);
        if is_valid {
            valid_count += 1;
        }
    }

    println!("{}", valid_count);
}
