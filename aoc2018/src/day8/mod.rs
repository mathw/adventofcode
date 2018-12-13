mod node;

use self::node::Node;
use crate::day::Day;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day8 {
    input: &'static str,
}

impl Day8 {
    pub fn new() -> Day8 {
        Day8 {
            input: include_str!("input.txt"),
        }
    }
}

impl Day for Day8 {
    fn part1(&mut self, sender: &Sender<String>) {
        match sum_metadata(self.input) {
            Err(e) => sender.send(format!("Error parsing nodes: {}", e)).unwrap(),
            Ok(sum) => {
                sender.send(format!("The metadata sum is {}", sum)).unwrap();
            }
        }
    }

    fn part2(&mut self, sender: &Sender<String>) {
        match node_value(self.input) {
            Err(e) => sender.send(format!("Error parsing nodes: {}", e)).unwrap(),
            Ok(value) => sender.send(format!("The node value is {}", value)).unwrap(),
        }
    }
}

fn sum_metadata(input: &str) -> Result<u16, String> {
    let nums = input
        .trim()
        .split(' ')
        .filter_map(|w| u8::from_str(w).ok())
        .collect::<Vec<u8>>();
    let node = Node::from_u8(&mut nums.into_iter())?;

    Ok(node.sum_metadata())
}

fn node_value(input: &str) -> Result<u16, String> {
    let nums = input
        .trim()
        .split(' ')
        .filter_map(|w| u8::from_str(w).ok())
        .collect::<Vec<u8>>();
    let node = Node::from_u8(&mut nums.into_iter())?;

    Ok(node.value())
}

#[test]
fn example_part_one() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    ";

    let result = sum_metadata(input);

    assert_eq!(result, Ok(138));
}

#[test]
fn example_part_two() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
    ";

    let result = node_value(input);

    assert_eq!(result, Ok(66));
}
