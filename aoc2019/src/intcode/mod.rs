use std::num::ParseIntError;
use std::ops::Index;
use std::ops::IndexMut;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    locations: Vec<i32>,
}

impl FromStr for Program {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(',')
            .map(|n| i32::from_str(n.trim()))
            .collect::<Result<Vec<i32>, Self::Err>>()
            .map(|v| Program { locations: v })
    }
}

impl Program {
    pub fn run(&mut self, inputs: &Vec<i32>) -> Vec<i32> {
        let mut index = Some(0);
        let mut inputs = inputs.iter().rev().cloned().collect();
        let mut outputs = Vec::new();
        while let Some(i) = index {
            index = run_opcode(&mut self.locations, i, &mut inputs, &mut outputs);
        }
        outputs
    }
}

impl Index<usize> for Program {
    type Output = i32;

    fn index(&self, i: usize) -> &Self::Output {
        self.locations.index(i)
    }
}

impl IndexMut<usize> for Program {
    fn index_mut(&mut self, i: usize) -> &mut <Self as Index<usize>>::Output {
        self.locations.index_mut(i)
    }
}

fn run_opcode(
    program: &mut [i32],
    program_counter: usize,
    inputs: &mut Vec<i32>,
    outputs: &mut Vec<i32>,
) -> Option<usize> {
    let (opcode, modes) = opcode_and_modes(program[program_counter] as usize);
    match opcode {
        99 => None,
        1 => Some(run_add(program, program_counter, &modes)),
        2 => Some(run_multiply(program, program_counter, &modes)),
        3 => Some(run_input(program, program_counter, inputs)),
        4 => Some(run_output(program, program_counter, outputs, &modes)),
        5 => Some(run_jump_if(program, program_counter, true, &modes)),
        6 => Some(run_jump_if(program, program_counter, false, &modes)),
        7 => Some(run_less_than(program, program_counter, &modes)),
        8 => Some(run_equals(program, program_counter, &modes)),
        _ => panic!("Unable to interpret opcode {}", opcode),
    }
}

fn run_add(program: &mut [i32], program_counter: usize, modes: &[Mode]) -> usize {
    run_binary_operation(program, program_counter, |a, b| a + b, modes)
}

fn run_multiply(program: &mut [i32], program_counter: usize, modes: &[Mode]) -> usize {
    run_binary_operation(program, program_counter, |a, b| a * b, modes)
}

fn run_binary_operation<F>(
    program: &mut [i32],
    program_counter: usize,
    op: F,
    modes: &[Mode],
) -> usize
where
    F: FnOnce(i32, i32) -> i32,
{
    let result_position = program[program_counter + 3] as usize;

    let first_argument = get_argument_value(program, program_counter + 1, modes[0]);
    let second_argument = get_argument_value(program, program_counter + 2, modes[1]);

    program[result_position] = op(first_argument, second_argument);

    program_counter + 4
}

fn get_argument_value(program: &mut [i32], argument_counter: usize, mode: Mode) -> i32 {
    match mode {
        Mode::Immediate => program[argument_counter],
        Mode::Position => program[program[argument_counter] as usize],
    }
}

fn run_input(program: &mut [i32], program_counter: usize, inputs: &mut Vec<i32>) -> usize {
    let first_argument_position = program[program_counter + 1] as usize;
    let input = inputs.pop().expect("Cannot run input: no more inputs");
    program[first_argument_position] = input;
    program_counter + 2
}

fn run_output(
    program: &mut [i32],
    program_counter: usize,
    outputs: &mut Vec<i32>,
    modes: &[Mode],
) -> usize {
    let first_argument = get_argument_value(program, program_counter + 1, modes[0]);
    outputs.push(first_argument);
    program_counter + 2
}

fn run_jump_if(
    program: &mut [i32],
    program_counter: usize,
    want_true: bool,
    modes: &[Mode],
) -> usize {
    let first = get_argument_value(program, program_counter + 1, modes[0]);
    let second = get_argument_value(program, program_counter + 2, modes[1]);

    if want_true {
        if first != 0 {
            return second as usize;
        }
    } else if first == 0 {
        return second as usize;
    }

    program_counter + 3
}

fn run_less_than(program: &mut [i32], program_counter: usize, modes: &[Mode]) -> usize {
    run_comparative(program, program_counter, |a, b| a < b, modes)
}

fn run_equals(program: &mut [i32], program_counter: usize, modes: &[Mode]) -> usize {
    run_comparative(program, program_counter, |a, b| a == b, modes)
}

