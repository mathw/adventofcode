use crate::day::{DayResult, PartResult};
use std::collections::VecDeque;
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = include_str!("inputs/day10.txt");
    let part1 = part1(input)?;
    let part2 = part2(input)?;
    Ok(DayResult::new(
        PartResult::Success(format!("Score is {}", part1)),
        PartResult::Success(format!("Score is {}", part2)),
    ))
}

fn part1(input: &str) -> Result<u64, String> {
    Ok(input
        .lines()
        .map(|l| check_line(l))
        .collect::<Result<Vec<LineResult>, _>>()?
        .into_iter()
        .filter(|r| r.is_corrupted())
        .map(|r| r.score())
        .sum())
}

fn part2(input: &str) -> Result<u64, String> {
    let mut line_scores = input
        .lines()
        .map(|l| check_line(l))
        .collect::<Result<Vec<LineResult>, _>>()?
        .into_iter()
        .filter_map(|r| match r {
            LineResult::Incomplete(co) => Some(completion_score(&co)),
            _ => None,
        })
        .collect::<Vec<_>>();
    line_scores.sort();
    let middle_index = line_scores.len() / 2;
    Ok(line_scores[middle_index])
}

enum LineResult {
    Valid,
    Corrupted(char),
    Incomplete(Vec<char>),
}

impl LineResult {
    fn score(&self) -> u64 {
        match self {
            LineResult::Corrupted(')') => 3,
            LineResult::Corrupted(']') => 57,
            LineResult::Corrupted('}') => 1197,
            LineResult::Corrupted('>') => 25137,
            LineResult::Incomplete(completion) => completion_score(&completion),
            _ => 0,
        }
    }

    fn is_corrupted(&self) -> bool {
        match self {
            LineResult::Corrupted(_) => true,
            _ => false,
        }
    }
}

fn completion_score(completion: &Vec<char>) -> u64 {
    let mut score = 0;
    for c in completion.iter() {
        score *= 5;
        score += match c {
            ')' => 1,
            ']' => 2,
            '}' => 3,
            '>' => 4,
            _ => panic!("invalid character {} in completion", c),
        }
    }
    score
}

fn check_line(line: &str) -> Result<LineResult, String> {
    let mut state = VecDeque::with_capacity(line.len());
    for c in line.chars() {
        match c {
            '(' | '[' | '{' | '<' => state.push_back(c),
            ')' | ']' | '}' | '>' => {
                if let Some(last) = state.back() {
                    if matching_closing(*last)? == c {
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
        let completion = state
            .into_iter()
            .rev()
            .map(|c| matching_closing(c))
            .collect::<Result<Vec<char>, _>>()?;
        Ok(LineResult::Incomplete(completion))
    }
}

fn matching_closing(c: char) -> Result<char, String> {
    match c {
        '(' => Ok(')'),
        '[' => Ok(']'),
        '{' => Ok('}'),
        '<' => Ok('>'),
        _ => Err(format!("No closing character known for {}", c)),
    }
}
#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/samples/day10.txt");
    let result = part1(input);
    assert_eq!(result, Ok(26397));
}

#[test]
fn test_part2_sample() {
    let input = include_str!("inputs/samples/day10.txt");
    let result = part2(input);
    assert_eq!(result, Ok(288957));
}
