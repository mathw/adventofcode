mod circle;

use self::circle::Circle;
use crate::day::Day;
use std::collections::HashMap;
use std::sync::mpsc::Sender;

pub struct Day9 {
    players: usize,
    last_marble_score: u32,
}

impl Day9 {
    pub fn new() -> Day9 {
        Day9 {
            players: 404,
            last_marble_score: 71852,
        }
    }
}

impl Day for Day9 {
    fn part1(&mut self, sender: &Sender<String>) {
        sender
            .send(format!(
                "{} elves until marble {}",
                self.players, self.last_marble_score
            ))
            .unwrap();
        let result = run_until_marble(self.players, self.last_marble_score);
        sender
            .send(format!("The winning elf's score is {}", result))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        sender
            .send(format!(
                "{} elves until marble {}",
                self.players,
                self.last_marble_score * 100
            ))
            .unwrap();
        let result = run_until_marble(self.players, self.last_marble_score * 100);
        sender
            .send(format!("The winning elf's score is {}", result))
            .unwrap();
    }
}

fn run_until_marble(elves: usize, marble: u32) -> u32 {
    // return the winning elf's score after running until we hit the marble with the given marble score

    // #[cfg(test)]
    // println!("{} elves until marble {}", elves, marble);

    let mut elf_scores = HashMap::new();
    let mut increase_elf_score = |elf: usize, score: u32| {
        let entry = elf_scores.entry(elf).or_insert(0);
        *entry += score;
    };

    let mut circle = Circle::new(marble as usize);
    let mut current_elf = 0;

    for iteration in 0..marble {
        let move_score = circle.add_new_marble();

        increase_elf_score(current_elf, move_score);

        // #[cfg(test)]
        // println!("Elf {} gains {} points", current_elf, move_score);

        current_elf = (current_elf + 1) % elves;
    }

    elf_scores.values().max().map(|x| *x).unwrap_or(0)
}

#[test]
fn part_one_example_one() {
    let score = run_until_marble(10, 1618);

    assert_eq!(score, 8317);
}

#[test]
fn part_one_example_two() {
    let score = run_until_marble(13, 7999);

    assert_eq!(score, 146373);
}

#[test]
fn part_one_example_three() {
    let score = run_until_marble(17, 1104);

    assert_eq!(score, 2764);
}

#[test]
fn part_one_example_four() {
    let score = run_until_marble(21, 6111);

    assert_eq!(score, 54718);
}

#[test]
fn part_one_example_five() {
    let score = run_until_marble(30, 5807);

    assert_eq!(score, 37305);
}