fn run_comparative<F>(
    program: &mut [i32],
    program_counter: usize,
    compare: F,
    modes: &[Mode],
) -> usize
where
    F: Fn(i32, i32) -> bool,
{
    let first = get_argument_value(program, program_counter + 1, modes[0]);
    let second = get_argument_value(program, program_counter + 2, modes[1]);
    let answer_location = program[program_counter + 3] as usize;

    program[answer_location] = if compare(first, second) { 1 } else { 0 };

    program_counter + 4
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Mode {
    Position,
    Immediate,
}

fn opcode_and_modes(o: usize) -> (usize, Vec<Mode>) {
    let opcode = o % 100;
    let modes = o
        .to_string()
        .chars()
        .rev()
        .skip(2)
        .map(|c| match c {
            '0' => Mode::Position,
            '1' => Mode::Immediate,
            _ => panic!(format!("Unknown parameter mode {}", c)),
        })
        // ensure there is always a default pair of position parameters on the end
        .chain(vec![Mode::Position, Mode::Position])
        .take(match o {
            99 => 0,
            1 => 2,
            2 => 2,
            3 => 1,
            4 => 1,
            _ => 2,
        })
        .collect();

    (opcode, modes)
}

#[test]
fn test_opcode_no_modes() {
    assert_eq!(opcode_and_modes(4), (4, vec![Mode::Position]));
}

#[test]
fn test_opcode_two_modes() {
    assert_eq!(
        opcode_and_modes(1002),
        (2, vec![Mode::Position, Mode::Immediate])
    );
}

#[test]
fn test_from_str_happy() {
    let input = "2,4,5,219 ,00,2920";
    let program = Program::from_str(input).expect("This should not fail");
    assert_eq!(
        program,
        Program {
            locations: vec![2, 4, 5, 219, 0, 2920]
        }
    );
}

#[test]
fn test_from_str_unhappy() {
    let input = "2,4,potato,219,00,2920";
    let program = Program::from_str(input);
    assert!(program.is_err());
    assert_eq!(program.err(), usize::from_str("potato").err());
}

#[test]
fn test_sample_1() {
    let mut program = vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50];
    let new_counter = run_opcode(&mut program, 0, &mut vec![], &mut vec![]);
    assert_eq!(new_counter, Some(4), "New program counter is incorrect");
    assert_eq!(program[3], 70, "Index 3 should have been updated to be 70")
}

#[test]
fn test_halt() {
    let mut program = vec![1, 2, 3, 4, 99, 1, 2, 4, 4];
    let new_counter = run_opcode(&mut program, 4, &mut vec![], &mut vec![]);
    assert_eq!(new_counter, None);
}

#[test]
fn test_input() {
    let mut program = vec![3, 2, 5, 22];
    let mut inputs = vec![9, 8];
    let new_counter = run_opcode(&mut program, 0, &mut inputs, &mut vec![]);
    // remembering inputs are consumed from the end of the vec at this internal level
    assert_eq!(new_counter, Some(2));
    assert_eq!(inputs, vec![9]);
    assert_eq!(program[2], 8);
}

#[test]
fn test_output() {
    let mut program = vec![4, 2, 5, 22];
    let mut outputs = vec![];
    let new_counter = run_opcode(&mut program, 0, &mut vec![], &mut outputs);
    assert_eq!(new_counter, Some(2));
    assert_eq!(outputs, vec![5]);
    assert_eq!(program, vec![4, 2, 5, 22]);
}

#[test]
fn test_run_sample() {
    let mut program = Program {
        locations: vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
    };
    program.run(&mut vec![]);
    assert_eq!(program[0], 3500);
}

#[test]
fn test_output_immediate() {
    let mut program = vec![104, 2, 5, 22];
    let mut outputs = vec![];
    let new_counter = run_opcode(&mut program, 0, &mut vec![], &mut outputs);
    assert_eq!(new_counter, Some(2));
    assert_eq!(outputs, vec![2]);
    assert_eq!(program, vec![104, 2, 5, 22]);
}

#[test]
fn test_day5_part2_sample1() {
    // checks if input == 8, outputs 1 if so, 0 if not
    let program = Program::from_str("3,9,8,9,10,9,4,9,99,-1,8").expect("Program should parse");

    // check == 8
    let mut program1 = program.clone();
    let outputs = program1.run(&mut vec![8]);
    assert_eq!(outputs, vec![1]);

    // check != 8
    let mut program2 = program.clone();
    let outputs = program2.run(&mut vec![6]);
    assert_eq!(outputs, vec![0]);
}

#[test]
fn test_day5_part2_sample3() {
    // checks if input == 8, outputs 1 if so, 0 if not
    let program = Program::from_str("3,3,1108,-1,8,3,4,3,99").expect("Program should parse");

    // check == 8
    let mut program1 = program.clone();
    let outputs = program1.run(&mut vec![8]);
    assert_eq!(outputs, vec![1]);

    // check != 8
    let mut program2 = program.clone();
    let outputs = program2.run(&mut vec![6]);
    assert_eq!(outputs, vec![0]);
}

#[test]
fn test_day5_part2_big_sample() {
    // takes one input
    // outputs 999 if input < 8, 1000 if input == 8, 1001 if input > 8
    // uses jump instructions to do it
    let program = Program::from_str(
        "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99",
    )
    .expect("Program should parse");

    // check == 8
    let mut program1 = program.clone();
    let outputs = program1.run(&mut vec![8]);
    assert_eq!(outputs, vec![1000]);

    // check < 8
    let mut program1 = program.clone();
    let outputs = program1.run(&mut vec![7]);
    assert_eq!(outputs, vec![999]);

    // check > 8
    let mut program1 = program.clone();
    let outputs = program1.run(&mut vec![9]);
    assert_eq!(outputs, vec![1001]);
}
