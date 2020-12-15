use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = "2,0,6,12,1,3";

fn main() {
    let mut turn_seen: HashMap<u64, (usize, usize)> = HashMap::new();

    for (i, entry) in INPUT.split(',').enumerate() {
        let num = entry.parse::<u64>().unwrap();
        turn_seen.insert(num, (i + 1, i + 1));
    }

    let mut most_recent_num: u64 = INPUT.split(',').last().unwrap().parse::<u64>().unwrap();

    let loop_start = Instant::now();
    // 2020 for p1, 30000000 for p2
    for t in &turn_seen.len() + 1..=30000000 {
        match turn_seen.get_mut(&most_recent_num) {
            // these variables could be better, but so could the premise of the problem
            Some((last_spoken, last_last_spoken)) => {
                // if the most recent number said is its first occurrence
                if *last_spoken == *last_last_spoken {
                    most_recent_num = 0;
                    // 0's already in my given input, if it wasn't this would need to be branching
                    let (last_spoken, last_last_spoken) = turn_seen.get_mut(&most_recent_num).unwrap();
                    *last_last_spoken = *last_spoken;
                    *last_spoken = t;

                } else {
                    most_recent_num = (*last_spoken - *last_last_spoken) as u64;
                    match turn_seen.get_mut(&most_recent_num) {
                        Some((last_spoken, last_last_spoken)) => {
                            *last_last_spoken = *last_spoken;
                            *last_spoken = t;
                        },
                        None => {
                            turn_seen.insert(most_recent_num, (t, t));
                        }
                    }
                }
            },
            // every number encountered should have already been inserted into table
            None => ()
        }
    }
    let loop_end = Instant::now();
    
    println!("loop execution time: {:?}", loop_end.duration_since(loop_start));
    
    println!("ans: {}", most_recent_num);
}
