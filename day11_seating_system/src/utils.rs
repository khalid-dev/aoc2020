pub const EMPTY: char = 'L';
pub const OCCUPIED: char = '#';

pub fn apply_changes(seating_matrix: &mut Vec<Vec<char>>, seat_changes: Vec<(usize, usize, char)>) {
    for change in seat_changes.iter() {
        let (row, col, character) = change;
        seating_matrix[*row][*col] = *character;
    }
}
