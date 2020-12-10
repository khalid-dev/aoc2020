use std::fs;

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    let contents = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");
    let mut joltages: Vec<u8> = contents
        .trim_end()
        .split('\n')
        .map(|e| e.parse::<u8>().unwrap())
        .collect();

    // add the outlet, whose joltage = 0
    joltages.push(0);
    joltages.sort();
    // add the device, whose joltage = highest joltage + 3
    joltages.push(joltages.last().unwrap() + 3);

    let mut one_jolt_diff_count = 0;
    let mut three_jolt_diff_count = 0;

    for i in 1..joltages.len() {
        let joltage_difference = joltages[i] - joltages[i - 1];
        match joltage_difference {
            1 => one_jolt_diff_count += 1,
            3 => three_jolt_diff_count += 1,
            _ => ()
        }
    }

    println!("one jolt differences: {}, three jolt differences: {}", one_jolt_diff_count, three_jolt_diff_count);

    // Part 2 attempt, something off in this DP implementation

    let mut distinct_arrangements_to_ix: Vec<u64> = Vec::with_capacity(joltages.len());
    for i in 1..joltages.len() - 1 {
        let curr = joltages[i];

        match i - 1 {
            0 => distinct_arrangements_to_ix.push(1),
            1..=2 => {
                if curr <= 3 { distinct_arrangements_to_ix.push(2); }
            },
            _ => {
                let mut distinct_arrangements: u64 = distinct_arrangements_to_ix[i - 2];
                for j in i - 4..i - 2 {
                    if (curr as u64 - distinct_arrangements_to_ix[j]) <= 3 {
                        distinct_arrangements += distinct_arrangements_to_ix[j];
                    }
                }
                distinct_arrangements_to_ix.push(distinct_arrangements);
            }
        }
    }
}
