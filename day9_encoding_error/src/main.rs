use std::io::BufReader;
use std::io::prelude::*;
use std::fs;
use std::collections::HashMap;
use queues::*;
use std::cmp::Ordering;

fn main() {
    const INPUT_FILENAME: &str = "./src/input";

    let mut preamble_pairs: HashMap<i64, bool> = HashMap::new();
    let mut insert_order: Queue<i64> = queue![];
    let mut invalid_num: i64 = -1;

    let file = fs::File::open(INPUT_FILENAME)
        .expect("file not found");
    let reader = BufReader::new(file);


    // trying buffers
    'read_file_buf: for line in reader.lines() {
        let num = line.unwrap().parse::<i64>().unwrap();

        match insert_order.size() {
            // insert the first 25 entries to map & queue
            0..=24 => {
                preamble_pairs.insert(num, true);
                insert_order.add(num);
            },
            // check if entry is valid, pop oldest from queue and map, insert newest
            25 => {
                let mut num_passed_validation = false;
                for (key, _) in preamble_pairs.iter() {
                    let compliment = num - key;
                    match preamble_pairs.contains_key(&compliment) {
                        true => num_passed_validation = true,
                        false => (),
                    }
                }

                match num_passed_validation {
                    true => (),
                    false => {
                        invalid_num = num;
                        break 'read_file_buf;
                    }
                }

                let lru = insert_order.remove().unwrap();
                preamble_pairs.remove(&lru);

                insert_order.add(num);
                preamble_pairs.insert(num, true);
            },
            _ => println!("queue: {:?} has over 25 entries", insert_order),
        }
    }

    println!("num: {} failed XMAS validation", invalid_num);

    // Part 2:
    let contents = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");
    let nums: Vec<i64> = contents
        .trim_end()
        .split("\n")
        .map(|entry| entry.parse::<i64>().unwrap())
        .collect();

    let mut left_bound: usize = 0;
    let mut right_bound: usize = 1;
    let mut sum: i64 = nums[0] + nums[1];

    // find contiguous entries that sum to invalid num from Part 1
    'find_window: while right_bound < nums.len() - 1 {
        // if we encounter a number that would invalidate the entire window, skip window past it
        if nums[left_bound] > invalid_num || nums[right_bound] > invalid_num {
            while nums[left_bound] > invalid_num {
                left_bound += 1;
            }
            right_bound = left_bound + 1;
            sum = nums[left_bound] + nums[right_bound];
        }

        match sum.cmp(&invalid_num) {
            Ordering::Less => {
                right_bound += 1;
                sum += nums[right_bound];
            },
            Ordering::Greater => {
                sum -= nums[left_bound];
                left_bound +=1;
            },
            Ordering::Equal => {
                break 'find_window;
            }
        }
    }

    let mut min: i64 = i64::MAX;
    let mut max: i64 = i64::MIN;

    for i in left_bound..=right_bound {
        if nums[i] < min {
            min = nums[i];
        }
        if nums[i] > max {
            max = nums[i];
        }
    }

    println!("sum min max: {}", min + max);
}
