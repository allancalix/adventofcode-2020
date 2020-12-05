use std::fs::File;
use std::io::prelude::*;

const ROW_MAX: i32 = 127;

const COL_MAX: i32 = 7;

fn midpoint(min: i32, max: i32) -> i32 {
    let mut median = 0;
    for (i, idx) in (min..=max).enumerate() {
        if i as i32 == (max - min) / 2 {
            median = idx;
        }
    }

    median
}

struct BoardingPass {
    row: i32,
    col: i32,
    id: i32,
}

impl BoardingPass {
    fn parse(encoded: &str) -> Self {
        assert_eq!(encoded.len(), 10);

        let (row_enc, col_enc) = encoded.split_at(7);
        // Sanity check split behavior matches expectation.
        assert_eq!(row_enc.len(), 7);
        assert_eq!(col_enc.len(), 3);

        // FFBBBBB
        // 0-127
        // 0-63 - F
        // 0-31 - F
        // 15-31 - B
        // 23 - 31 - B
        // 27 - 31 - B
        // 29 - 31 - B
        let mut row_min = 0;
        let mut row_max = ROW_MAX;
        for r in row_enc.chars() {
            let median = midpoint(row_min, row_max);
            match r {
                'F' => {
                    row_max = median;
                },
                'B' => {
                    // Take the half above the median.
                    row_min = median + 1;
                }
                _ => panic!("unexpected encoding value"),
            }
        }
        assert!(row_max == row_min);
        let row = row_max;

        let mut col_min = 0;
        let mut col_max = COL_MAX;
        for r in col_enc.chars() {
            let median = midpoint(col_min, col_max);
            match r {
                'L' => {
                    col_max = median;
                },
                'R' => {
                    // Take the half above the median.
                    col_min = median + 1;
                },
                _ => panic!("unexpected encoding value"),
            }
        }
        assert!(col_max == col_min);
        let col = col_max;

        Self {
            id: row * 8 + col,
            row,
            col,
        }
    }
}

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut ids: Vec<i32> = content.lines().map(|line| BoardingPass::parse(line).id).collect();

    ids.sort();
    println!("Max: {}", ids.last().unwrap());

    let mut anomolyous = vec![];
    for (i, &id) in ids.iter().enumerate() {
        if i != 0 && i != ids.len() - 1 {
            if ids[i - 1] != id - 1 || ids[i + 1] != id + 1 {
                anomolyous.push(id);
            }
        }
    }
    assert_eq!(anomolyous.len(), 2);
    println!("Missing Seat ID: {}", midpoint(anomolyous[0], anomolyous[1]));
}
