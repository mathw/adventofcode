extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;
#[macro_use]
extern crate nom;

mod day9;

fn main() {
    let input = include_str!("day9_input.txt");
    day9::do_day9(input);
    // day9::part2::do_day9(input);
}
