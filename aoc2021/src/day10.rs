use crate::day::{DayResult, PartResult};
use std::collections::VecDeque;
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = include_str!("inputs/day10.txt");
    let part1 = part1(input)?;
    Ok(DayResult::new(
        PartResult::Success(format!("Score is {}", part1)),
        PartResult::NotImplemented,
    ))
}

enum LineResult {
    Valid,
    Corrupted(char),
    Incomplete,
}

impl LineResult {
    fn score(&self) -> u32 {
        match self {
            LineResult::Corrupted(')') => 3,
            LineResult::Corrupted(']') => 57,
            LineResult::Corrupted('}') => 1197,
            LineResult::Corrupted('>') => 25137,
            _ => 0,
        }
    }
}

fn part1(input: &str) -> Result<u32, String> {
    Ok(input
        .lines()
        .map(|l| check_line(l))
        .collect::<Result<Vec<LineResult>, _>>()?
        .into_iter()
        .map(|r| r.score())
        .sum())
}

fn check_line(line: &str) -> Result<LineResult, String> {
    let mut state = VecDeque::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => state.push_back(c),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = state.back() {
                    if *last == matching_opening(c)? {
                        state.pop_back();
                    } else {
                        return Ok(LineResult::Corrupted(c));
                    }
                }
            }
            _ => panic!("Undefined character {} in line", c),
        }
    }

    if state.len() == 0 {
        Ok(LineResult::Valid)
    } else {
        Ok(LineResult::Incomplete)
    }
}

fn matching_opening(c: char) -> Result<char, String> {
    match c {
        ')' => Ok('('),
        ']' => Ok('['),
        '}' => Ok('{'),
        '>' => Ok('<'),
        _ => Err(format!("No closing character known for {}", c)),
    }
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/samples/day10.txt");
    let result = part1(input);
    assert_eq!(result, Ok(26397));
}
