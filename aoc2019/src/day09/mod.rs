use crate::day::Day;
use crate::intcode::Program;
use itertools::Itertools;
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
        let outputs = self.program.run_pure(&vec![1]);
        if outputs.len() == 1 {
            Ok(format!("BOOST code is {}", outputs[0]))
        } else {
            Err(format!(
                "The following opcodes may be malfunctioning: {}",
                outputs
                    .iter()
                    .map(|x| x.to_string())
                    .intersperse(", ".to_owned())
                    .collect::<String>()
            ))
        }
    }

    fn part2(&mut self) -> Result<String, String> {
        Err("Not implemented".into())
    }
}
