use std::path::Path;
use std::fs;
use std::collections::HashMap;
use std::cmp::Ordering;

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    println!("In file {}", INPUT_FILENAME);

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");
    
    let entries: Vec<u16> = contents.split_whitespace().map(|entry| entry.parse::<u16>().unwrap()).collect();
    println!("entries: {:?}", entries);

    let mut entry_compliments: HashMap<u16, bool> = HashMap::new();

    for entry in entries.iter() {
        let entry_compliment = 2020 - entry;
        // println!("parsed_entry: {}, entry_compliment: {}", parsed_entry, entry_compliment);
        // println!("{:?}", entry_compliments.get(&entry_compliment));
        if entry_compliments.get(&entry) == Some(&true) {
            println!("Found entries, {} and {}, that sum to 2020", entry, entry_compliment);
            break;
        } else {
            entry_compliments.insert(entry_compliment, true);
        }
    }

    // PART DOS
    let mut sorted_entries = entries.to_vec();
    sorted_entries.sort();
    // println!("sorted entries: {:?}", sorted_entries);

    'find_sum: for i in 0..sorted_entries.len() - 2 {
        // println!("i: {}", i);
        let mut left_boundary = i + 1;
        let mut right_boundary = sorted_entries.len() - 1;
        
        while left_boundary < right_boundary {
            let curr = sorted_entries[i];
            let left = sorted_entries[left_boundary];
            let right = sorted_entries[right_boundary];
            let sum = curr + left + right;

            match sum.cmp(&2020) {
                Ordering::Less => {
                    left_boundary = left_boundary + 1;
                }
                Ordering::Greater => {
                    right_boundary = right_boundary - 1;
                }
                Ordering::Equal => {
                    println!("found 3 numbers that sum to 2020: {}, {}, {}", curr, left, right);
                    break 'find_sum;
                }
            }
        }
    }
}
