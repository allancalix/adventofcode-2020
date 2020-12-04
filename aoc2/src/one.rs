use std::fs::File;
use std::io::prelude::*;

use regex::Regex;

#[derive(Debug)]
struct Policy {
    required_char: char,
    min: i32,
    max: i32,
}

impl Policy {
    pub fn apply(&self, password: &str) -> bool {
        let mut required_char_count = 0;
        for c in password.chars() {
            if c == self.required_char {
                required_char_count += 1;
            }
        }

        required_char_count >= self.min && required_char_count <= self.max
    }
}

fn parse_line(line: &str) -> (Policy, String) {
    let re = Regex::new(r"(\d+)-(\d+) (\w{1}): (\w+)").unwrap();
    let caps = re.captures(line).unwrap();

    let raw_min = caps.get(1).map_or("", |m| m.as_str());
    let raw_max = caps.get(2).map_or("", |m| m.as_str());
    let raw_required_char = caps.get(3).map_or("", |m| m.as_str());
    let password = caps.get(4).map_or("", |m| m.as_str());

    let policy = Policy {
        required_char: raw_required_char.chars().nth(0).unwrap(),
        min: raw_min.parse::<i32>().unwrap(),
        max: raw_max.parse::<i32>().unwrap(),
    };
    (policy, password.into())
}

fn main() {
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
