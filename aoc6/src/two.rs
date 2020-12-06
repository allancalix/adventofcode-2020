use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

use regex::Regex;

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let group_re = Regex::new(r"\n\n\s*").unwrap();
    let mut solutions = vec!();
    for group in group_re.split(&content) {
        let mut unique_ans = HashMap::new();
        let mut group_member_count = 0;
        for ans in group.split('\n').filter(|split| split != &"") {
            for c in ans.chars() {
                let entry = unique_ans.entry(c).or_insert(0);
                *entry += 1;
            }
            group_member_count += 1;
        }

        let valid_answers = unique_ans.iter()
            .filter(|&(_, v)| *v == group_member_count)
            .count();
        solutions.push(valid_answers);
    }

    println!("{}", solutions.iter().fold(0, |acc, x| { acc + x }))
}
