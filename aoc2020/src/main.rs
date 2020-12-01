mod day1;
mod dayerror;

use std::{env::args, error::Error, fmt, str::FromStr};

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = args();
    let daynum = args.nth(1).expect("Expected a day number argument");
    let day: u8 =
        u8::from_str(&daynum).expect(&format!("Expected day number {} to be a u8", daynum));

    let part1 = match day {
        1 => crate::day1::part1()?,
        d => return Err(BadDayError::boxed(d)),
    };

    println!("[1]: {}", part1);

    let part2 = match day {
        1 => crate::day1::part2()?,
        d => return Err(BadDayError::boxed(d)),
    };

    println!("[2]: {}", part2);

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
