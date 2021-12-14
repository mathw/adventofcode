use crate::day::{DayResult, PartResult};
use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, error::Error};

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let sequence = parse_sequence(include_str!("inputs/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/day14/rules.txt"))?;

    let part1 = part1(&sequence, &rules);

    Ok(DayResult::new(
        PartResult::Success(format!("The answer is {}", part1)),
        PartResult::NotImplemented,
    ))
}

type Rules = HashMap<(char, char), char>;

fn apply_rules(sequence: &Vec<char>, rules: &Rules) -> Vec<char> {
    let last_char = sequence[sequence.len() - 1];
    let mut new_sequence = sequence
        .into_iter()
        .tuple_windows()
        .flat_map(|(&a, &b)| {
            if rules.contains_key(&(a, b)) {
                vec![a, rules[&(a, b)]]
            } else {
                vec![a]
            }
        })
        .collect::<Vec<char>>();
    new_sequence.push(last_char);
    new_sequence
}

fn parse_rule(line: &str) -> Result<((char, char), char), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.)(.) -> (.)").unwrap();
    }

    let caps = RE.captures(line).ok_or(format!("Bad input string"))?;
    Ok((
        (
            caps[1].chars().next().unwrap(),
            caps[2].chars().next().unwrap(),
        ),
        caps[3].chars().next().unwrap(),
    ))
}

fn parse_rules(rules: &str) -> Result<Rules, String> {
    rules.lines().map(|l| parse_rule(l.trim())).collect()
}

fn parse_sequence(sequence: &str) -> Vec<char> {
    sequence.trim().chars().collect()
}

fn character_frequencies(sequence: &Vec<char>) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for c in sequence {
        *map.entry(*c).or_default() += 1;
    }
    map
}

fn part1(sequence: &Vec<char>, rules: &Rules) -> usize {
    let mut current = sequence.clone();
    for _ in 0..10 {
        current = apply_rules(&current, rules);
    }
    let frequencies = character_frequencies(&current);
    let mut max_frequency = 0;
    let mut min_frequency = usize::MAX;

    for (_, f) in frequencies {
        if f < min_frequency {
            min_frequency = f;
        }
        if f > max_frequency {
            max_frequency = f;
        }
    }

    max_frequency - min_frequency
}

#[test]
fn test_part1_step1() {
    let sequence = parse_sequence(include_str!("inputs/samples/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/samples/day14/rules.txt")).unwrap();

    let result = apply_rules(&sequence, &rules);

    assert_eq!(result, vec!['N', 'C', 'N', 'B', 'C', 'H', 'B'])
}

#[test]
fn test_part1() {
    let sequence = parse_sequence(include_str!("inputs/samples/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/samples/day14/rules.txt")).unwrap();
    let result = part1(&sequence, &rules);
    assert_eq!(result, 1588);
}
