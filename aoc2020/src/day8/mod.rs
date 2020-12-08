use crate::{dayerror::DayError, interpreter::Interpreter};
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let mut interpreter = Interpreter::from_str(include_str!("input.txt"))?;
    interpreter.run_until_loop();
    Ok(format!(
        "Accumulator at loop is {}",
        interpreter.accumulator
    ))
}

pub fn part2() -> Result<String, DayError> {
    let mut interpreter = Interpreter::from_str(include_str!("input.txt"))?;
    let (answer, modified_instruction) = part2_bruteforce(&mut interpreter)?;
    Ok(format!(
        "Accumulator after termination is {}. I modified instruction {}",
        answer, modified_instruction
    ))
}

fn part2_bruteforce(interp: &mut Interpreter) -> Result<(i64, usize), DayError> {
    let mut modified_instruction = 0;
    loop {
        let did_loop = interp.run_until_loop();
        if !did_loop {
            return Ok((interp.accumulator, modified_instruction - 1));
        }

        interp.reset();
        if modified_instruction > 0 {
            interp.swap_jmp(modified_instruction - 1);
        }
        let m = interp.swap_jmp(modified_instruction);
        if m.is_none() {
            return Err(DayError::NoSolutionFoundError);
        }
        modified_instruction += 1;
    }
}

#[test]
fn test_part2_bruteforce() {
    let code = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    let mut interp = Interpreter::from_str(code).expect("Code should parse");
    let result = part2_bruteforce(&mut interp).expect("Solution should be found");
    assert_eq!(result, (8, 7));
}
