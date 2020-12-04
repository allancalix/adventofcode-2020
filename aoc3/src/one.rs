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

fn main() {
    // Uses the same input file as the first part of the challenge.
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut tree_collisions = 0;
    let mut y_axis = 0 as usize;
    for line in content.lines() {
        let line = parse_line(line);
        let i = y_axis % line.len();

        if line[i] == State::Tree {
            tree_collisions += 1;
        }

        y_axis += 3;
    }

    println!("{}", tree_collisions);
}
