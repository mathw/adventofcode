use crate::DayError;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let mut computer = Computer::from_str(input)?;
    computer.run();
    Ok(format!(
        "The sum of all memory is now {}",
        computer.memory_sum()
    ))
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MaskBit {
    One,
    Floating,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Mask(HashMap<usize, MaskBit>);

impl Mask {
    fn apply(&self, value: u64) -> Vec<u64> {
        let mut value_bits = format!("{:b}", value).chars().collect::<Vec<char>>();
        value_bits.reverse();
        let mut template = Vec::new();
        for bit in 0..36 {
            match self.0.get(&bit) {
                Some(MaskBit::One) => template.push(MaskingValue::One),
                Some(MaskBit::Floating) => template.push(MaskingValue::Float),
                None => template.push(match value_bits.get(bit) {
                    Some('0') => MaskingValue::Zero,
                    Some('1') => MaskingValue::One,
                    None => MaskingValue::Zero,
                    Some(v) => panic!("non-bit value {} found while masking", v),
                }),
            }
        }

        let all_values = Mask::apply_template_float(template);
        all_values
            .iter()
            .map(|v| Mask::filled_template_to_value(v))
            .collect()
    }

    fn apply_template_float(template: Vec<MaskingValue>) -> Vec<Vec<MaskingValue>> {
        let first_float_index = template
            .iter()
            .enumerate()
            .filter(|(_, c)| c == &&MaskingValue::Float)
            .map(|(i, _)| i)
            .next();
        if let Some(i) = first_float_index {
            let mut results = vec![template.clone(), template.clone()];
            results[0][i] = MaskingValue::Zero;
            results[1][i] = MaskingValue::One;
            results
                .into_iter()
                .flat_map(|r| Mask::apply_template_float(r).into_iter())
                .collect()
        } else {
            vec![template.clone()]
        }
    }

    fn filled_template_to_value(template: &Vec<MaskingValue>) -> u64 {
        let mut result = ['0'; 36];

        for (index, value) in template.iter().enumerate() {
            result[index] = match value {
                MaskingValue::One => '1',
                MaskingValue::Zero => '0',
                MaskingValue::Float => {
                    panic!("Someone called this function on an unfilled template")
                }
            }
        }

        u64::from_str_radix(&result.iter().collect::<String>(), 2)
            .expect("This should parse just fine as binary")
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum MaskingValue {
    One,
    Zero,
    Float,
}

impl FromStr for Mask {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mask = HashMap::new();

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '1' => {
                    mask.insert(i, MaskBit::One);
                }
                'X' => {
                    mask.insert(i, MaskBit::Floating);
                }
                _ => {}
            }
        }

        Ok(Mask(mask))
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Instruction {
    SetMask(Mask),
    SetMem(u64, u64),
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
            let address = caps[1].parse::<u64>()?;
            let value = caps[2].parse::<u64>()?;
            return Ok(Instruction::SetMem(address, value));
        }

        Err(DayError::NoSolutionFoundError)
    }
}

struct Computer {
    memory: HashMap<u64, u64>,
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
                    let addresses = self.mask.apply(*address);
                    for address in addresses {
                        self.memory.insert(address, *value);
                    }
                }
            }
        }
    }

    fn memory_sum(&self) -> u64 {
        self.memory.values().sum()
    }
}

#[test]
fn test_part2_sample() {
    let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    let mut computer = Computer::from_str(input).expect("Computer should have parsed");
    computer.run();
    let result = computer.memory_sum();
    assert_eq!(result, 208);
}
