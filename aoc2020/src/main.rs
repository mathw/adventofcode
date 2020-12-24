mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day22;
mod day23;
mod day24;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod dayerror;
mod grid3d;
mod grid4d;
mod interpreter;
mod point3d;
mod point4d;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate lalrpop_util;

use crate::dayerror::DayError;
use clap::{App, Arg};
use std::{io, num::ParseIntError, str::FromStr, time::Instant};
use thiserror::Error;
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), ApplicationError> {
    let matches = App::new("Advent of Code 2020")
        .version("0.0")
        .author("Matthew Walton")
        .about("Solutions to Advent of Code 2020 puzzles")
        .arg(
            Arg::with_name("DAY")
                .help("Specifies which day to execute")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("part")
                .short("p")
                .long("part")
                .value_name("PART")
                .help("Specifies which part to execute (both will run if not specified)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("visualise")
                .short("v")
                .long("visualise")
                .help("If present, days with optional visualisations will run them"),
        )
        .get_matches();
    let daynum = matches.value_of("DAY").unwrap();
    let partnum = matches.value_of("part").unwrap_or("3");
    let do_visualisation = matches.occurrences_of("visualise") == 1;
    let day: u8 =
        u8::from_str(&daynum).expect(&format!("Expected day number {} to be a u8", daynum));
    let part: u8 = u8::from_str(&partnum)?;

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    println!(
        "Running day {} part {} {} visualisation",
        day,
        if part == 3 {
            "1 + 2".to_string()
        } else {
            part.to_string()
        },
        if do_visualisation { "with" } else { "without" }
    );

    if part == 1 || part == 3 {
        let part1_start = Instant::now();
        let part1 = match day {
            1 => crate::day1::part1()?,
            2 => crate::day2::part1()?,
            3 => crate::day3::part1()?,
            4 => crate::day4::part1()?,
            5 => crate::day5::part1()?,
            6 => crate::day6::part1()?,
            7 => crate::day7::part1()?,
            8 => crate::day8::part1()?,
            9 => crate::day9::part1()?,
            10 => crate::day10::part1()?,
            11 => crate::day11::part1(&mut terminal, do_visualisation)?,
            12 => crate::day12::part1()?,
            13 => crate::day13::part1()?,
            14 => crate::day14::part1()?,
            15 => crate::day15::part1()?,
            16 => crate::day16::part1()?,
            17 => crate::day17::part1()?,
            18 => crate::day18::part1()?,
            19 => crate::day19::part1()?,
            22 => crate::day22::part1()?,
            23 => crate::day23::part1(do_visualisation)?,
            24 => crate::day24::part1()?,
            d => return Err(ApplicationError::BadDayError(BadDayError(d))),
        };
        let part1_duration = part1_start.elapsed();

        println!("[Part 1 in {}ms]: {}", part1_duration.as_millis(), part1);
    }

    if part == 2 || part == 3 {
        let part2_start = Instant::now();
        let part2 = match day {
            1 => crate::day1::part2()?,
            2 => crate::day2::part2()?,
            3 => crate::day3::part2()?,
            4 => crate::day4::part2()?,
            5 => crate::day5::part2()?,
            6 => crate::day6::part2()?,
            7 => crate::day7::part2()?,
            8 => crate::day8::part2()?,
            9 => crate::day9::part2()?,
            10 => crate::day10::part2()?,
            11 => crate::day11::part2(&mut terminal, do_visualisation)?,
            12 => crate::day12::part2()?,
            13 => crate::day13::part2()?,
            14 => crate::day14::part2()?,
            15 => crate::day15::part2()?,
            16 => crate::day16::part2()?,
            17 => crate::day17::part2()?,
            18 => crate::day18::part2()?,
            19 => crate::day19::part2()?,
            22 => crate::day22::part2(do_visualisation)?,
            23 => crate::day23::part2(do_visualisation)?,
            24 => crate::day24::part2()?,
            d => return Err(ApplicationError::BadDayError(BadDayError(d))),
        };
        let part2_duration = part2_start.elapsed();

        println!("[Part 2 in {}ms]: {}", part2_duration.as_millis(), part2);
    }

    Ok(())
}

#[derive(Debug, Error)]
enum ApplicationError {
    #[error(transparent)]
    DayError(#[from] DayError),
    #[error(transparent)]
    BadDayError(#[from] BadDayError),
    #[error(transparent)]
    IoError(#[from] io::Error),
    #[error(transparent)]
    CoreIntParsingError(#[from] ParseIntError),
}

#[derive(Debug, Error)]
#[error("Unknown or invalid day")]
struct BadDayError(u8);
