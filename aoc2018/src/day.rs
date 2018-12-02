use std::sync::mpsc::Sender;

pub trait Day {
    fn part1(&mut self, sender: &Sender<String>);
    fn part2(&mut self, sender: &Sender<String>);
}
