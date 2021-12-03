use crate::day::{DayResult, PartResult};
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error + 'static>> {
    let (input, input_size) = parse_input(include_str!("inputs/day3.txt"))?;
    let part1 = part1(&input, input_size);
    Ok(DayResult::new(
        PartResult::Success(format!("Power consumption is {}", part1)),
        PartResult::NotImplemented,
    ))
}

fn parse_input_line(l: &str) -> Result<Vec<bool>, String> {
    l.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(format!("{} is not 1 or 0", c)),
        })
        .collect::<Result<Vec<bool>, String>>()
}

fn parse_input(input: &str) -> Result<(Vec<Vec<bool>>, usize), String> {
    let parsed = input
        .lines()
        .map(|l| parse_input_line(l))
        .collect::<Result<Vec<_>, _>>()?;

    let first_size = parsed[0].len();
    for n in parsed.iter() {
        if first_size != n.len() {
            return Err(format!("Not all input numbers are the same length"));
        }
    }

    Ok((parsed, first_size))
}

fn part1(inputs: &Vec<Vec<bool>>, input_size: usize) -> u64 {
    let mut ones = (0..input_size).map(|_| 0).collect::<Vec<usize>>();
    let mut zeroes = (0..input_size).map(|_| 0).collect::<Vec<usize>>();

    for entry in inputs {
        for bit in 0..input_size {
            if entry[bit] {
                ones[bit] += 1;
            } else {
                zeroes[bit] += 1;
            }
        }
    }

    let gamma_bits = (0..input_size)
        .map(|bit| if ones[bit] > zeroes[bit] { true } else { false })
        .collect::<Vec<_>>();
    let epsilon_bits = gamma_bits.iter().map(|bit| !bit).collect::<Vec<_>>();

    let gamma = make_decimal_from_bit_vec(&gamma_bits);
    let epsilon = make_decimal_from_bit_vec(&epsilon_bits);

    gamma * epsilon
}

fn make_decimal_from_bit_vec(input: &Vec<bool>) -> u64 {
    let str = input
        .iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect::<String>();
    u64::from_str_radix(&str, 2).expect("Expected a valid binary string")
}

#[test]
fn test_part1_example() {
    let (input, input_size) =
        parse_input(include_str!("inputs/day3-sample.txt")).expect("Input should parse");
    let part1 = part1(&input, input_size);
    assert_eq!(part1, 198);
}
