extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;

mod day7;

fn main() {
    day7::do_day7(include_str!("day7_input.txt"));
}
