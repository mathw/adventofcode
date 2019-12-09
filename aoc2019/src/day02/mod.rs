use crate::day::Day;
use crate::intcode::Program;
use std::str::FromStr;

pub struct Day2 {
    program: Program<i32>,
}

impl Day2 {
    pub fn new() -> Result<Day2, String> {
        let it = Day2 {
            program: Program::<i32>::from_str(include_str!("input.txt"))
                .map_err(|e| e.to_string())?,
        };
        Ok(it)
    }
}

impl Day for Day2 {
    fn part1(&mut self) -> Result<String, String> {
        let mut program1 = self.program.clone();
        run_part_1(&mut program1);
        Ok(program1[0].to_string())
    }

    fn part2(&mut self) -> Result<String, String> {
        if let Some((noun, verb)) = run_part_2(self.program.clone()) {
            Ok(format!("{}", 100 * noun + verb))
        } else {
            Ok(format!("no answer found"))
        }
    }
}

fn run_part_1(program: &mut Program<i32>) {
    program[1] = 12;
    program[2] = 2;
    program.run(&mut vec![]);
}

fn run_part_2(program: Program<i32>) -> Option<(usize, usize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            program.run(&mut vec![]);

            if program[0] == 19690720 {
                return Some((noun as usize, verb as usize));
            }
        }
    }
    None
}
