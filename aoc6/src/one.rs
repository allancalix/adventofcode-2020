use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

use regex::Regex;

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let group_re = Regex::new(r"\n\n\s*").unwrap();
    let mut solutions: Vec<HashSet<char>> = vec!();
    for group in group_re.split(&content) {
        let mut unique_ans = HashSet::new();
        for ans in group.split('\n') {
            for c in ans.chars() {
                unique_ans.insert(c);
            }
        }
        solutions.push(unique_ans);
    }

    println!("{}", solutions.iter().fold(0, |acc, x| { acc + x.len() }))
}
