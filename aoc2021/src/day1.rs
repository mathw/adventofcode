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

        let part1_answer = count_increases(input.iter().cloned());
        let part2_windows = sliding_window_sums(&input);
        let part2_answer = count_increases(part2_windows.into_iter());
        return Ok(DayResult::new(
            PartResult::Success(format!("{} increases", part1_answer)),
            PartResult::Success(format!("{} increases", part2_answer)),
        ));
    }
}

fn parse_input(input: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    return input
        .lines()
        .filter(|l| l.len() > 0)
        .map(|s| u32::from_str(s.trim()))
        .collect::<Result<_, _>>();
}

fn count_increases<'a>(input: impl Iterator<Item = u32>) -> usize {
    let mut last: Option<u32> = None;
    let mut count = 0;

    for n in input {
        match last {
            None => last = Some(n),
            Some(l) => {
                if n > l {
                    count += 1;
                }
                last = Some(n);
            }
        }
    }

    return count;
}

fn sliding_window_sums(input: &[u32]) -> Vec<u32> {
    input.windows(3).map(|w| w.iter().sum()).collect()
}

#[test]
fn test_part1_sample() {
    let input =
        parse_input(include_str!("inputs/day1-sample.txt")).expect("Sample input should parse");
    let result = count_increases(input.into_iter());
    assert_eq!(result, 7);
}

#[test]
fn test_part2_sample() {
    let input =
        parse_input(include_str!("inputs/day1-sample.txt")).expect("Sample input should parse");
    let window_sums = sliding_window_sums(&input);
    let result = count_increases(window_sums.into_iter());
    assert_eq!(result, 5);
}
