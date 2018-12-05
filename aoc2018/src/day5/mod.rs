use crate::day::Day;
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Sender;

pub struct Day5 {
    units: Vec<Unit>,
}

impl Day5 {
    pub fn new() -> Option<Day5> {
        let input = include_str!("input.txt");
        Some(Day5 {
            units: parse_input(input),
        })
    }
}

impl Day for Day5 {
    fn part1(&mut self, sender: &Sender<String>) {
        let result = cycle_until_done(&self.units);
        sender
            .send(format!("Result has {} units", result.len()))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let unique_unit_types = &self
            .units
            .iter()
            .map(|u| u.family)
            .collect::<HashSet<char>>();

        let mut results = HashMap::new();

        for u in unique_unit_types {
            sender.send(format!("Trying {}", u)).unwrap();

            let units = self
                .units
                .iter()
                .filter(|unit| unit.family != *u)
                .cloned()
                .collect::<Vec<Unit>>();
            let result = cycle_until_done(&units);
            results.insert(result.len(), u);
        }

        let shortest = results.keys().min().unwrap();

        sender
            .send(format!(
                "The shortest polymer I can make is {} which I got by removing {}",
                shortest, results[shortest]
            ))
            .unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Polarity {
    Upper,
    Lower,
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct Unit {
    family: char,
    polarity: Polarity,
}

impl Polarity {
    fn from_char(c: char) -> Polarity {
        if c.is_uppercase() {
            Polarity::Upper
        } else {
            Polarity::Lower
        }
    }
}

impl Unit {
    fn from_char(c: char) -> Unit {
        Unit {
            family: c.to_ascii_lowercase(),
            polarity: Polarity::from_char(c),
        }
    }

    fn inverse_of(&self, other: &Unit) -> bool {
        self.family == other.family && self.polarity != other.polarity
    }
}

fn parse_input(input: &str) -> Vec<Unit> {
    input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(Unit::from_char)
        .collect()
}

fn cycle(source: &[Unit]) -> Vec<Unit> {
    let mut result = Vec::new();

    let mut last_unit = None;

    for unit in source {
        match last_unit {
            None => {
                last_unit = Some(*unit);
            }
            Some(u) => {
                if u.inverse_of(unit) {
                    last_unit = None;
                } else {
                    result.push(u);
                    last_unit = Some(*unit);
                }
            }
        }
    }

    match last_unit {
        Some(u) => result.push(u),
        None => {}
    }

    result
}

fn cycle_until_done(source: &[Unit]) -> Vec<Unit> {
    let mut last = cycle(source);
    let mut next = cycle(&last);

    while last != next {
        last = next;
        next = cycle(&last);
    }

    last
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cycle_simple() {
        let source = parse_input("Aabb");
        let result = cycle(&source);
        assert_eq!(result, parse_input("bb"));
    }

    #[test]
    fn cycle_examples() {
        fn test(input: &str, expected: &str) {
            let source = parse_input(input);
            let result = cycle(&source);
            assert_eq!(
                result,
                parse_input(expected),
                "{} becomes {}",
                input,
                expected
            );
        }

        test("Aa", "");
        test("abBA", "aA");
        test("aBAb", "aBAb");
        test("aaBAAb", "aaBAAb");
    }

    #[test]
    fn part1_example() {
        let input = parse_input("dabAcCaCBAcCcaDA");
        let result = cycle_until_done(&input);
        assert_eq!(cycle_until_done(&input), parse_input("dabCBAcaDA"));
        assert_eq!(result.len(), 10);
    }
}
