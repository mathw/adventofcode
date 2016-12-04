extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;

mod dayfour;
use dayfour::do_dayfour;

fn main() {
    do_dayfour();
}
