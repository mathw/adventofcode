extern crate assembly;
extern crate util;

use util::timed;

fn main() {
    let input = include_str!("input.txt");

    let (result, time) = timed(|| part1(input));
    println!("[{}ms] mul was called {} times", time, result);
}

fn part1(input: &str) -> usize {
    let result = assembly::run_for_day_23_part_one(input);

    result.unwrap()
}
