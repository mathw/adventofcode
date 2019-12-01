use crate::day::Day;
use std::sync::mpsc::Sender;

pub struct Day14 {
    scoreboard: Vec<u8>,
    elf1_current_recipe_index: usize,
    elf2_current_recipe_index: usize,
}

impl Default for Day14 {
    fn default() -> Day14 {
        Day14 {
            scoreboard: vec![3, 7],
            elf1_current_recipe_index: 0,
            elf2_current_recipe_index: 1,
        }
    }
}

impl Day for Day14 {
    fn part1(&mut self, sender: &Sender<String>) {
        let result = self
            .ten_scores_after_iteration(919901)
            .iter()
            .map(|n| format!("{}", n))
            .collect::<String>();
        sender.send(result).unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let result = self.scores_to_left_of_sequence(&vec![9, 1, 9, 9, 0, 1]);
        sender.send(format!("{}", result)).unwrap();
    }
}

impl Day14 {
    fn combine_recipes(&mut self) {
        let recipe1 = self.scoreboard[self.elf1_current_recipe_index];
        let recipe2 = self.scoreboard[self.elf2_current_recipe_index];
        let combined = recipe1 + recipe2;

        if combined < 10 {
            self.scoreboard.push(combined);
        } else {
            let new1 = combined / 10;
            let new2 = combined % 10;
            self.scoreboard.push(new1);
            self.scoreboard.push(new2);
        }
    }

    fn advance_elves(&mut self) {
        let elf1_steps = self.scoreboard[self.elf1_current_recipe_index] as usize + 1;
        let elf2_steps = self.scoreboard[self.elf2_current_recipe_index] as usize + 1;

        self.elf1_current_recipe_index =
            (self.elf1_current_recipe_index + elf1_steps) % self.scoreboard.len();
        self.elf2_current_recipe_index =
            (self.elf2_current_recipe_index + elf2_steps) % self.scoreboard.len();
    }

    fn iterate(&mut self) {
        self.combine_recipes();
        self.advance_elves();
    }

    fn ten_scores_after_iteration(&mut self, iteration: usize) -> Vec<u8> {
        while self.scoreboard.len() < iteration + 10 {
            self.iterate();
        }

        self.scoreboard
            .iter()
            .skip(iteration)
            .take(10)
            .cloned()
            .collect()
    }

    fn scoreboard_contains_sequence(
        &self,
        sequence: &Vec<u8>,
        starting_at: usize,
    ) -> Option<usize> {
        if sequence.len() > self.scoreboard.len() - starting_at {
            return None;
        }

        for (index, score) in self
            .scoreboard
            .iter()
            .enumerate()
            .skip(starting_at)
            .take((self.scoreboard.len() - starting_at) - sequence.len())
        {
            if *score == sequence[0] {
                let potential = self.scoreboard[index..index + sequence.len()]
                    .iter()
                    .cloned()
                    .collect::<Vec<u8>>();
                if potential == *sequence {
                    return Some(index);
                }
            }
        }

        None
    }

    fn scores_to_left_of_sequence(&mut self, sequence: &Vec<u8>) -> usize {
        let mut last_start = 0;
        loop {
            if let Some(pos) = self.scoreboard_contains_sequence(sequence, last_start) {
                return pos;
            }

            last_start = self.scoreboard.len();

            for _ in 0..1000000 {
                self.iterate();
            }
        }
    }
}

#[test]
fn example_1() {
    let mut d = Day14::default();
    let result: String = d
        .ten_scores_after_iteration(9)
        .iter()
        .map(|n| format!("{}", n))
        .collect();
    assert_eq!(result, "5158916779");
}

#[test]
fn example_2() {
    let mut d = Day14::default();
    let result: String = d
        .ten_scores_after_iteration(5)
        .iter()
        .map(|n| format!("{}", n))
        .collect();
    assert_eq!(result, "0124515891");
}

#[test]
fn example_3() {
    let mut d = Day14::default();
    let result: String = d
        .ten_scores_after_iteration(18)
        .iter()
        .map(|n| format!("{}", n))
        .collect();
    assert_eq!(result, "9251071085");
}

#[test]
fn example_4() {
    let mut d = Day14::default();
    let result: String = d
        .ten_scores_after_iteration(2018)
        .iter()
        .map(|n| format!("{}", n))
        .collect();
    assert_eq!(result, "5941429882");
}

#[test]
fn part2_example1() {
    let mut d = Day14::default();
    let pos = d.scores_to_left_of_sequence(&vec![5, 1, 5, 8, 9]);

    assert_eq!(pos, 9);
}

#[test]
fn part2_example2() {
    let mut d = Day14::default();
    let pos = d.scores_to_left_of_sequence(&vec![0, 1, 2, 4, 5]);

    assert_eq!(pos, 5);
}

#[test]
fn part2_example3() {
    let mut d = Day14::default();
    let pos = d.scores_to_left_of_sequence(&vec![9, 2, 5, 1, 0]);

    assert_eq!(pos, 18);
}

#[test]
fn part2_example4() {
    let mut d = Day14::default();
    let pos = d.scores_to_left_of_sequence(&vec![5, 9, 4, 1, 4]);

    assert_eq!(pos, 2018);
}
