use crate::day::Day;
use crate::day::DayResult;
use clap::{App, Arg};
use std::error::Error;

mod common;
mod day;
mod day1;
mod day2;

fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();

    let matches = App::new("Advent of Code 2021")
        .version("1.0")
        .author("Matthew Walton")
        .about("Solves Advent of Code 2021 problems")
        .arg(
            Arg::with_name("DAY")
                .help("Chooses which day to run")
                .required(true)
                .index(1),
        )
        .get_matches();

    let day = matches.value_of("DAY").expect("Day must be provided");

    match day {
        "1" => run_day(1, || {
            let mut day = day1::Day1::new();
            day.run()
        }),
        "2" => {
            run_day(2, || day2::run());
        }
        _ => log::error!("Unimplemented day {}", day),
    }
}

fn run_day(day_num: u8, day_func: impl Fn() -> Result<DayResult, Box<dyn Error>>) {
    log::info!("Starting day {}", day_num);
    match day_func() {
        Ok(r) => log::info!("Day {} result:\n{}", day_num, r),
        Err(e) => log::error!("{}", e),
    }
}
