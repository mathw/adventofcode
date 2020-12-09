use crate::dayerror::DayError;
use itertools::Itertools;
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let input = parse_input(include_str!("input.txt"))?;

    let num = find_first_failing_num(&input, 25).ok_or_else(|| DayError::NoSolutionFoundError)?;

    Ok(format!("The first failing number is {}", num))
}

pub fn part2() -> Result<String, DayError> {
    let input = parse_input(include_str!("input.txt"))?;
    let num = find_first_failing_num(&input, 25).ok_or_else(|| DayError::NoSolutionFoundError)?;
    let (smallest, largest) = find_contiguous_range_adding_to(num, &input)
        .ok_or_else(|| DayError::NoSolutionFoundError)?;
    Ok(format!(
        "smallest {} largest {} encryption weakness {}",
        smallest,
        largest,
        smallest + largest
    ))
}

fn find_contiguous_range_adding_to(target: u64, nums: &Vec<u64>) -> Option<(u64, u64)> {
    'skip: for skip in 0..nums.len() {
        let mut sum = 0;
        let mut smallest = u64::MAX;
        let mut largest = 0;
        for n in nums.iter().skip(skip) {
            if smallest > *n {
                smallest = *n
            }
            if largest < *n {
                largest = *n
            }
            sum += n;
            if sum == target {
                return Some((smallest, largest));
            }
            if sum > target {
                continue 'skip;
            }
        }
    }
    None
}

fn parse_input(s: &str) -> Result<Vec<u64>, DayError> {
    s.lines()
        .map(|l| u64::from_str(l).map_err(|e| DayError::ParseIntError(e)))
        .collect()
}

fn check_num(nums: impl Iterator<Item = u64> + Clone, preamble_size: usize) -> bool {
    if let Some(num) = nums.clone().skip(preamble_size).next() {
        nums.take(preamble_size)
            .combinations(2)
            .any(|v| num == v[0] + v[1])
    } else {
        false
    }
}

fn find_first_failing_num(nums: &Vec<u64>, preamble_size: usize) -> Option<u64> {
    for skip in 0..(nums.len() - (preamble_size + 1)) {
        let chunk = nums
            .iter()
            .cloned()
            .skip(skip)
            .take(preamble_size + 1)
            .collect::<Vec<_>>();
        let the_num = chunk[preamble_size];
        if !check_num(chunk.into_iter(), preamble_size) {
            return Some(the_num);
        }
    }
    return None;
}

#[test]
fn test_check_num_sample_1() {
    let input = "35
20
15
25
47
40";
    let nums = parse_input(input).unwrap();
    assert!(check_num(nums.into_iter(), 5));
}

#[test]
fn test_check_num_sample_2() {
    let input = "20
15
25
47
40
62";
    let nums = parse_input(input).unwrap();
    assert!(check_num(nums.into_iter(), 5));
}

#[test]
fn test_check_num_sample_3() {
    let input = "15
25
47
40
62
55";
    let nums = parse_input(input).unwrap();
    assert!(check_num(nums.into_iter(), 5));
}

#[test]
fn test_check_num_sample_4() {
    let input = "25
47
40
62
55
65";
    let nums = parse_input(input).unwrap();
    assert!(check_num(nums.into_iter(), 5));
}

#[test]
fn test_check_num_sample_11() {
    let input = "95
102
117
150
182
127";
    let nums = parse_input(input).unwrap();
    assert!(!check_num(nums.into_iter(), 5));
}

#[test]
fn test_part1_sample() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let nums = parse_input(input).unwrap();
    let result = find_first_failing_num(&nums, 5);
    assert_eq!(result, Some(127));
}

#[test]
fn test_part2_sample() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    let nums = parse_input(input).unwrap();
    let (smallest, largest) = find_contiguous_range_adding_to(127, &nums).unwrap();
    assert_eq!(smallest, 15);
    assert_eq!(largest, 47);
}
