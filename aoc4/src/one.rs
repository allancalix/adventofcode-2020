use regex::Regex;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Passport {
    birth_year: String,
    issue_year: String,
    expiration_year: String,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: String,
}

impl Passport {
    fn new() -> Self {
        Self {
            birth_year: String::new(),
            issue_year: String::new(),
            expiration_year: String::new(),
            height: String::new(),
            hair_color: String::new(),
            eye_color: String::new(),
            passport_id: String::new(),
            country_id: String::new(),
        }
    }

    pub fn parse(encoded_pass: &str) -> Self {
        let mut p = Passport::new();
        let re = Regex::new(r"(\n|\s)").unwrap();

        for kv in re.split(encoded_pass).filter(|pass| pass != &"") {
            let values: Vec<&str> = kv.split(':').collect();
            let key = values[0];
            let value = values[1];

            match key {
                "byr" => p.birth_year = value.into(),
                "iyr" => p.issue_year = value.into(),
                "eyr" => p.expiration_year = value.into(),
                "hgt" => p.height = value.into(),
                "hcl" => p.hair_color = value.into(),
                "ecl" => p.eye_color = value.into(),
                "pid" => p.passport_id = value.into(),
                "cid" => p.country_id = value.into(),
                _ => panic!("unrecognized passport key"),
            }
        }

        p
    }

    pub fn is_valid(&self) -> bool {
        if self.birth_year == ""
            || self.issue_year == ""
            || self.expiration_year == ""
            || self.height == ""
            || self.hair_color == ""
            || self.eye_color == ""
            || self.passport_id == "" {
            return false;
        }

        true
    }
}

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    let mut total_count = 0;
    let mut valid_count = 0;
    let re = Regex::new(r"\n\n\s*").unwrap();
    for passport in re.split(&content) {
        let p = Passport::parse(passport);

        if p.is_valid() {
            valid_count += 1;
        }

        total_count += 1;
    }

    println!("Total: {}, Valid: {}", total_count, valid_count);
}
