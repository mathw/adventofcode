extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;

mod day8;

fn main() {
    let input = include_str!("day8_input.txt");

    day8::do_day8(input);
}
