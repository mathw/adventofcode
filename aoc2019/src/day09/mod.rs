use crate::day::Day;
use crate::intcode::Program;
use std::num::ParseIntError;
use std::str::FromStr;

pub struct Day9 {
    program: Program<i64>,
}

impl Day9 {
    pub fn new() -> Result<Day9, String> {
        Ok(Day9 {
            program: Program::from_str(include_str!("input.txt"))
                .map_err(|e: ParseIntError| e.to_string())?,
        })
    }
}

impl Day for Day9 {
    fn part1(&mut self) -> Result<String, String> {
        let p =
            Program::<i32>::from_str("109,1,12101,1,-1,2,109,2,99").expect("Program should parse");
        p.run_pure(&Vec::new());
        Err("Not implemented".into())
    }

    fn part2(&mut self) -> Result<String, String> {
        Err("Not implemented".into())
    }
}
