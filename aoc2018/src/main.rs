mod day1;
mod util;

use std::env::args;
use std::str::FromStr;
use util::ErrString;

fn main() -> Result<(), String> {
    println!("Advent of Code 2018");

    let args = args().collect::<Vec<_>>();

    if args.len() < 2 {
        Err("You must supply a day number to run a puzzle".into())
    } else {
        let day = usize::from_str(&args[1]).err_string()?;
        println!("You have requested day {}", day);

        match day {
            1 => day1::run(),
            _ => Err(format!("I don't know how to be day {} yet", day)),
        }
    }
}
