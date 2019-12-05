use crate::day::Day;
use crate::intcode::Program;
use std::str::FromStr;

pub struct Day5 {
    program: Program,
}

impl Day5 {
    pub fn new() -> Result<Day5, String> {
        Ok(Day5 {
            program: Program::from_str(include_str!("input.txt")).map_err(|e| e.to_string())?,
        })
    }
}

impl Day for Day5 {
    fn part1(&mut self) -> Result<String, String> {
        Ok(format!(
            "Diagnostic code is {}",
            part1(&mut self.program.clone())?
        ))
    }
    fn part2(&mut self) -> Result<String, String> {
        Ok(format!(
            "Diagnostic code is {}",
            part2(&mut self.program.clone())?
        ))
    }
}

fn part1(program: &mut Program) -> Result<i32, String> {
    let input = vec![1];
    let mut outputs = program.run(&input);
    let last_output = outputs.pop().ok_or("No last value!".to_owned())?;

    if outputs.iter().any(|x| *x != 0) {
        return Err(format!(
            "Diagnostic outputs weren't all zero: {:?}",
            outputs
        ));
    }

    Ok(last_output)
}

fn part2(program: &mut Program) -> Result<i32, String> {
    let input = vec![5];
    let mut outputs = program.run(&input);
    let last_output = outputs.pop().ok_or("No last value!".to_owned())?;
    if outputs.len() > 0 {
        return Err("The system output more than one number".into());
    }
    Ok(last_output)
}
