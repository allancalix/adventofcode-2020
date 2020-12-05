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
        let hex_re = Regex::new(r"#[0-9a-f]{6}$").unwrap();
        let pid_re = Regex::new(r"[0-9]{9}").unwrap();

        if !valid_year(&self.birth_year, 1920, 2002) {
            println!("birth_year");
        }

        if !valid_year(&self.issue_year, 2010, 2020) {
            println!("issue_year");
        }

        if !valid_year(&self.expiration_year, 2020, 2030) {
            println!("expiration_year");
        }

        if !valid_height(&self.height) {
            println!("height");
        }

        if !hex_re.is_match(&self.hair_color) {
            println!("hair_color");
        }

        if !valid_eye_color(&self.eye_color) {
            println!("eye_color");
        }

        if self.passport_id.len() != 9 || !pid_re.is_match(&self.passport_id) {
            println!("pass_id");
        }

        valid_year(&self.birth_year, 1920, 2002) &&
            valid_year(&self.issue_year, 2010, 2020) &&
            valid_year(&self.expiration_year, 2020, 2030) &&
            valid_height(&self.height) &&
            hex_re.is_match(&self.hair_color) &&
            valid_eye_color(&self.eye_color) &&
            self.passport_id.len() == 9 && pid_re.is_match(&self.passport_id)
    }
}

fn valid_eye_color(raw: &str) -> bool {
    match raw {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn valid_height(raw: &str) -> bool {
    if !raw.ends_with("cm") && !raw.ends_with("in") {
        return false;
    }

    let (value, metric) = raw.split_at(raw.len() - 2);

    match metric {
        "cm" => {
            let h = value.parse::<i32>().unwrap();
            if h < 150 || h > 193 {
                return false;
            }
        },
        "in" => {
            let h = value.parse::<i32>().unwrap();
            if h < 59 || h > 76 {
                return false;
            }
        },
        _ => panic!("invalid metric value"),
    }

    true
}

fn valid_year(raw: &str, min: i32, max: i32) -> bool {
    if raw.len() != 4 {
        return false;
    }

    let year = raw.parse::<i32>().unwrap();
    if year < min || year > max {
        return false;
    }

    true
}

fn count_valid(batch: &str) -> i32 {
    let mut valid_count = 0;
    let re = Regex::new(r"\n\n\s*").unwrap();
    for passport in re.split(&batch) {
        let p = Passport::parse(passport);

        if p.is_valid() {
            valid_count += 1;
        } else {
            println!("{:?}", p);
        }
    }
    return valid_count;
}

fn main() {
    let mut f = File::open("data/one.txt").unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    println!("{}", count_valid(&content));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INVALID: &str = r#"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007
"#;

    const VALID: &str = r#"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

pid:858541281 hgt:150cm eyr:2021
iyr:2013 ecl:gry
hcl:#fffffd byr:1959
"#;

    #[test]
    fn invalid_entries() {
        assert_eq!(count_valid(INVALID), 0);
    }

    #[test]
    fn valid_entries() {
        assert_eq!(count_valid(VALID), 4);
    }
}
