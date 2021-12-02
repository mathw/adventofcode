use crate::common::{Command, Position};
use crate::day::{DayResult, PartResult};
use std::error::Error;
use std::str::FromStr;

pub fn run() -> std::result::Result<DayResult, Box<(dyn Error + 'static)>> {
    let commands = parse_input(include_str!("inputs/day2.txt"))?;
    let part1_result = part1(commands.iter().cloned());
    let part2_result = part2(commands.into_iter());
    Ok(DayResult::new(
        PartResult::Success(format!(
            "Ended up at position x={} y={} answer={}",
            part1_result.x(),
            part1_result.y(),
            part1_result.x() * part1_result.y()
        )),
        PartResult::Success(format!(
            "Ended up at position x={} y={} answer={}",
            part2_result.x(),
            part2_result.y(),
            part2_result.x() * part2_result.y()
        )),
    ))
}

fn parse_input(input: &str) -> Result<Vec<Command>, Box<(dyn Error + 'static)>> {
    Ok(input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| Command::from_str(l))
        .collect::<Result<Vec<_>, _>>()?)
}

fn part1(commands: impl Iterator<Item = Command>) -> Position {
    let starting_position = Position::default();
    Command::execute_commands(starting_position, commands)
}

fn part2(commands: impl Iterator<Item = Command>) -> Position {
    let starting_position = Position::default();
    Command::execute_commands_with_aim(starting_position, commands)
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/day2-sample.txt");
    let commands = parse_input(input).expect("Commands must parse");
    let result = part1(commands.into_iter());
    assert_eq!(result.x(), 15);
    assert_eq!(result.y(), 10);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("inputs/day2-sample.txt");
    let commands = parse_input(input).expect("Commands must parse");
    let result = part2(commands.into_iter());
    assert_eq!(result.x(), 15);
    assert_eq!(result.y(), 60);
}
