use std::fs;
use std::path::Path;
use std::process;

fn boot(instructions: &mut Vec<(String, bool)>, expected_end: usize) -> (bool, i32, String)
{
    let mut accumulator: i32 = 0;
    let mut instruction_ix: usize = 0;
    // using mutable tuples to track if a command has been executed
    let mut curr_instruction: &mut (String, bool) = &mut instructions[instruction_ix];

    'advance_inst: while curr_instruction.1 == false {
        let parsed_instruction: Vec<&str> = curr_instruction.0.split_whitespace().collect();
        let operation = parsed_instruction[0];
        let argument: i32 = parsed_instruction[1].parse().unwrap();

        match operation {
            "acc" => {
                accumulator += argument;
                // mark instruction as executed
                curr_instruction.1 = true;
                instruction_ix += 1;
                if instruction_ix >= expected_end {
                    break 'advance_inst;
                }
                else {
                    curr_instruction = &mut instructions[instruction_ix];
                }
            },
            "jmp" => {
                curr_instruction.1 = true;
                instruction_ix = (instruction_ix as i32 + argument) as usize;
                if instruction_ix >= expected_end {
                    break 'advance_inst;
                }
                else {
                    curr_instruction = &mut instructions[instruction_ix];
                }
            },
            "nop" => {
                curr_instruction.1 = true;
                instruction_ix += 1;
                if instruction_ix >= expected_end {
                    break 'advance_inst;
                }
                else {
                    curr_instruction = &mut instructions[instruction_ix];
                }
            },
            _ => println!("found unmatched operation: {:?} at line: {}", curr_instruction, instruction_ix),
        }
    }

    let boot_succeeded = instruction_ix == expected_end;
    return (boot_succeeded, accumulator, curr_instruction.0.to_string());
}

fn replace_and_boot(
    instructions: &mut Vec<(String, bool)>,
    expected_end: usize,
    i: usize,
    from: &str,
    to: &str)
{
    instructions[i].0 = instructions[i].0.replace(from, to);
    let (succeeded, acc, _) = boot(instructions, expected_end);
    if succeeded {
        println!("boot succeeded with accumulator: {}", acc);
        process::exit(0);
    }
    // resetting 'state'
    else {
        for j in 0..expected_end -1 {
            instructions[j].1 = false;
        }
        instructions[i].0 = instructions[i].0.replace(to, from);
    }
}

fn main() {
    const INPUT_FILENAME: &str = "./src/input";

    let input_path = Path::new(INPUT_FILENAME);
    let contents = fs::read_to_string(input_path)
        .expect("err when reading file");

    // each instruction is a tuple of the original instruction & an executed flag
    let mut instructions: Vec<(String, bool)> = contents
        .trim_end()
        .split("\n")
        .map(|instr| (String::from(instr), false))
        .collect();
    let expected_end = instructions.len();

    let (boot_succeeded, accumulator, last_executed_instruction) = boot(&mut instructions, expected_end);
    println!("boot succeeded? {}\naccumulator: {}\nlast executed instruction: {}", boot_succeeded, accumulator, last_executed_instruction);

    // Part 2:

    for i in 0..expected_end - 1 {
        let parsed_instruction: Vec<&str> = instructions[i].0.split_whitespace().collect();
        let operation = parsed_instruction[0];

        match operation {
            "jmp" => {
                replace_and_boot(
                    &mut instructions,
                    expected_end,
                    i,
                    "jmp",
                    "nop"
                )
            },
            "nop" => {
                replace_and_boot(
                    &mut instructions,
                    expected_end,
                    i,
                    "nop",
                    "jmp"
                )
            },
            _ => (),
        }
    }
}
