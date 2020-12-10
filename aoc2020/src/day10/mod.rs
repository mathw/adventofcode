use crate::dayerror::DayError;
use std::{collections::HashMap, num::ParseIntError, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let input = parse_input(include_str!("input.txt"))?;
    let (ones, threes) = do_part1(&input)?;

    Ok(format!(
        "Ones: {} Threes: {} Answer: {}",
        ones,
        threes,
        ones * threes
    ))
}

pub fn part2() -> Result<String, DayError> {
    let input = parse_input(include_str!("input.txt"))?;
    let answer = do_part2(&input)?;
    Ok(format!("There are {} possible combinations", answer))
}

fn parse_input(s: &str) -> Result<Vec<u16>, ParseIntError> {
    let mut input = s
        .lines()
        .map(|l| u16::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;
    input.sort();
    Ok(input)
}

fn do_part1(adapters: &Vec<u16>) -> Result<(usize, usize), DayError> {
    let mut sorted = adapters.clone();
    sorted.sort();
    let mut one_skips = 0;
    let mut three_skips = 1; // device adapter always causes a three-skip
    let mut last_adapter = 0; // outlet is 0
    for adapter in sorted.iter() {
        match adapter - last_adapter {
            0 | 2 => {}
            1 => one_skips += 1,
            3 => three_skips += 1,
            _ => {
                return Err(DayError::NoSolutionFoundWithReasonError(format!(
                    "There was a skip more than three between {} and {}!",
                    last_adapter, adapter
                )))
            }
        }
        last_adapter = *adapter;
    }
    Ok((one_skips, three_skips))
}

#[test]
fn test_p1_s1() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";
    let input = parse_input(input).unwrap();
    let (ones, threes) = do_part1(&input).unwrap();
    assert_eq!(ones, 7);
    assert_eq!(threes, 5);
}

#[test]
fn test_p1_s2() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let input = parse_input(input).unwrap();
    let (ones, threes) = do_part1(&input).unwrap();
    assert_eq!(ones, 22);
    assert_eq!(threes, 10);
}

fn do_part2(adapters: &Vec<u16>) -> Result<usize, DayError> {
    let goal = adapters.iter().max().ok_or_else(|| {
        DayError::NoSolutionFoundWithReasonError(format!("No adapters were supplied!"))
    })? + 3;
    let mut cache = HashMap::new();
    Ok(count_paths_betweeen(0, goal, adapters, &mut cache))
}

fn count_paths_betweeen(
    start: u16,
    end: u16,
    adapters: &Vec<u16>,
    cache: &mut HashMap<(u16, u16), usize>,
) -> usize {
    if let Some(result) = cache.get(&(start, end)) {
        return *result;
    }
    if start + 3 == end {
        cache.insert((start, end), 1);
        return 1;
    }
    let candidates = find_next_adapters(adapters.iter().skip_while(|a| **a <= start), start);
    let result = candidates
        .iter()
        .map(|c| count_paths_betweeen(*c, end, adapters, cache))
        .sum();
    cache.insert((start, end), result);
    result
}

/// Take a sorted iterator of adapters and find all the candidates for the next adapter based on the current value.
/// Assumes that any adapters <= current value are not included in the iterator.
fn find_next_adapters<'a>(adapters: impl Iterator<Item = &'a u16>, current_value: u16) -> Vec<u16> {
    let mut result = Vec::new();
    for candidate in adapters.take_while(|a| *a - current_value <= 3) {
        result.push(*candidate)
    }
    result
}

#[test]
fn test_p2_s1() {
    let input = "16
10
15
5
1
11
7
19
6
12
4";
    let input = parse_input(input).unwrap();
    let answer = do_part2(&input).unwrap();
    assert_eq!(answer, 8);
}

#[test]
fn test_p2_s2() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    let input = parse_input(input).unwrap();
    let answer = do_part2(&input).unwrap();
    assert_eq!(answer, 19208);
}
