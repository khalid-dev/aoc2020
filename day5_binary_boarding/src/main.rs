use std::path::Path;
use std::fs;
use std::cmp;

const FRONT: char = 'F';
const BACK: char = 'B';
const LEFT: char = 'L';
const RIGHT: char = 'R';

fn get_seat_id(boarding_pass_str: &str) -> u16 {
    let mut row_lower_bound: u8 = 0;
    let mut row_upper_bound: u8 = 127;
    let mut col_left_bound: u8 = 0;
    let mut col_right_bound: u8 = 7;

    for letter in boarding_pass_str.chars() {
        match letter {
            FRONT => {
                let sum = row_lower_bound + row_upper_bound;
                row_upper_bound = sum / 2;
            },
            BACK => {
                let sum = row_lower_bound + row_upper_bound;
                row_lower_bound = (sum / 2) + (sum % 2);
            },
            LEFT => {
                let sum = col_left_bound + col_right_bound;
                col_right_bound = sum / 2;
            },
            RIGHT => {
                let sum = col_left_bound + col_right_bound;
                col_left_bound = (sum / 2) + (sum % 2);
            },
            _ => panic!("found bad letter {} in boarding pass {}", letter, boarding_pass_str)
        }
    }

    let row = row_lower_bound as u16;
    let col = col_left_bound as u16;
    return row * 8 + col;
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading input file");

    let boarding_passes: Vec<&str> = contents
        .trim_end()
        .split('\n')
        .collect();

    let max_seat_id = boarding_passes.iter()
        .fold(0, |max, boarding_pass_str| cmp::max(max, get_seat_id(boarding_pass_str)));

    println!("highest seat id: {}", max_seat_id);

    //Part 2:

    let mut seat_ids: Vec<u16> = boarding_passes.iter()
        .map(|boarding_pass_str| get_seat_id(boarding_pass_str))
        .collect();
    seat_ids.sort();

    for i in 1..seat_ids.len() - 2 {
        let seat_id = seat_ids[i];
        let prev_seat_id = seat_ids[i - 1];
        let expected_prev = seat_id - 1;

        if prev_seat_id != expected_prev {
            println!("missing pass: {}", expected_prev);
        }
    }
    
    // Alt approach: instantiate array of appropriate size,
    // map over seat_id arr, insert entries into array via offset index,
    // then find the entry in array that is 0,
    // then return the index of that entry + offset

    let mut fixed_seat_ids = vec![0; boarding_passes.len() + 1];
    let offset = max_seat_id as usize - fixed_seat_ids.len() + 1;
    for pass in boarding_passes {
        let seat_id = get_seat_id(pass);
        let dest = seat_id as usize - offset;
        fixed_seat_ids[dest] = seat_id;
    }
    let missing_seat_ix = fixed_seat_ids.iter().position(|&x| x == 0);
    let missing_seat_id = missing_seat_ix.unwrap() + offset;
    println!("missing seat id: {}", missing_seat_id);
}
