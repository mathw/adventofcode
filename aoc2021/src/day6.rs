use crate::day::{DayResult, PartResult};
use rayon::prelude::*;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = parse_input(include_str!("inputs/day6.txt"))?;
    let day1 = simulate_fishes(&input, 80);
    return Ok(DayResult::new(
        PartResult::Success(format!("There are {} fish after 80 days", day1)),
        PartResult::NotImplemented,
    ));
}

type Fish = u8;

const FISH_POST_SPAWN: Fish = 6;
const NEW_FISH: Fish = 8;

fn simulate_fish(fish: Fish, days: usize) -> u64 {
    if days < fish as usize {
        return 1;
    }
    // jump to next spawning day
    let days = days - fish as usize;
    if days == 0 {
        return 1;
    }
    // run this fish and the new fish
    simulate_fish(FISH_POST_SPAWN, days - 1) + simulate_fish(NEW_FISH, days - 1)
}

fn simulate_fishes(fishes: &Vec<Fish>, days: usize) -> u64 {
    fishes.iter().map(|fish| simulate_fish(*fish, days)).sum()
}

fn parse_input(input: &str) -> Result<Vec<Fish>, impl Error> {
    input
        .split(',')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .map(|x| u8::from_str(x))
        .collect::<Result<Vec<_>, _>>()
}

#[test]
fn test_one_sample_fish() {
    let result = simulate_fish(3, 5);
    assert_eq!(result, 2, "one extra fish should have been spawned");
}

#[test]
fn test_part1_sample_short() {
    let fish = vec![3, 4, 3, 1, 2];
    let fish = simulate_fishes(&fish, 18);
    assert_eq!(fish, 26);
}

#[test]
fn test_part1_sample_long() {
    let fish = vec![3, 4, 3, 1, 2];
    let fish = simulate_fishes(&fish, 80);
    assert_eq!(fish, 5934);
}

// #[test]
// fn test_part2_sample() {
//     let fish = vec![3];
//     let fish = simulate_fishes(&fish, 256);
//     assert_eq!(fish, 26984457539);
// }
