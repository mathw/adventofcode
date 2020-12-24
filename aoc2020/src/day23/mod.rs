mod ring;

use self::ring::Ring;
use crate::dayerror::DayError;
use std::str::FromStr;

pub fn part1(visualise: bool) -> Result<String, DayError> {
    let answer = run_part1("368195742", visualise)?;
    Ok(format!(
        "The answer to this silly crab's game is {}",
        answer
    ))
}

fn make_move(cups: &mut Ring<u8>, current_cup: u8, visualise: bool) -> u8 {
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

    let destination = find_destination_cup(cups, current_cup);
    if visualise {
        println!("Destination cup is {}", destination);
    }

    cups.insert_three_after(&destination, removed);
    if visualise {
        println!("Move complete: {:?}", cups);
    }

    let new_current_cup = *cups
        .value_after(&current_cup)
        .expect("There should always be a new current cup");

    if visualise {
        println!("New current cup: {}", new_current_cup);
    }

    new_current_cup
}

fn find_destination_cup(cups: &Ring<u8>, current_cup: u8) -> u8 {
    let minus_one = current_cup.checked_sub(1);
    match minus_one {
        None => {
            *(cups
                .highest_value()
                .expect("Don't call this on an empty circle!"))
        }
        Some(n) => {
            if cups.contains(&n) {
                n
            } else {
                find_destination_cup(cups, n)
            }
        }
    }
}

fn make_one_hundred_moves(cups: &mut Ring<u8>, current_cup: u8, visualise: bool) {
    let mut current_cup = current_cup;
    for m in 1..=100 {
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
    make_one_hundred_moves(&mut cups, current_cup, visualise);
    Ok(make_answer(&gather_order(&cups)))
}

#[test]
fn test_move() {
    let mut cups = Ring::new(vec![3, 8, 9, 1, 2, 5, 4, 6, 7]);
    let current_cup = make_move(&mut cups, 3, true);
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
