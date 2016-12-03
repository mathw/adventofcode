extern crate regex;
#[macro_use]
extern crate lazy_static;

mod geometry;
mod daythree;

use daythree::{do_daythree, do_daythree_parttwo};

fn main() {
    do_daythree();
    do_daythree_parttwo();
}
