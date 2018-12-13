mod node;

use crate::day::Day;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day8 {
    input: Vec<u8>,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            input: include_str!("input.txt")
                .split(' ')
                .filter_map(|w| u8::from_str(w).ok())
                .collect(),
        }
    }
}

impl Day for Day8 {
    fn part1(&mut self, _sender: &Sender<String>) {}

    fn part2(&mut self, _sender: &Sender<String>) {}
}
