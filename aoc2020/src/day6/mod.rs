use crate::dayerror::DayError;
use std::collections::HashSet;

pub fn part1() -> Result<String, DayError> {
    let answer = do_part1(include_str!("input.txt"));

    Ok(format!("The answer is {}", answer))
}

pub fn part2() -> Result<String, DayError> {
    let answer = do_part2(include_str!("input.txt"));

    Ok(format!("The answer is {}", answer))
}

fn do_part1(input: &str) -> usize {
    all_sets(input).map(|s| s.len()).sum()
}

fn do_part2(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|g| group_to_person_sets(g))
        .map(|people| group_intersection(people).len())
        .sum()
}

fn group_to_sets(s: &str) -> HashSet<char> {
    s.chars().filter(|c| c.is_alphabetic()).collect()
}

fn all_sets<'a>(s: &'a str) -> impl Iterator<Item = HashSet<char>> + 'a {
    s.split("\n\n").map(|g| group_to_sets(g))
}

fn group_to_person_sets(s: &str) -> Vec<HashSet<char>> {
    s.lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_alphabetic())
                .collect::<HashSet<char>>()
        })
        .collect()
}

fn group_intersection(people: impl IntoIterator<Item = HashSet<char>>) -> HashSet<char> {
    let mut iter = people.into_iter();
    if let Some(first) = iter.next() {
        iter.fold(first, |state, item| {
            state
                .intersection(&item)
                .cloned()
                .collect::<HashSet<char>>()
        })
    } else {
        HashSet::new()
    }
}

#[test]
fn test_group_to_set() {
    let group = "a
    bc
    d
    d
    a";
    let set = group_to_sets(group);

    let expected_set: HashSet<char> = vec!['a', 'b', 'c', 'd'].into_iter().collect();

    assert_eq![set, expected_set];
}
