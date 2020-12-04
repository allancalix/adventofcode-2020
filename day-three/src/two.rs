
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
enum State {
    Tree,
    NoTree,
}

fn parse_line(line: &str) -> Vec<State> {
    line.chars().map(|c| {
        match c {
            '.' => State::NoTree,
            '#' => State::Tree,
            _ => {
                panic!("Unrecognized character.");
            },
        }
    }).collect()
}

// Accepts a slope and returns the number of tree collisions on that slope path.
fn check_slope(y_offset: usize, x_offset: usize, slope: &Vec<Vec<State>>) -> i64 {
    let mut tree_collisions = 0;
    let mut y_axis = 0 as usize;
    for level in slope.iter().step_by(x_offset) {
        let i = y_axis % level.len();

        if level[i] == State::Tree {
            tree_collisions += 1;
        }

        y_axis += y_offset;
    }
    tree_collisions
}

fn main() {
    // Uses the same input file as the first part of the challenge.
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let slope = content.lines().map(|line| parse_line(line)).collect();

    let p1 = check_slope(1, 1, &slope);
    let p2 = check_slope(3, 1, &slope);
    let p3 = check_slope(5, 1, &slope);
    let p4 = check_slope(7, 1, &slope);
    let p5 = check_slope(1, 2, &slope);
    println!("{}", p1 * p2 * p3 * p4 * p5);
    // println!("{}", p1 * p2 * p3 * p4 * p5);
}
