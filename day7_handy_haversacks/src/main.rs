// Could have done better today; TODO: read more about Ownership in Rust

use std::fs;
use std::path::Path;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use queues::*;

fn parse_rules(rule_str: &str, value_re: &Regex, adjacencies_re: &Regex) -> (String, Vec<String>) {
    // two scenarios: str.find("no other")
    // if has, don't use regex, split on space, take first 2 words, return RuleNode
    // if doesn't have, use regex, return RuleNode

    let value: String;
    let adjacencies: Vec<String>;

    match value_re.captures(rule_str) {
        Some(caps) => {
            value = caps[1].to_string();
            adjacencies = adjacencies_re.find_iter(rule_str)
                .map(|adj| adj.as_str().to_string())
                .collect();
        },
        None => panic!("value regex failed at rule: {}", rule_str)
    };

    return (value, adjacencies);
}

// BFS from node, increment count for every node not yet visited
fn count_paths(adjacency_table: HashMap<String, Vec<String>>, source: &str) -> u32 {
    let mut visited: HashSet<&str> = HashSet::new();
    let mut paths_count: u32 = 0;

    let mut node_queue: Queue<&str> = queue![source];
    while node_queue.size() > 0 {
        let node = node_queue.remove().unwrap();
        if !visited.contains(node) {
            paths_count += 1;
            match adjacency_table.get(&node.to_string()) {
                Some(adjacencies) => {
                    for adjacency in adjacencies {
                        match node_queue.add(adjacency) {
                            Ok(_) => (),
                            Err(e) => println!("error here: {}", e)
                        }
                    }
                },
                None => ()
            }
            visited.insert(node);
        }
    }

    // subtract 1 to ignore the queue starting w/ source in it
    return paths_count - 1;
}

// BFS from node (parent_bag), for each child we encounter, increment & enqueue appropriately
fn count_bags(adjacency_count_table: HashMap<String, Vec<(String, u32)>>, parent_bag: &str) -> u32 {
    let mut bags_count: u32 = 0;

    let mut node_queue: Queue<&str> = queue![parent_bag];
    while node_queue.size() > 0 {
        let node = node_queue.remove().unwrap();
        match adjacency_count_table.get(&node.to_string()) {
            Some(adjacencies) => {
                for adjacency_with_count in adjacencies {
                    let (adjacency, count) = adjacency_with_count;
                    for _i in 0..*count {
                        bags_count += 1;
                        match node_queue.add(adjacency) {
                            Ok(_) => (),
                            Err(e) => println!("error here: {}", e)
                        }
                    }
                }
            },
            None => ()
        }
    }

    return bags_count;
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    let rules: Vec<&str> = contents
        .trim_end()
        .split("\n")
        .collect();

    let mut contained_in_adjacency_table: HashMap<String, Vec<String>> = HashMap::new();
    let mut contains_count_adjacency_table: HashMap<String, Vec<(String, u32)>> = HashMap::new();
    let value_re = Regex::new(r"^(\w+\s\w+)").unwrap();
    let adjacencies_re = Regex::new(r"(?:\d\s(?P<rule>\w+\s\w+)*)").unwrap();

    for rule_str in rules {
        let (value, adjacencies) = parse_rules(rule_str, &value_re, &adjacencies_re);
        for adjacency in adjacencies {
            // bad rust in this loop don't look free me from this prison
            let count = adjacency.chars().nth(0).unwrap().to_digit(10).unwrap();
            let stripped_adjacency = String::from(adjacency).split_off(2);
            // part 1
            match contained_in_adjacency_table.get_mut(&stripped_adjacency) {
                Some(v) => { v.push(value.to_string()); },
                None => { contained_in_adjacency_table.insert(stripped_adjacency.to_string(), vec![value.to_string()]); }
            }
            // part 2
            match contains_count_adjacency_table.get_mut(&value) {
                Some(v) => { v.push((stripped_adjacency, count)); },
                None => { contains_count_adjacency_table.insert(value.to_string(), vec![(stripped_adjacency, count)]); },
            }
        }
    }

    let paths_to_shiny_gold = count_paths(contained_in_adjacency_table, "shiny gold");
    println!("paths to shiny gold: {}", paths_to_shiny_gold);

    let bags_in_shiny_gold = count_bags(contains_count_adjacency_table, "shiny gold");
    println!("bags_in_shiny_gold: {}", bags_in_shiny_gold);
}
