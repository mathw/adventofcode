use crate::intcode::Program;
use std::str::FromStr;

pub fn run() -> Result<(), String> {
    let program = Program::from_str(include_str!("input.txt")).map_err(|e| e.to_string())?;

    let result = part1(&mut program.clone())?;

    println!("Part 1: Diagnostic code is {}", result);

    let result = part2(&mut program.clone())?;

    println!("Part 2: Diagnostic code is {}", result);

    Ok(())
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
