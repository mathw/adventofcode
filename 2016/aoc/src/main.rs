extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;

mod day6;

fn main() {
    day6::do_day6(include_str!("day6_input.txt"));
}
