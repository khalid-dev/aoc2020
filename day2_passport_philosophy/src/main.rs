use std::path::Path;
use std::fs;
use regex::Regex;

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    println!("In file {}", INPUT_FILENAME);

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    let mut entries: Vec<&str> = contents.split('\n').collect();
    // omegalaughing
    entries.pop();

    let mut valid_passwords: u16 = 0;
    let mut super_valid_passwords: u16 = 0;

    let re = Regex::new(r"(\d+)-(\d+)\s([a-z]):\s(.*)").unwrap();

    for entry in entries.iter() {
        let caps = re.captures(entry).unwrap();
        let min: u8 = caps[1].parse::<u8>().unwrap();
        let max: u8 = caps[2].parse::<u8>().unwrap();
        let letter: char = caps[3].parse::<char>().unwrap();
        let password = &caps[4];
        
        let pw_regex = format!(r"^(?:[^{}]*{}[^{}]*){{{},{}}}$", letter, letter, letter, min, max);
        let valid_pattern = Regex::new(&pw_regex).unwrap();
        if valid_pattern.is_match(&password) {
            valid_passwords += 1;
        } 

        //PART 2
        let char_a = password.chars().nth((min - 1).into()).unwrap();
        let char_b = password.chars().nth((max - 1).into()).unwrap();

        if (char_a == letter) ^ (char_b == letter) {
            super_valid_passwords += 1;
        }
    }

    println!("valid pw count: {}", valid_passwords);
    println!("super valid pw count: {}", super_valid_passwords);
}
