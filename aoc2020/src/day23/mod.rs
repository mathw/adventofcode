mod ring;

use self::ring::Ring;
use crate::dayerror::DayError;
use std::{fmt::Debug, fmt::Display, hash::Hash, str::FromStr};

pub fn part1(visualise: bool) -> Result<String, DayError> {
    let answer = run_part1("368195742", visualise)?;
    Ok(format!(
        "The answer to this silly crab's game is {}",
        answer
    ))
}

pub fn part2(visualise: bool) -> Result<String, DayError> {
    let answer = run_part2("368195742", visualise)?;
    Ok(format!(
        "The answer to this silly crab's even sillier game is {}",
        answer
    ))
}

fn make_move<T>(cups: &mut Ring<T>, current_cup: T, visualise: bool) -> T
where
    T: Debug + Display + Eq + PartialOrd + CheckedDecrement + Copy + Hash,
{
    if visualise {
        println!(
            "BEGIN MOVE:\nCurrent cup: {}\nCircle: {:?}",
            current_cup, cups
        );
    }
    if !cups.contains(&current_cup) {
        panic!(
            "Attempt to make a move when the current cup {} does not exist in the circle!",
            current_cup
        );
    }

    let removed = cups
        .remove_three_after(&current_cup)
        .expect("Don't call this on a ring without enough cups in it!");
    if visualise {
        println!("Removed {:?}", removed);
    }

    let destination = find_destination_cup(cups, &current_cup);
    if visualise {
        println!("Destination cup is {}", destination);
    }

    cups.insert_three_after(&destination, removed);
    if visualise {
        println!("Move complete: {:?}", cups);
    }

    let new_current_cup = cups
        .value_after(&current_cup)
        .expect("There should always be a new current cup")
        .clone();

    if visualise {
        println!("New current cup: {}", new_current_cup);
    }

    new_current_cup
}

fn find_destination_cup<T>(cups: &Ring<T>, current_cup: &T) -> T
where
    T: PartialOrd + CheckedDecrement + Eq + Copy + Hash,
{
    let minus_one = current_cup.checked_decrement();
    match minus_one {
        None => (cups
            .highest_value()
            .expect("Don't call this on an empty circle!"))
        .clone(),
        Some(n) => {
            if cups.contains(&n) {
                n
            } else {
                find_destination_cup(cups, &n)
            }
        }
    }
}

fn make_moves<T>(cups: &mut Ring<T>, current_cup: T, moves: usize, visualise: bool)
where
    T: Debug + Display + Eq + PartialOrd + CheckedDecrement + Copy + Hash,
{
    let mut current_cup = current_cup;
    for m in 1..=moves {
        if visualise {
            println!("** MOVE {} **", m);
        }
        current_cup = make_move(cups, current_cup, visualise);
    }
}

fn gather_order(cups: &Ring<u8>) -> Vec<u8> {
    let ordered_values = cups.iter().collect::<Vec<&u8>>();
    let index_of_one = ordered_values
        .iter()
        .enumerate()
        .filter(|(_, v)| v == &&&1)
        .next()
        .expect("1 must be in the circle")
        .0;
    let mut ordered_cups_from_one = Vec::new();
    for n in 0..ordered_values.len() {
        ordered_cups_from_one
            .push(ordered_values[(n + index_of_one) % ordered_values.len()].clone());
    }
    ordered_cups_from_one
}

fn make_answer(ordered_cups: &Vec<u8>) -> String {
    ordered_cups
        .iter()
        .map(|i| i.to_string())
        .collect::<String>()
        .strip_prefix("1")
        .expect("I expected a 1 on the start of the ordered list")
        .into()
}

fn run_part1(input: &str, visualise: bool) -> Result<String, DayError> {
    let cups = input
        .chars()
        .map(|c| u8::from_str(&c.to_string()))
        .collect::<Result<Vec<u8>, _>>()?;
    let current_cup = cups[0];
    let mut cups = Ring::new(cups);
    make_moves(&mut cups, current_cup, 100, visualise);
    Ok(make_answer(&gather_order(&cups)))
}

fn two_cups_after_one(cups: &Ring<u32>) -> (u32, u32) {
    let first = cups.value_after(&1).unwrap();
    let second = cups.value_after(first).unwrap();
    (*first, *second)
}

fn run_part2(input: &str, visualise: bool) -> Result<u64, DayError> {
    let mut cups = input
        .chars()
        .map(|c| u32::from_str(&c.to_string()))
        .collect::<Result<Vec<u32>, _>>()?;
    let highest_cup = cups
        .iter()
        .fold(0, |acc, x| if *x > acc { *x } else { acc });
    let mut current = highest_cup + 1;
    while current <= 1_000_000 {
        cups.push(current);
        current += 1;
    }
    assert_eq!(cups.len(), 1_000_000);
    let current_cup = cups[0];
    let mut cups = Ring::new(cups);
    make_moves(&mut cups, current_cup, 10_000_000, visualise);
    let (first, second) = two_cups_after_one(&cups);
    Ok(first as u64 * second as u64)
}

#[test]
fn test_move() {
    let mut cups = Ring::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let current_cup = make_move(&mut cups, 3u8, true);
    assert_eq!(format!("{:?}", cups), "[3, 2, 8, 9, 1, 5, 4, 6, 7, ]");
    assert_eq!(current_cup, 2);
    let current_cup = make_move(&mut cups, current_cup, true);
    assert_eq!(format!("{:?}", cups), "[3, 2, 5, 4, 6, 7, 8, 9, 1, ]");
    assert_eq!(current_cup, 5);
}

#[test]
fn test_sample_part1() {
    let answer = run_part1("389125467", true).unwrap();
    assert_eq!(answer, "67384529");
}

trait CheckedDecrement {
    fn checked_decrement(self) -> Option<Self>
    where
        Self: Sized;
}

impl CheckedDecrement for u8 {
    fn checked_decrement(self) -> Option<Self> {
        self.checked_sub(1)
    }
}

impl CheckedDecrement for u32 {
    fn checked_decrement(self) -> Option<Self> {
        self.checked_sub(1)
    }
}

#[test]
fn test_sample_part2() {
    let answer = run_part2("389125467", false).unwrap();
    assert_eq!(answer, 149245887792);
}
