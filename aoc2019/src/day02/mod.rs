use crate::intcode::Program;
use std::str::FromStr;

pub fn run() -> Result<(), String> {
    let program = Program::from_str(include_str!("input.txt")).map_err(|e| e.to_string())?;

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

fn run_part_1(program: &mut Program) {
    program[1] = 12;
    program[2] = 2;
    program.run(&mut vec![]);
}

fn run_part_2(program: Program) -> Option<(usize, usize)> {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            program.run(&mut vec![]);

            if program[0] == 19690720 {
                return Some((noun, verb));
            }
        }
    }
    None
}
