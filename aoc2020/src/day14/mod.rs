use crate::dayerror::DayError;
use regex::Regex;
use std::{collections::HashMap, str, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let mut computer = Computer::from_str(input)?;
    computer.run();
    Ok(format!(
        "The sum of memory upon completion is {}",
        computer.memory_sum()
    ))
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mask(HashMap<usize, bool>);

impl Mask {
    fn get(&self, index: usize) -> Option<bool> {
        self.0.get(&index).map(|x| *x)
    }

    fn apply(&self, value: u64) -> u64 {
        // horrible hacky slow method but hey let's just start somewhere
        let mut binary_string = Vec::from(format!("{:b}", value).as_bytes());
        binary_string.reverse();
        for i in 0..36 {
            if let Some(replacement) = self.get(i) {
                while i >= binary_string.len() {
                    binary_string.push('0' as u8);
                }
                binary_string[i] = if replacement { '1' } else { '0' } as u8;
            }
        }
        binary_string.reverse();
        u64::from_str_radix(
            str::from_utf8(&binary_string)
                .expect("This should absolutely not have produced a bad string"),
            2,
        )
        .expect("The absolutely not wrong string should parse back to u64")
    }
}

impl FromStr for Mask {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = HashMap::new();

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '1' => {
                    mask.insert(i, true);
                }
                '0' => {
                    mask.insert(i, false);
                }
                _ => {}
            }
        }

        Ok(Mask(mask))
    }
}

#[test]
fn test_apply_mask() {
    let mask = Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X").unwrap();
    let result = mask.apply(0b1001);
    assert_eq!(result, 0b1001001);
    let result = mask.apply(0b1100101);
    assert_eq!(result, 0b1100101);
    let result = mask.apply(0b0);
    assert_eq!(result, 0b1000000);
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    SetMask(Mask),
    SetMem(usize, u64),
}

impl FromStr for Instruction {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MASK_SET: Regex = Regex::new(r"mask = ([01X]{36})").unwrap();
            static ref MEM_SET: Regex = Regex::new(r"mem\[(\d+)\] = (\d+)").unwrap();
        }

        if let Some(caps) = MASK_SET.captures(s) {
            let new_mask = Mask::from_str(&caps[1])?;
            return Ok(Instruction::SetMask(new_mask));
        }

        if let Some(caps) = MEM_SET.captures(s) {
            let address = caps[1].parse::<usize>()?;
            let value = caps[2].parse::<u64>()?;
            return Ok(Instruction::SetMem(address, value));
        }

        Err(DayError::NoSolutionFoundError)
    }
}

struct Computer {
    memory: HashMap<usize, u64>,
    mask: Mask,
    program: Vec<Instruction>,
}

impl FromStr for Computer {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let program = s
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .map(|l| Instruction::from_str(l))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Computer {
            memory: HashMap::new(),
            mask: Mask::from_str("XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")?,
            program,
        })
    }
}

impl Computer {
    fn run(&mut self) {
        for instruction in &self.program {
            match instruction {
                Instruction::SetMask(mask) => {
                    self.mask = mask.clone();
                }
                Instruction::SetMem(address, value) => {
                    self.memory.insert(*address, self.mask.apply(*value));
                }
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[test]
fn test_part1_sample() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";
    let mut computer = Computer::from_str(input).expect("Computer should have parsed");
    computer.run();
    let result = computer.memory_sum();
    assert_eq!(result, 165);
}
