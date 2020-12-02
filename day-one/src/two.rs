use std::fs::File;
use std::io::prelude::*;

const MONEY_VALUE: i32 = 2020;

fn main() {
    let mut f = File::open("data/two.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    let mut expense: Vec<i32> = content.lines().map(|line| { line.parse::<i32>().unwrap() }).collect();
    expense.sort();

    for (i, value) in expense.iter().enumerate() {
        for (j, check_value) in expense.iter().enumerate() {
            for (k, inner_value) in expense.iter().enumerate() {
                if i == j || j == k || i == k {
                    continue;
                }

                if value + check_value + inner_value == MONEY_VALUE {
                    println!("{}", value * check_value * inner_value);
                    return;
                }
            }
        }
    }
}

