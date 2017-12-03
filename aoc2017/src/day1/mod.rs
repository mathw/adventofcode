use std::time::Instant;
use util::asmillis::AsMillis;
use util;

pub fn go(count: usize) {
    // parse
    let input = include_str!("input.txt");

    let digits = parse_digits(input.trim());
    if digits.len() == 0 {
        panic!("No input");
    }

    // part one
    let timer1 = Instant::now();
    let partone = util::repeatedly(count, || sum_as_u32(&items_matching_next(&digits)));

    println!("[{}ms] The sum of all matching digits is {}",
             timer1.elapsed().as_millis(),
             partone);

    // part two
    let timer2 = Instant::now();
    let parttwo = util::repeatedly(count, || sum_as_u32(&items_matching_halfway_round(&digits)));

    println!("[{}ms] The sum of all digits which match the digit halfway around the list is {}",
             timer2.elapsed().as_millis(),
             parttwo);
}

fn sum_as_u32(items: &[u8]) -> u32 {
    items.iter().map(|&x| x as u32).sum()
}

fn parse_digits(input: &str) -> Vec<u8> {
    input.chars()
        .map(util::char_to_digit)
        .map(|x| {
            x.expect(&format!("The input string contained something which was not a digit. It \
                               was {}",
                              input))
        })
        .collect::<Vec<_>>()
}

fn items_matching_next<T: Eq + Clone>(items: &[T]) -> Vec<T> {
    get_items_matching(|index| &items[(index + 1) % items.len()], items)
}

fn items_matching_halfway_round<T: Clone + Eq>(items: &[T]) -> Vec<T> {
    let half = items.len() / 2;
    get_items_matching(|index| &items[(index + half) % items.len()], items)
}

fn get_items_matching<'a, T: Clone + Eq + 'a, F>(get_other: F, items: &[T]) -> Vec<T>
    where F: Fn(usize) -> &'a T
{
    items.iter()
        .enumerate()
        .filter_map(|(index, item)| if get_other(index) == item {
            Some(item.clone())
        } else {
            None
        })
        .collect()
}

#[cfg(test)]
fn match_test_helper(digits: Vec<u8>, expected: Vec<u8>) {
    let matches = items_matching_next(&digits);

    assert_eq!(matches, expected);
}

#[cfg(test)]
fn match_test_helper_2(digits: Vec<u8>, expected: Vec<u8>) {
    let matches = items_matching_halfway_round(&digits);

    assert_eq!(matches, expected);
}

#[test]
fn test_digits_matching_next_none_match() {
    match_test_helper(vec![0, 1, 2, 3, 4], vec![]);
}

#[test]
fn test_digits_matching_next_all_match() {
    match_test_helper(vec![1, 1, 1, 1], vec![1, 1, 1, 1]);
}

#[test]
fn test_digits_matching_next_some_match() {
    match_test_helper(vec![2, 3, 4, 4, 2], vec![4, 2]);
}

#[test]
fn test_parse_digits() {
    assert_eq!(parse_digits(""), vec![]);
    assert_eq!(parse_digits("0"), vec![0]);
    assert_eq!(parse_digits("23288839"), vec![2, 3, 2, 8, 8, 8, 3, 9]);
}

#[test]
fn test_digits_matching_half_none_match() {
    match_test_helper_2(vec![0, 1, 2, 3, 4], vec![]);
}

#[test]
fn test_digits_matching_half_all_match() {
    match_test_helper_2(vec![1, 2, 1, 2], vec![1, 2, 1, 2]);
}

#[test]
fn test_digits_matching_half_some_match() {
    match_test_helper_2(vec![1, 2, 3, 4, 2, 5], vec![2, 2]);
}
