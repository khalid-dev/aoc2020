use std::fs;

#[path = "./utils.rs"]
mod utils;
use utils::{ EMPTY, OCCUPIED, apply_changes };

const FLOOR: char = '.';

fn get_adjacencies(seating_matrix: &Vec<Vec<char>>, seat: (isize, isize)) -> [char; 9] {
    // Array represents the 9 seats adjacent to the current one (diagonals count) & the curr seat.
    // [Upper Left (UL), Upper (U), Upper Right (UR), L, Curr, R, BL, B, BR]
    let mut adjacencies: [char; 9] = [FLOOR; 9];
    let mut adjacency_ix: usize = 0;

    let (row, col) = seat;
    let vec_row_len = seating_matrix.len() as isize;
    let vec_col_len = seating_matrix[0].len() as isize;

    for i in row-1..=row+1 {
        for j in col-1..=col+1 {
            let is_curr_seat = i == row && j == col;
            let adjacency_in_bounds = i >= 0 && i < vec_row_len && j >= 0 && j < vec_col_len;
            // skip current seat
            if !is_curr_seat && adjacency_in_bounds {
                match seating_matrix[i as usize][j as usize] {
                    EMPTY => adjacencies[adjacency_ix] = EMPTY,
                    OCCUPIED => adjacencies[adjacency_ix] = OCCUPIED,
                    _ => ()
                }
            }
            adjacency_ix += 1;
        }
    }

    return adjacencies;
}

fn get_seat_changes(seating_matrix: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let mut seat_changes: Vec<(usize, usize, char)> = Vec::new();

    for (row, _) in seating_matrix.iter().enumerate() {
        for (col, _) in seating_matrix[row].iter().enumerate() {
            let seat = seating_matrix[row][col];
            match seat {
                EMPTY => {
                    let adjacencies = get_adjacencies(seating_matrix, (row as isize, col as isize));
                    match adjacencies.iter().find(|c| c == &&OCCUPIED) {
                        Some(_) => (),
                        None => seat_changes.push((row, col, OCCUPIED))
                    }
                },
                OCCUPIED => {
                    let adjacencies = get_adjacencies(seating_matrix, (row as isize, col as isize));
                    let occupied_adjacency_count = adjacencies.iter().filter(|x| x == &&OCCUPIED).collect::<Vec<&char>>().len();
                    match occupied_adjacency_count {
                        0..=3 => (),
                        _ => seat_changes.push((row, col, EMPTY)),
                    }
                },
                _ => ()
            }
        }
    }

    return seat_changes;
}

pub fn get_occupied_count() {
    const INPUT_FILENAME: &str = "./src/input";
    let contents = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");
    let mut seating_matrix: Vec<Vec<char>> = contents
        .trim_end()
        .split('\n')
        .map(|string| string.chars().collect())
        .collect();

    let mut seat_changes = get_seat_changes(&seating_matrix);
    let mut num_seat_changes = seat_changes.len();
    while num_seat_changes > 0 {
        apply_changes(&mut seating_matrix, seat_changes);
        seat_changes = get_seat_changes(&seating_matrix);
        num_seat_changes = seat_changes.len();
    }

    let final_occupied_count = seating_matrix.iter().fold(
        0, |acc, row| acc + row.iter().fold(
            0, |acc, x| if *x == OCCUPIED { acc + 1 } else { acc }
        )
    );

    println!("final_occupied_count: {}", final_occupied_count);
}
