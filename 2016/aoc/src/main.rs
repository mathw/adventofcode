extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate nom_test_helpers;

mod day10;

fn main() {
    let input = include_str!("day10_input.txt");

    day10::do_day10(input);
}
