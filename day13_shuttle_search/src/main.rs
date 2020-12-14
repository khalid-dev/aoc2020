use std::fs;
use std::collections::HashMap;
use std::time::Instant;

fn does_tick_satisfy_rules(tick: i64, rules: &HashMap<i64, (i64, bool)>) -> bool {
    return rules.iter()
        .all(|(freq, (offset, _used))| (tick + offset) % freq == 0);
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";
    let contents = fs::read_to_string(INPUT_FILENAME)
        .expect("err when reading file to string");
    let lines: Vec<&str> = contents
        .trim_end()
        .split('\n')
        .collect();

    let earliest_departure_time: i64 = lines[0].parse::<i64>().unwrap();
    let shuttle_frequencies: Vec<i64> = lines[1]
        .split(',')
        .filter(|x| x != &"x")
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let earliest_bus = shuttle_frequencies.iter()
        .min_by_key(|x| earliest_departure_time / **x)
        .unwrap();

    let departure_time: i64 = (earliest_bus * (earliest_departure_time/earliest_bus)) + earliest_bus;
    let ans: i64 = (departure_time - earliest_departure_time) * earliest_bus;

    println!("ans: {}", ans);

    // part 2: failed to discover ancient Sun-tzu Suan-Ching theorems on my own with 1 number theory class
    // under my belt, doing it the 'naive' way with GIGA SMART optimizations to narrow down search range.
    // Search in Range Iterations:
    // 1) Use first bus freq as step, too long
    // 2) Use largest bus freq as step, too long
    // 3) Generalize refactor b/t #1 & #2 to adjust step when a 'mini' convergence is found between
    //    two bus cycles, ~247 µs
    let mut rules: HashMap<i64, (i64, bool)> = HashMap::new();

    for (i, entry) in lines[1].split(',').enumerate() {
        if entry != "x" {
            let parsed_num: i64 = entry.parse().unwrap();
            rules.insert(parsed_num, (i as i64, false));
        }
    }

    let lowest_freq_bus = shuttle_frequencies.iter().max().unwrap();
    let mut step = *lowest_freq_bus;
    let mut tick: i64 = 0 - rules.get(lowest_freq_bus).unwrap().0;


    // set used for this frequency to true b/c we have 'incorporated' it into our step
    rules.get_mut(lowest_freq_bus).unwrap().1 = true;

    let loop_start = Instant::now();

    loop {
        if does_tick_satisfy_rules(tick, &rules) {
            break
        } else {
            for (freq, (offset, used)) in rules.iter_mut() {
                if (tick + *offset) % freq == 0  && !*used {
                    *used = true;
                    // @mpaul418 (https://github.com/mpaul418/) explained to me that this adjustment only works b/c bus
                    // frequencies in input are prime; otherwise we would have to do some GCD stuff
                    // most likely.
                    step *= freq;
                }
            }
            tick += step;
        }
    }

    let loop_end = Instant::now();

    println!("first valid tick: {}", tick);
    println!("loop execution time: {:?}", loop_end.duration_since(loop_start));
}
