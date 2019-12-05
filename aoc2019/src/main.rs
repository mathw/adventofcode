mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod intcode;

use std::env::args;
use std::str::FromStr;

fn main() -> Result<(), String> {
    let mut args = args();

    if args.len() != 2 {
        // run all days!
        let last_known_day = 5;
        for day in 1..=last_known_day {
            println!("Running Day {}/{}...", day, last_known_day);
            run_day(day)?;
        }
        return Ok(());
    }

    let requested_day = u8::from_str(
        args.nth(1)
            .expect("We already checked there was an argument though")
            .as_str(),
    )
    .map_err(|e| e.to_string())?;

    run_day(requested_day)
}

fn run_day(day: u8) -> Result<(), String> {
    match day {
        1 => day01::run(),
        2 => day02::run(),
        3 => day03::run(),
        4 => day04::run(),
        5 => day05::run(),
        _ => Err("I don't know what that day is".into()),
    }
}
