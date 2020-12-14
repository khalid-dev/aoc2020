use std::fs;
use std::collections::HashMap;

/** P1 Notes:
 * An 'operation group' is a bitmask and the attempted memory assignments below it.
 * 1) apply bitmask to every value in operation group before it is assigned to memory address
 * 2) assign value to memory address hashmap(key: address, val: bitmasked val)
 * 3) sum all values in memory hashmap
 * */
fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    let contents = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");
    let lines: Vec<&str> = contents
        .trim_end()
        .split('\n')
        .collect();

    let mut memory: HashMap<u64, u64> = HashMap::new();
    let mut mask: String = "".to_string();
    let mut zero_mask: String = "".to_string();

    for line in lines.iter() {
        let assignment: Vec<&str> = line
            .split('=')
            .map(|x| x.trim())
            .collect();
        let command = assignment[0];
        let value = &assignment[1];

        if command == "mask" {
            zero_mask = value
                .replace("1", "0")
                .replace("X", "1");
            mask = value
                .replace("X", "0");
        } else {
            let address: u64 = command.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse::<u64>()
                .unwrap();
            let mut masked_val = value.parse::<u64>().unwrap();

            let mask_val: u64 = u64::from_str_radix(&mask, 2).unwrap();
            let zero_mask_val: u64 = u64::from_str_radix(&zero_mask, 2).unwrap();
            masked_val &= zero_mask_val;
            masked_val |= mask_val;

            memory.insert(address, masked_val);
        }
    }

    let memory_val_sum = memory.values().fold(0, |acc, x| acc + x);

    println!("memory sum: {:?}", memory_val_sum);

    // TODO: P2 :)
}
