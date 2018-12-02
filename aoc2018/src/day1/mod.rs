use crate::day::Day;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day1 {
    input: &'static str,
}

impl Day1 {
    pub fn new() -> Day1 {
        Day1 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day1 {
    fn part1(&mut self, sender: &Sender<String>) {
        let parsed = parse_input(self.input).expect("Unable to parse input");

        let result: i32 = parsed.iter().sum();

        sender
            .send(format!("The final frequency is {}", result))
            .expect("Unable to send result to sender");
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let parsed = parse_input(self.input).expect("Unable to parse input");

        let result = part_two(&parsed);

        sender
            .send(format!("The first frequency reached twice is {}", result))
            .expect("Unable to send result to sender");
    }
}

fn parse_input(input: &str) -> Result<Vec<i32>, String> {
    let mut output = Vec::new();
    for line in input.lines().map(|l| i32::from_str(l.trim())) {
        match line {
            Ok(num) => output.push(num),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(output)
}

fn part_two(instructions: &Vec<i32>) -> i32 {
    let mut seen: HashSet<i32> = HashSet::new();

    let mut current = 0;

    let mut looper = Looper::new(instructions.clone());

    loop {
        let instruction = looper.next();
        current += instruction;

        if seen.contains(&current) {
            return current;
        }

        seen.insert(current);
    }
}

struct Looper<T> {
    vector: Vec<T>,
    current: usize,
}

impl<T> Looper<T>
where
    T: Clone,
{
    fn new(contents: Vec<T>) -> Looper<T> {
        Looper {
            vector: contents,
            current: 0,
        }
    }

    fn next(&mut self) -> T {
        let yielded = self.vector[self.current].clone();

        self.current = (self.current + 1) % self.vector.len();

        yielded
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_input() {
        let out = parse_input("+1\n-2\n-4");

        assert_eq!(out, Ok(vec![1, -2, -4]));
    }

    #[test]
    fn part_two_example() {
        let result = part_two(&vec![1, -2, 3, 1]);

        assert_eq!(result, 2);
    }
}
