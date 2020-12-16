// Okay, today was very challenging implementation wise, but I'm happy I got the approach
// pretty quickly.
// Didn't have the time to polish it as much as I would have liked :(
// But I learned a lot more about working around ownership today :)
#![feature(str_split_once)]
use std::fs;
use regex::Regex;
use std::collections::HashMap;

const INPUT_FILENAME: &str = "./src/input";

fn is_invalid_ticket(ticket: &str, valid_ranges: &Vec<(String, (u16, u16), (u16, u16))>) -> Option<u16> {
    for value in ticket.split(',') {
        let val_num: u16 = value.parse::<u16>().unwrap();
        // l_s = lower range start, u_e = upper range end, etc.
        if valid_ranges.iter()
            // 1) value is less than the start of the lower valid range
            // 2) value is between the lower range end and upper range start
            // 3) value is greater than the end of the upper valid range
            .all(|(_field, (l_s, l_e), (u_s, u_e))| val_num < *l_s || (val_num > *l_e && val_num < *u_s) || val_num > *u_e) {
                return Some(val_num);
        }
    }
    return None;
}

fn main() {
    let input_str = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");

    // [0] = field rules, [1] = your ticket, [2] = nearby tickets
    let input_groups: Vec<&str> = input_str
        .trim_end()
        .split("\n\n")
        .collect();

    let range_re = Regex::new(r"(.+):\s(\d+)-(\d+)\sor\s(\d+)-(\d+)").unwrap();

    let valid_ranges: Vec<(String, (u16, u16), (u16, u16))> = input_groups[0]
        .split('\n')
        .map(|line| {
            match range_re.captures(line) {
                Some(caps) => (
                    caps[1].to_owned(),
                    (caps[2].parse::<u16>().unwrap(), caps[3].parse::<u16>().unwrap()),
                    (caps[4].parse::<u16>().unwrap(), caps[5].parse::<u16>().unwrap())
                    ),
                None => panic!("range regex failed at line: {}", line)
            }
        })
        .collect();

    let mut invalid_value_sum: u16 = 0;

    for ticket in input_groups[2].strip_prefix("nearby tickets:\n").unwrap().split('\n') {
        match is_invalid_ticket(ticket, &valid_ranges) {
            Some(val) => invalid_value_sum += val,
            None => (),
        }
    }

    println!("invalid_value_sum: {}", invalid_value_sum);

    // Part 2:
    // 1) Filter out all invalid tickets from P1
    // 2) HashMap<field_rule, Vec<usize>>
    // 3) Iterate over col's of tickets, if all entries in a col fulfill a field rule, push col
    // 4) for every rule that is only fulfilled by 1 col, remove that col from all the other rules
    //    it fulfills. this helps narrow down rules that are fulfilled (and thus correspond to)
    //    multiple cols.
    // 5) By virtue of an answer existing, this should be sufficient & conclusive.
    // 6) Do the straight forward part of the problem.

    // 1
    let valid_tickets: Vec<&str> = input_groups[2]
        .strip_prefix("nearby tickets:\n")
        .unwrap()
        .split('\n')
        .filter(|ticket| match is_invalid_ticket(ticket, &valid_ranges) {
            Some(_val) => false,
            None => true
        })
        .collect();

    // 2
    let mut field_possibilities: HashMap<String, Vec<usize>> = HashMap::new();
    for field in input_groups[0].split('\n') {
        let parsed_field = field.split_once(':').unwrap().0;
        field_possibilities.insert(parsed_field.to_string(), Vec::new());
    }

    // 3, assuming all valid tickets have the same # of fields..., could have preprocessed this
    for (col, _) in valid_tickets[0].split(',').enumerate() {
        for (field, (l_s, l_e), (u_s, u_e)) in valid_ranges.iter() {
            if valid_tickets.iter().all(|ticket| {
                let val = ticket.split(',').collect::<Vec<&str>>()[col].parse::<u16>().unwrap();
                return (val >= *l_s && val <= *l_e) || (val >= *u_s && val <= *u_e);
            }) {
                match field_possibilities.get_mut(field) {
                    Some(v) => v.push(col),
                    None => { field_possibilities.insert(field.to_string(), vec![col]); }
                }
            }
        }
    }
    
    let mut fields_have_multiple_possibilities = true;

    // 4 & 5
    // The two for loops were originally nested as that's how I'd approach this in JS,
    // but that caused a lot of ownership issues in Rust, so I used a 'processing' vector
    // to split up the logic of getting all the potential changes and applying them.
    while fields_have_multiple_possibilities {
        let mut single_possibilities: Vec<(String, usize)> = Vec::new();

        for (k, v) in field_possibilities.iter() {
            if v.len() == 1 {
                single_possibilities.push((k.to_string(), v[0]));
            }
        }

        for (k, v) in field_possibilities.iter_mut() {
            *v = v.iter()
                .filter(|e| match single_possibilities.iter().find(|(sub_k, x)| x == *e && sub_k != k) {
                    Some(_) => false,
                    None => true
                })
                .map(|e| *e).collect();
        }

        fields_have_multiple_possibilities = field_possibilities.values()
            .any(|v| v.len() > 1);
    }

    // 6
    let my_ticket: Vec<u64> = input_groups[1]
        .strip_prefix("your ticket:\n").unwrap()
        .split(',')
        .map(|e| e.parse::<u64>().unwrap())
        .collect();
    let mut ans: u64 = 1;
    for (k, v) in field_possibilities.iter() {
        match k.strip_prefix("departure") {
            Some(_) => ans *= my_ticket[v[0]],
            None => ()
        }
    }
    println!("ans: {}", ans);
}
