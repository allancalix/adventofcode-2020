use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let expense: Vec<i32> = content.lines().map(|line| { line.parse::<i32>().unwrap() }).collect();

    for (i, value) in expense.iter().enumerate() {
        for (j, check_value) in expense.iter().enumerate() {
            if i == j {
                continue;
            }

            if value + check_value == 2020 {
                println!("{}", value * check_value);
                return;
            }
        }
    }
}

