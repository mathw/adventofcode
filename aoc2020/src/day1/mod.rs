use crate::dayerror::DayError;
use std::{num::ParseIntError, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let input = get_input()?;
    let (first, second) =
        find_numbers_summing_to(&input, 2020).ok_or(DayError::NoSolutionFoundError)?;
    let result = first * second;
    Ok(format!("{}", result))
}

pub fn part2() -> Result<String, DayError> {
    let input = get_input()?;
    let (first, second, third) =
        find_three_numbers_summing_to(&input, 2020).ok_or(DayError::NoSolutionFoundError)?;
    let result = first * second * third;
    Ok(format!("{}", result))
}

fn get_input() -> Result<Vec<u32>, ParseIntError> {
    include_str!("input.txt")
        .lines()
        .map(|line| u32::from_str(line))
        .collect::<Result<_, ParseIntError>>()
}

fn find_numbers_summing_to(numbers: &Vec<u32>, target: u32) -> Option<(u32, u32)> {
    for first in numbers.iter() {
        for second in numbers.iter() {
            if first + second == target {
                return Some((*first, *second));
            }
        }
    }
    None
}

fn find_three_numbers_summing_to(numbers: &Vec<u32>, target: u32) -> Option<(u32, u32, u32)> {
    for first in numbers.iter() {
        if *first > target {
            continue;
        }
        for second in numbers.iter() {
            if second + first > target {
                continue;
            }
            for third in numbers.iter() {
                if first + second + third == target {
                    return Some((*first, *second, *third));
                }
            }
        }
    }
    None
}
