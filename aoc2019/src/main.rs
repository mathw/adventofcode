mod day01;

use std::env::args;
use std::str::FromStr;

fn main() -> Result<(), String> {
    let mut args = args();

    if args.len() != 2 {
        return Err("Provide a day number as argument".into());
    }

    let requested_day = u8::from_str(
        args.nth(1)
            .expect("We already checked there was an argument though")
            .as_str(),
    )
    .map_err(|e| e.to_string())?;

    match requested_day {
        1 => day01::run(),
        _ => Err("I don't know what that day is".into()),
    }
}
