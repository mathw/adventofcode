mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod dayerror;

#[macro_use]
extern crate lazy_static;

use std::{env::args, error::Error, fmt, str::FromStr, time::Instant};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    let daynum = args.nth(1).expect("Expected a day number argument");
    let day: u8 =
        u8::from_str(&daynum).expect(&format!("Expected day number {} to be a u8", daynum));

    let part1_start = Instant::now();
    let part1 = match day {
        1 => crate::day1::part1()?,
        2 => crate::day2::part1()?,
        3 => crate::day3::part1()?,
        4 => crate::day4::part1()?,
        5 => crate::day5::part1()?,
        6 => crate::day6::part1()?,
        7 => crate::day7::part1()?,
        d => return Err(BadDayError::boxed(d)),
    };
    let part1_duration = part1_start.elapsed();

    println!("[Part 1 in {}ms]: {}", part1_duration.as_millis(), part1);

    let part2_start = Instant::now();
    let part2 = match day {
        1 => crate::day1::part2()?,
        2 => crate::day2::part2()?,
        3 => crate::day3::part2()?,
        4 => crate::day4::part2()?,
        5 => crate::day5::part2()?,
        6 => crate::day6::part2()?,
        d => return Err(BadDayError::boxed(d)),
    };
    let part2_duration = part2_start.elapsed();

    println!("[Part 2 in {}ms]: {}", part2_duration.as_millis(), part2);

    Ok(())
}

#[derive(Debug)]
struct BadDayError {
    day: u8,
}

impl BadDayError {
    fn boxed(day: u8) -> Box<BadDayError> {
        Box::new(BadDayError { day })
    }
}

impl fmt::Display for BadDayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown or bad day {}", self.day)
    }
}

impl Error for BadDayError {}
