use crate::day::{DayResult, PartResult};
use memoise::memoise;
use rayon::prelude::*;
use std::collections::HashMap;
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let positions = parse_input(include_str!("inputs/day7.txt"))?;
    let part1 =
        part1(&positions).ok_or("Unable to find a least fuel position for part 1".to_owned())?;
    let part2 =
        part2(&positions).ok_or("Unable to find a least fuel position for part 2".to_owned())?;

    Ok(DayResult::new(
        PartResult::Success(format!("Least fuel is {}", part1)),
        PartResult::Success(format!("Least fuel by the proper rules is {}", part2)),
    ))
}

fn part1(positions: &Vec<u32>) -> Option<u32> {
    try_find_least_fuel(positions, fuel_to)
}

fn part2(positions: &Vec<u32>) -> Option<u32> {
    try_find_least_fuel(positions, part2_fuel_to)
}

fn parse_input(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    Ok(input
        .split(',')
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?)
}

fn try_find_least_fuel(
    positions: &Vec<u32>,
    fuel_to: impl Fn(u32, &Vec<u32>) -> u32 + Send + Sync,
) -> Option<u32> {
    let max_position = *positions.iter().max()?;
    let min_position = *positions.iter().min()?;

    let search_space = (min_position..=max_position).collect::<Vec<u32>>();

    let fuels = search_space
        .par_iter()
        .map(|pos| (*pos, fuel_to(*pos, positions)))
        .collect::<Vec<(u32, u32)>>();

    let least_fuel = fuels
        .into_iter()
        .min_by_key(|&(_, fuel)| fuel)
        .map(|(_, fuel)| fuel);

    least_fuel
}

fn fuel_to(target: u32, positions: &Vec<u32>) -> u32 {
    positions
        .iter()
        .map(|p| u32::max(target, *p) - u32::min(target, *p))
        .sum()
}

fn part2_fuel_to(target: u32, positions: &Vec<u32>) -> u32 {
    positions
        .iter()
        .map(|p| fuel_for_distance_part2(u32::max(target, *p) - u32::min(target, *p)))
        .sum()
}

#[memoise(distance < 3000)]
fn fuel_for_distance_part2(distance: u32) -> u32 {
    (1..=distance).sum()
}

#[test]
fn test_part1_sample() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let result = part1(&input).unwrap();
    assert_eq!(result, 37);
}

#[test]
fn test_fuel_to() {
    assert_eq!(fuel_to(2, &vec![16]), 14);
    assert_eq!(fuel_to(2, &vec![1]), 1);
    assert_eq!(fuel_to(2, &vec![2]), 0);
    assert_eq!(fuel_to(2, &vec![0]), 2);
    assert_eq!(fuel_to(2, &vec![4]), 2);
    assert_eq!(fuel_to(2, &vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 37);
}

#[test]
fn test_fuel_to_part2() {
    assert_eq!(part2_fuel_to(5, &vec![16]), 66);
    assert_eq!(part2_fuel_to(5, &vec![1]), 10);
    assert_eq!(part2_fuel_to(5, &vec![2]), 6);
    assert_eq!(part2_fuel_to(5, &vec![0]), 15);
    assert_eq!(part2_fuel_to(5, &vec![4]), 1);
    assert_eq!(part2_fuel_to(5, &vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14]), 168);
}

#[test]
fn test_part2_sample() {
    let input = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];
    let result = part2(&input).unwrap();
    assert_eq!(result, 168);
}
