extern crate clap;

mod day1;

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

    if day < 1 || day > 25 {
        println!("Day must be from 1 to 25 inclusive");
    }

    print_day_header(day);

    match day {
        1 => day1::go(),
        _ => println!("I don't know how to do that day yet"),
    }
}

fn print_day_header(day: u8) {
    println!("Day {} coming right up...\n", day);
}