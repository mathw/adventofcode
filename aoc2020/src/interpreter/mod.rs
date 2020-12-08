use std::{collections::HashMap, str::FromStr};
use thiserror::Error;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Instruction {
    Acc(i64),
    Nop(i64),
    Jmp(i64),
}

#[derive(Clone, Debug)]
struct Program {
    instructions: Vec<Instruction>,
}

pub struct Interpreter {
    program: Program,
    pub accumulator: i64,
    instruction_pointer: usize,
    instruction_history: HashMap<usize, usize>,
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.trim().split(" ").collect();
        if parts.len() != 2 {
            Err(ParseError(format!("Not two parts in '{}'", s)))
        } else {
            let arg = i64::from_str(parts[1])
                .map_err(|_| ParseError(format!("couldn't parse '{}' as i64", parts[1])))?;
            match parts[0] {
                "acc" => Ok(Instruction::Acc(arg)),
                "nop" => Ok(Instruction::Nop(arg)),
                "jmp" => Ok(Instruction::Jmp(arg)),
                _ => Err(ParseError(format!("Unknown instruction '{}'", parts[0]))),
            }
        }
    }
}

impl FromStr for Program {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions = s
            .lines()
            .map(|l| Instruction::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Program { instructions })
    }
}

impl FromStr for Interpreter {
    type Err = InterpreterError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Interpreter {
            program: Program::from_str(s)?,
            accumulator: 0,
            instruction_pointer: 0,
            instruction_history: HashMap::new(),
        })
    }
}

#[derive(Error, Debug)]
#[error("Parse error")]
pub struct ParseError(String);

#[derive(Error, Debug)]
pub enum InterpreterError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
}

impl Interpreter {
    pub fn run_until_loop(&mut self) -> bool {
        loop {
            let history = self
                .instruction_history
                .entry(self.instruction_pointer)
                .or_insert(0);
            if *history > 0 {
                return true;
            }
            (*history) += 1;

            match self.program.instructions[self.instruction_pointer] {
                Instruction::Nop(_) => self.instruction_pointer += 1,
                Instruction::Acc(arg) => {
                    self.accumulator += arg;
                    self.instruction_pointer += 1;
                }
                Instruction::Jmp(arg) => {
                    self.instruction_pointer = (self.instruction_pointer as i64 + arg) as usize
                }
            }

            if self.instruction_pointer >= self.program.instructions.len() {
                return false;
            }
        }
    }

    pub fn reset(&mut self) {
        self.instruction_pointer = 0;
        self.instruction_history = HashMap::new();
        self.accumulator = 0;
    }

    pub fn swap_jmp(&mut self, instruction: usize) -> Option<Instruction> {
        let i = self.program.instructions.get(instruction)?;
        let new_instruction = match i {
            Instruction::Jmp(arg) => Instruction::Nop(*arg),
            Instruction::Nop(arg) => Instruction::Jmp(*arg),
            Instruction::Acc(arg) => Instruction::Acc(*arg),
        };
        self.program.instructions[instruction] = new_instruction;
        Some(new_instruction)
    }
}

#[test]
fn test_day8_part1() {
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
    interp.run_until_loop();
    assert_eq!(interp.accumulator, 5);
}
