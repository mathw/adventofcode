extern crate clap;
extern crate md5;
#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use clap::{Arg, App};
use std::str::FromStr;

fn main() {
    let matches = App::new("Advent of Code 2015")
        .version("1.0")
        .author("Matthew Walton")
        .arg(Arg::with_name("DAY")
            .help("Chooses which day's problem to run")
            .required(true)
            .index(1))
        .get_matches();

    let day = u8::from_str(matches.value_of("DAY").expect("Day must be specified"))
        .expect("Day must be a u8");

    print_day_header(day as u32);

    match day {
        1 => day1::do_dayone(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        _ => panic!("Unknown day"),
    }
}

fn print_day_header(day: u32) {
    println!("Day {} coming right up...\n", day);
}
