use crate::day::Day;
use crate::intcode::{Program, RunState, State};
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::VecDeque;
use std::str::FromStr;

pub struct Day7 {
    program: Program,
}

impl Day7 {
    pub fn new() -> Result<Self, String> {
        Ok(Day7 {
            program: Program::from_str(include_str!("input.txt")).map_err(|e| e.to_string())?,
        })
    }
}

impl Day for Day7 {
    fn part1(&mut self) -> Result<String, String> {
        let answer = find_best_output(&self.program);
        Ok(format!("The best possible output is {}", answer))
    }

    fn part2(&mut self) -> Result<String, String> {
        let answer = find_best_output_2(&self.program)?;
        Ok(format!(
            "The best possible output in loop mode is {}",
            answer
        ))
    }
}

fn all_inputs() -> Vec<Vec<u8>> {
    (0..=4).permutations(5).collect()
}

fn run_iteration(program: &Program, input: &[u8]) -> i32 {
    let mut input_signal = 0;

    for i in input {
        let outputs = program.run_pure(&vec![*i as i32, input_signal]);
        input_signal = outputs[0];
    }

    input_signal
}

fn find_best_output(program: &Program) -> i32 {
    all_inputs()
        .par_iter()
        .map(|input| run_iteration(program, &input))
        .max()
        .expect("Should always have elements")
}

fn all_inputs_2() -> Vec<Vec<u8>> {
    (5..=9).permutations(5).collect()
}

fn find_best_output_2(program: &Program) -> Result<i32, String> {
    all_inputs_2()
        .par_iter()
        .map(|i| run_iteration_2(program, i))
        .collect::<Result<Vec<i32>, String>>()
        .map(|v| v.iter().max().expect("Should always have elements").clone())
}

fn run_iteration_2(program: &Program, phases: &[u8]) -> Result<i32, String> {
    let mut a1_inputs = VecDeque::from(vec![phases[0] as i32, 0]);
    let mut a1 = program.run_until_needs_interaction();
    let mut a2_inputs = VecDeque::from(vec![phases[1] as i32]);
    let mut a2 = program.run_until_needs_interaction();
    let mut a3_inputs = VecDeque::from(vec![phases[2] as i32]);
    let mut a3 = program.run_until_needs_interaction();
    let mut a4_inputs = VecDeque::from(vec![phases[3] as i32]);
    let mut a4 = program.run_until_needs_interaction();
    let mut a5_inputs = VecDeque::from(vec![phases[4] as i32]);
    let mut a5 = program.run_until_needs_interaction();

    loop {
        a1 = process_amplifier(a1, &mut a1_inputs, &mut a2_inputs);
        a2 = process_amplifier(a2, &mut a2_inputs, &mut a3_inputs);
        a3 = process_amplifier(a3, &mut a3_inputs, &mut a4_inputs);
        a4 = process_amplifier(a4, &mut a4_inputs, &mut a5_inputs);
        a5 = process_amplifier(a5, &mut a5_inputs, &mut a1_inputs);

        if a5.state == State::Completed {
            if a1_inputs.len() == 1 {
                return a1_inputs
                    .pop_front()
                    .ok_or("Expected one input on a1".into());
            } else {
                return Err("Multiple unconsumed a1 inputs found at a5 termination".into());
            }
        }
    }

    fn process_amplifier(
        state: RunState,
        inputs: &mut VecDeque<i32>,
        next_inputs: &mut VecDeque<i32>,
    ) -> RunState {
        match state.state {
            State::NeedsInput => {
                if let Some(input) = inputs.pop_front() {
                    state.resume_with_input(input)
                } else {
                    state
                }
            }
            State::ProvidedOutput(o) => {
                next_inputs.push_back(o);
                state.resume()
            }
            State::Completed => state,
        }
    }
}

#[test]
fn test_one_iteration() {
    let prog = Program::from_str("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        .expect("test program should compile!");
    let phases = vec![4, 3, 2, 1, 0];
    let result = run_iteration(&prog, &phases);
    assert_eq!(result, 43210);
}

#[test]
fn test_find_first_sample() {
    let prog = Program::from_str("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0")
        .expect("test program should compile!");
    let result = find_best_output(&prog);
    assert_eq!(result, 43210);
}

#[test]
fn test_find_second_sample() {
    let prog = Program::from_str(
        "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0",
    )
    .expect("test program should compile!");
    let result = find_best_output(&prog);
    assert_eq!(result, 54321);
}

#[test]
fn test_find_third_sample() {
    let prog = Program::from_str(
        "3,31,3,32,1002,32,10,32,
1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0",
    )
    .expect("test program should compile!");
    let result = find_best_output(&prog);
    assert_eq!(result, 65210);
}

#[test]
fn test_part2_first_sample() {
    let prog = Program::from_str(
        "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5",
    )
    .expect("test program should compile!");
    let result = find_best_output_2(&prog).expect("A result should be delivered");
    assert_eq!(result, 139629729);
}

#[test]
fn test_part2_second_sample() {
    let prog = Program::from_str(
        "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10",
    )
    .expect("test program should compile!");
    let result = find_best_output_2(&prog).expect("A result should be delivered");
    assert_eq!(result, 18216);
}
