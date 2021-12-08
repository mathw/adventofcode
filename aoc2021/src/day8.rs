use crate::day::{DayResult, PartResult};
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = include_str!("inputs/day8.txt");
    let part1 = part1(input)?;
    Ok(DayResult::new(
        PartResult::Success(format!("{} unique segment numbers", part1)),
        PartResult::NotImplemented,
    ))
}
fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let parsed = parse_input(input)?;
    Ok(parsed
        .into_iter()
        .flat_map(|readout| readout.displayed.into_iter().map(|s| s.is_unique_pattern()))
        .filter(|&x| x)
        .count())
}

struct Segments(HashMap<char, bool>);

impl Segments {
    fn is_unique_pattern(&self) -> bool {
        match self.num_segments_lit() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }

    fn num_segments_lit(&self) -> usize {
        self.0.iter().map(|(_, v)| if *v { 1 } else { 0 }).sum()
    }
}

impl Default for Segments {
    fn default() -> Self {
        Segments(HashMap::new())
    }
}

impl FromStr for Segments {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Segments::default();
        for c in s.chars() {
            match c {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => {
                    this.0.insert(c, true);
                }
                _ => {
                    return Err(format!(
                        "Character '{}' is not recognised as a display segment",
                        c
                    ))
                }
            }
        }
        Ok(this)
    }
}

struct Readout {
    displayed: Vec<Segments>,
}

fn parse_input_line(input: &str) -> Result<Readout, Box<dyn Error>> {
    let parts = input.split("|").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!("No delimiter found").into());
    }
    Ok(Readout {
        displayed: parts[1]
            .split_whitespace()
            .map(|s| Segments::from_str(s))
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn parse_input(input: &str) -> Result<Vec<Readout>, Box<dyn Error>> {
    input.lines().map(|l| parse_input_line(l)).collect()
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/samples/day8.txt");
    let result = part1(input).unwrap();
    assert_eq!(result, 26);
}
