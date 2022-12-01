use crate::day::{Day, DayResult, PartResult};
use std::str::FromStr;

pub struct Day1;

impl Day1 {
    pub fn new() -> Day1 {
        Day1
    }
}

impl Day for Day1 {
    fn run(
        &mut self,
    ) -> std::result::Result<DayResult, std::boxed::Box<(dyn std::error::Error + 'static)>> {
        let input = parse_input(include_str!("inputs/day1.txt"))?;
        let most_calories = calculate_most_calories(&input);
        Ok(
            Day::new(PartResult::Success(most_calories.to_string(), PartResult::NotImplemented))
        )
}

fn parse_input(input: &str) -> Result<Vec<Vec<u32>>, std::num::ParseIntError> {
    let mut groups = Vec::new();
    let mut i = input.lines();

    loop {
        let current_group = i.take_while(|l| l.trim().len() > 0).map(u32::from_str).collect::<Result<Vec<u32>, _>>()?;
        groups.add(current_group);
    }

    return Ok(groups);
}

fn calculate_most_calories(input: &Vec<Vec<u32>>) -> u32 {
    let all_calories = input.map(|elf| elf.iter().sum());
    all_calories.max()
}

#[test]
fn test_part1_sample() {
    let input = parse_input(include_str!("inputs/part1-sample.txt")).unwrap();
    let most_calories = calculate_most_calories(&input);
    assert_eq!(most_calories, 24000);
}

#[test]
fn test_part2_sample() {
}
