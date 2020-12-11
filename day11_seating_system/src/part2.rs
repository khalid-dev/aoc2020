use std::fs;

#[path = "./utils.rs"]
mod utils;
use utils::{ EMPTY, OCCUPIED, apply_changes };

// TODO: make struct for matrix and put these methods on it

fn is_point_in_bounds(seating_matrix: &Vec<Vec<char>>, point: (isize, isize)) -> bool {
    let (row, col) = point;
    let vec_row_len = seating_matrix.len() as isize;
    let vec_col_len = seating_matrix[0].len() as isize;
    return row >= 0 && row < vec_row_len && col >= 0 && col < vec_col_len;
}

fn find_unoccupied_directional(
    seating_matrix: &Vec<Vec<char>>,
    seat: (isize, isize),
    slope: (isize, isize)) -> Option<(usize, usize)> {
    let mut search_ptr = seat;

    'search: while is_point_in_bounds(seating_matrix, search_ptr) {
        // skip current seat
        if search_ptr != seat {
            let row = search_ptr.0 as usize;
            let col = search_ptr.1 as usize;
            match seating_matrix[row][col] {
                OCCUPIED | EMPTY => break 'search,
                _ => ()
            }
        }
        search_ptr.0 += slope.0;
        search_ptr.1 += slope.1;
    }

    if is_point_in_bounds(seating_matrix, search_ptr) {
        let row = search_ptr.0 as usize;
        let col = search_ptr.1 as usize;
        match seating_matrix[row][col] {
            OCCUPIED => Some((row, col)),
            _ => None
        }
    } else {
        None
    }
}

fn get_occupied_adjacencies(seating_matrix: &Vec<Vec<char>>, seat: (isize, isize)) -> Vec<(usize, usize)> {
    let mut adjacencies: Vec<(usize, usize)> = Vec::new();
    let (row, col) = seat;

    for i in row-1..=row+1 {
        for j in col-1..=col+1 {
            let slope = (i - row, j - col);
            // skip slope corresponding to current seat, same reasoning as p1
            if slope != (0, 0) {
                match find_unoccupied_directional(&seating_matrix, seat, slope) {
                    Some((row, col)) => adjacencies.push((row, col)),
                    None => ()
                }
            }
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
                    let occupied_adjacencies = get_occupied_adjacencies(seating_matrix, (row as isize, col as isize));
                    match occupied_adjacencies.len() {
                        0 => seat_changes.push((row, col, OCCUPIED)),
                        _ => ()
                    }
                },
                OCCUPIED => {
                    let occupied_adjacencies = get_occupied_adjacencies(seating_matrix, (row as isize, col as isize));
                    match occupied_adjacencies.len() {
                        0..=4 => (),
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
