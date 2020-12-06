use std::path::Path;
use std::fs;
use regex::Regex;

fn is_valid_passport(passport_str: String) -> bool {
    let mut byr_exists = false;
    let mut iyr_exists = false;
    let mut eyr_exists = false;
    let mut hgt_exists = false;
    let mut hcl_exists = false;
    let mut ecl_exists = false;
    let mut pid_exists = false;

    let fields: Vec<&str> = passport_str.split(' ').collect();
    for field_str in fields {

        let field_tuple: Vec<&str> = field_str.split(':').collect();
        let field = field_tuple[0];

        match field {
            "byr" => byr_exists = true,
            "iyr" => iyr_exists = true,
            "eyr" => eyr_exists = true,
            "hgt" => hgt_exists = true,
            "hcl" => hcl_exists = true,
            "ecl" => ecl_exists = true,
            "pid" => pid_exists = true,
            _ => (),
        }
    }

    return byr_exists & iyr_exists & eyr_exists & hgt_exists & hcl_exists & ecl_exists & pid_exists;
}

//Part 2 validator:
fn is_super_valid_passport(
    passport_str: String,
    hgt_regex: &Regex,
    hcl_regex: &Regex,
    ecl_regex: &Regex,
    pid_regex: &Regex) -> bool {
    let mut byr_is_valid = false;
    let mut iyr_is_valid = false;
    let mut eyr_is_valid = false;
    let mut hgt_is_valid = false;
    let mut hcl_is_valid = false;
    let mut ecl_is_valid = false;
    let mut pid_is_valid = false;

    let fields: Vec<&str> = passport_str.split(' ').collect();
    for field_str in fields {

        let field_tuple: Vec<&str> = field_str.split(':').collect();
        let field = field_tuple[0];
        let value = field_tuple[1];

        match field {
            "byr" => {
                let byr = value.parse::<u16>().unwrap();
                if byr >= 1920 && byr <= 2002 {
                    byr_is_valid = true;
                }
            },
            "iyr" => {
                let iyr = value.parse::<u16>().unwrap();
                if iyr >= 2010 && iyr <= 2020 {
                    iyr_is_valid = true;
                }
            },
            "eyr" => {
                let eyr = value.parse::<u16>().unwrap();
                if eyr >= 2020 && eyr <= 2030 {
                    eyr_is_valid = true;
                }
            },
            "hgt" => {
                match hgt_regex.captures(value) {
                    Some(caps) => {
                        let measurement = caps[1].parse::<u16>().unwrap();
                        let unit = &caps[2];

                        if unit == "cm" {
                            if measurement >= 150 && measurement <= 193 {
                                hgt_is_valid = true;
                            }
                        }

                        if unit == "in" {
                            if measurement >= 59 && measurement <= 76 {
                                hgt_is_valid = true;
                            }
                        }
                    },
                    None => (),
                }
            },
            "hcl" => {
                match hcl_regex.captures(value) {
                    Some(caps) => {
                        if caps.len() == 2 {
                            hcl_is_valid = true
                        }
                    },
                    None => (),
                }
            },
            "ecl" => {
                match ecl_regex.captures(value) {
                    Some(caps) => {
                        let valid_colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                        if valid_colors.contains(&&caps[1]) {
                            ecl_is_valid = true;
                        }
                    },
                    None => (),
                }
            },
            "pid" => {
                match pid_regex.captures(value) {
                    // prefix unused vars with underscore
                    Some(_caps) => {
                        pid_is_valid = true
                    },
                    None => (),
                }
            }
            _ => (),
        }
    }

    return byr_is_valid & iyr_is_valid & eyr_is_valid & hgt_is_valid & hcl_is_valid & ecl_is_valid & pid_is_valid;
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    println!("In file {}", INPUT_FILENAME);

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    let passports: Vec<String> = contents
        .trim_end()
        .split("\n\n")
        .map(|passport| passport.replace('\n', " "))
        .collect();

    let valid_passport_count = passports.iter()
        .fold(0, |acc, passport| {
            if is_valid_passport(passport.to_string()) { acc + 1 } else { acc }
        });

    println!("valid passport count: {}", valid_passport_count);

    let hgt_regex = Regex::new(r"(\d+)(\D+)").unwrap();
    let hcl_regex = Regex::new(r"^#([a-f0-9]{6})$").unwrap();
    let ecl_regex = Regex::new(r"^(.{3})$").unwrap();
    let pid_regex = Regex::new(r"^(\d{9})$").unwrap();

    let super_valid_passport_count = passports.iter()
        .fold(0, |acc, passport| {
            if is_super_valid_passport(
                passport.to_string(),
                &hgt_regex,
                &hcl_regex,
                &ecl_regex,
                &pid_regex) { acc + 1 } else { acc }
        });

    println!("super valid passport count: {}", super_valid_passport_count);
}
