mod day;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day11;
mod day12;
mod day13;
mod day14;
mod intcode;

use crate::day::Day;
use std::env::args;
use std::str::FromStr;
use std::time::Instant;

fn main() -> Result<(), String> {
    let mut args = args();

    if args.len() != 2 {
        return Err("Please specify a day number to run".into());
    }

    let requested_day = u8::from_str(
        args.nth(1)
            .expect("We already checked there was an argument though")
            .as_str(),
    )
    .map_err(|e| e.to_string())?;

    let total_time = run_day(requested_day)?;

    println!("[{}ms] total", total_time);
    Ok(())
}

fn run_day(day: u8) -> Result<u128, String> {
    let (mut day, construct_time) = timed_result(|| make_day(day))?;
    println!("[{}ms] construct", construct_time);

    let (part1, part1_time) = timed_result(|| day.part1())?;
    println!("[{}ms] 1: {}", part1_time, part1);

    let (part2, part2_time) = timed_result(|| day.part2())?;
    println!("[{}ms] 2: {}", part2_time, part2);

    Ok(construct_time + part1_time + part2_time)
}

fn make_day(day: u8) -> Result<Box<dyn Day>, String> {
    match day {
        1 => Ok(Box::new(day01::Day1::new())),
        2 => Ok(Box::new(day02::Day2::new()?)),
        3 => Ok(Box::new(day03::Day3::new())),
        4 => Ok(Box::new(day04::Day4::new())),
        5 => Ok(Box::new(day05::Day5::new()?)),
        6 => Ok(Box::new(day06::Day6::new()?)),
        7 => Ok(Box::new(day07::Day7::new()?)),
        8 => Ok(Box::new(day08::Day8::new()?)),
        9 => Ok(Box::new(day09::Day9::new()?)),
        11 => Ok(Box::new(day11::Day11::new()?)),
        12 => Ok(Box::new(day12::Day12::new()?)),
        13 => Ok(Box::new(day13::Day13::new()?)),
        14 => Ok(Box::new(day14::Day14::new()?)),
        _ => Err(format!("I don't know how to make day {} yet", day)),
    }
}

fn timed<W, R>(mut work: W) -> (R, u128)
where
    W: FnMut() -> R,
{
    let timer = Instant::now();
    (work(), timer.elapsed().as_millis())
}

fn timed_result<W, S, E>(work: W) -> Result<(S, u128), E>
where
    W: FnMut() -> Result<S, E>,
{
    let (result, time) = timed(work);
    result.map(|s| (s, time))
}
