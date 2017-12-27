extern crate util;

mod grid;
mod agent;

use util::timed;
use grid::Grid;
use agent::Agent;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");

    let (result, time) = timed(|| part1(input));
    println!("[{}ms] {}", time, result);

    let (result, time) = timed(|| part2(input));
    println!("[{}ms] {}", time, result);
}

fn part1(input: &str) -> usize {
    let mut count = 0;

    let grid = Grid::from_str(input).expect("Couldn't parse grid");
    let mut agent = Agent::new(grid);

    for _ in 0..10_000 {
        if agent.step() {
            count += 1;
        }
    }

    count
}

fn part2(input: &str) -> usize {
    let mut count = 0;

    let grid = Grid::from_str(input).expect("Couldn't parse grid");
    let mut agent = Agent::new(grid);

    for _ in 0..10_000_000 {
        if agent.step_part_two() {
            count += 1;
        }
    }

    count
}
