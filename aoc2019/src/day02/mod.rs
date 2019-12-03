use std::str::FromStr;

pub fn run() -> Result<(), String> {
    let program = parse_input(include_str!("input.txt"));

    let mut program1 = program.clone();

    run_part_1(&mut program1);

    println!("Part 1: {}", program1[0]);

    if let Some((noun, verb)) = run_part_2(program) {
        println!("Part 2: {}", 100 * noun + verb);
    } else {
        println!("Part 2: no answer found");
    }

    Ok(())
}

fn parse_input(input: &str) -> Vec<usize> {
    input
        .split(',')
        .map(|n| {
            usize::from_str(n.trim()).expect(&format!("Puzzle input '{}' should be trustworthy", n))
        })
        .collect()
}

fn run_part_1(program: &mut [usize]) {
    program[1] = 12;
    program[2] = 2;
    run_program(program);
}

fn run_part_2(program: Vec<usize>) -> Option<(usize, usize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            run_program(&mut program);

            if program[0] == 19690720 {
                return Some((noun, verb));
            }
        }
    }
    None
}

fn run_program(program: &mut [usize]) {
    let mut index = Some(0);
    while let Some(i) = index {
        index = run_opcode(program, i);
    }
}

fn run_opcode(program: &mut [usize], program_counter: usize) -> Option<usize> {
    let opcode = program[program_counter];
    match opcode {
        99 => None,
        1 => Some(run_add(program, program_counter)),
        2 => Some(run_multiply(program, program_counter)),
        _ => panic!("Unable to interpret opcode {}", opcode),
    }
}

fn run_add(program: &mut [usize], program_counter: usize) -> usize {
    run_operation(program, program_counter, |a, b| a + b)
}

fn run_multiply(program: &mut [usize], program_counter: usize) -> usize {
    run_operation(program, program_counter, |a, b| a * b)
}

fn run_operation<F>(program: &mut [usize], program_counter: usize, op: F) -> usize
where
    F: FnOnce(usize, usize) -> usize,
{
    let first_argument_position = program[program_counter + 1];
    let second_argument_position = program[program_counter + 2];
    let result_position = program[program_counter + 3];

    program[result_position] = op(
        program[first_argument_position],
        program[second_argument_position],
    );

    program_counter + 4
}

#[test]
fn test_sample_1() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let new_counter = run_opcode(&mut program, 0);
    assert_eq!(new_counter, Some(4), "New program counter is incorrect");
    assert_eq!(program[3], 70, "Index 3 should have been updated to be 70")
}

#[test]
fn test_halt() {
    let mut program = vec![1, 2, 3, 4, 99, 1, 2, 4, 4];
    let new_counter = run_opcode(&mut program, 4);
    assert_eq!(new_counter, None);
}

#[test]
fn test_run_sample() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    run_program(&mut program);
    assert_eq!(program[0], 3500);
}
