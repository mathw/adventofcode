use crate::day::Day;
use std::sync::mpsc::Sender;

pub struct Day4 {
    input: &'static str,
}

impl Day4 {
    pub fn new() -> Day4 {
        Day4 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day4 {
    fn part1(&mut self, sender: &Sender<String>) {}

    fn part2(&mut self, sender: &Sender<String>) {}
}
