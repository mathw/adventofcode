use crate::day::{DayResult, PartResult};
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = parse_input(include_str!("inputs/day6.txt"))?;
    let day1 = simulate_fishes(&input, 80);
    return Ok(DayResult::new(
        PartResult::Success(format!("There are {} fish after 80 days", day1.len())),
        PartResult::NotImplemented,
    ));
}

type Fish = u8;

const FISH_SPAWN_DAY: Fish = 0;
const FISH_POST_SPAWN: Fish = 6;
const NEW_FISH: Fish = 8;

enum FishDayEvent {
    Ages,
    Produces,
}

fn lanternfish_day(fish: Fish) -> FishDayEvent {
    if fish == FISH_SPAWN_DAY {
        FishDayEvent::Produces
    } else {
        FishDayEvent::Ages
    }
}

fn simulate_fish(fish: Fish, days: usize) -> Vec<Fish> {
    let mut all_fish = vec![fish];
    for _ in 0..days {
        for index in 0..all_fish.len() {
            let fish = all_fish[index];
            match lanternfish_day(fish) {
                FishDayEvent::Ages => all_fish[index] = fish - 1,
                FishDayEvent::Produces => {
                    all_fish[index] = FISH_POST_SPAWN;
                    all_fish.push(NEW_FISH);
                }
            }
        }
    }
    all_fish
}

fn simulate_fishes(fishes: &Vec<Fish>, days: usize) -> Vec<Fish> {
    fishes
        .iter()
        .flat_map(|fish| simulate_fish(*fish, days))
        .collect()
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
    assert_eq!(result.len(), 2, "one extra fish should have been spawned");
    assert_eq!(result[0], 5, "the first fish should have 5 days to go");
    assert_eq!(result[1], 7, "the second fish should have 7 days to go");
}

#[test]
fn test_part1_sample_short() {
    let fish = vec![3, 4, 3, 1, 2];
    let fish = simulate_fishes(&fish, 18);
    assert_eq!(fish.len(), 26);
}

#[test]
fn test_part1_sample_long() {
    let fish = vec![3, 4, 3, 1, 2];
    let fish = simulate_fishes(&fish, 80);
    assert_eq!(fish.len(), 5934);
}
