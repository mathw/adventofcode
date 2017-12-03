extern crate clap;

mod util;
mod day1;
mod day2;
mod day3;

use clap::{Arg, App};
use std::str::FromStr;
use std::time::Instant;
use util::asmillis::AsMillis;

fn main() {
    let matches = App::new("Advent of Code 2015")
        .version("1.0")
        .author("Matthew Walton")
        .arg(Arg::with_name("DAY")
            .help("Chooses which day's problem to run")
            .required(true)
            .index(1))
        .arg(Arg::with_name("REPS")
            .help("How many repetitions to run each part through for timing purposes")
            .required(false)
            .index(2))
        .get_matches();

    let day = u8::from_str(matches.value_of("DAY").expect("Day must be specified"))
        .expect("Day must be a u8");

    if day < 1 || day > 25 {
        panic!("Day must be from 1 to 25 inclusive");
    }

    let reps = matches.value_of("REPS").and_then(|r| usize::from_str(r).ok()).unwrap_or(1);

    if reps < 1 {
        panic!("There's really no point doing anything 0 times you know.");
    }

    print_day_header(day, reps);

    let start = Instant::now();
    match day {
        1 => day1::go(reps),
        2 => day2::go(reps),
        3 => day3::go(reps),
        _ => println!("I don't know how to do that day yet"),
    }

    let time_taken = start.elapsed().as_millis();

    println!("Execution complete in {}ms", time_taken);
}

fn print_day_header(day: u8, count: usize) {
    if count == 1 {
        println!("Day {} coming right up...\n", day);
    } else {
        println!("Day {}, with {} repetitions...\n", day, count);
    }
}