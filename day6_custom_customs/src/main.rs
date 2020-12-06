use std::fs;
use std::path::Path;
use std::collections::HashSet;
use std::collections::HashMap;

fn count_unique_answers(group_response_str: &str) -> usize {
    let mut unique_answers: HashSet<char> = HashSet::new();
    let formatted_group_response = group_response_str.replace('\n', "");

    for answer in formatted_group_response.chars() {
        unique_answers.insert(answer);
    }

    return unique_answers.len();
}

fn count_matching_answers(group_response_str: &str) -> usize {
    let mut matching_answers: HashMap<char, usize> = HashMap::new();
    let group_responses: Vec<&str> = group_response_str.split('\n').collect();

    for response in group_responses.iter() {
        for answer in response.chars() {
            *matching_answers.entry(answer).or_insert(0) += 1;
        }
    }

    let matching_answers_count = matching_answers
        .values()
        .fold(0, |acc, answer_count| {
            if answer_count == &group_responses.len() { acc + 1 } else { acc }
        });
    
    return matching_answers_count;
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    let group_responses: Vec<&str> = contents
        .trim_end()
        .split("\n\n")
        .collect();

    let unique_answers_sum = group_responses.iter()
        .fold(0, |acc, group_response_str| acc + count_unique_answers(group_response_str));

    println!("unique_answers_sum: {}", unique_answers_sum);

    let matching_answers_sum = group_responses.iter()
        .fold(0, |acc, group_response_str| acc + count_matching_answers(group_response_str));

    println!("matching_answers_sum: {}", matching_answers_sum);
}
