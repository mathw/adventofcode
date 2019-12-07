use crate::day::Day;
use crate::intcode::Program;
use itertools::Itertools;
use rayon::prelude::*;
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
        Err("Not implemented".to_owned())
    }
}

fn all_inputs() -> Vec<Vec<u8>> {
    (0..=4).permutations(5).collect()
}

fn all_inputs_2() -> Vec<Vec<u8>> {
    (5..=9).permutations(5).collect()
}

fn run_iteration(program: &Program, input: &[u8]) -> i32 {
    let mut input_signal = 0;

    for i in input {
        let mut prog = program.clone();
        let outputs = prog.run(&vec![*i as i32, input_signal]);
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
