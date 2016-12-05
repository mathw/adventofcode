extern crate regex;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate itertools;
extern crate md5;
extern crate rustc_serialize;

mod dayfive;

fn main() {
    dayfive::do_dayfive();
}
