use crate::day::{DayResult, PartResult};
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error + 'static>> {
    let (input, input_size) = parse_input(include_str!("inputs/day3.txt"))?;
    let part1 = part1(&input, input_size);
    let part2 = part2(&input, input_size)?;
    Ok(DayResult::new(
        PartResult::Success(format!("Power consumption is {}", part1)),
        PartResult::Success(format!("Life support rating is {}", part2)),
    ))
}

fn parse_input_line(l: &str) -> Result<Vec<bool>, String> {
    l.chars()
        .map(|c| match c {
            '0' => Ok(false),
            '1' => Ok(true),
            _ => Err(format!("{} is not 1 or 0", c)),
        })
        .collect::<Result<Vec<bool>, String>>()
}

fn parse_input(input: &str) -> Result<(Vec<Vec<bool>>, usize), String> {
    let parsed = input
        .lines()
        .map(|l| parse_input_line(l))
        .collect::<Result<Vec<_>, _>>()?;

    let first_size = parsed[0].len();
    for n in parsed.iter() {
        if first_size != n.len() {
            return Err(format!("Not all input numbers are the same length"));
        }
    }

    Ok((parsed, first_size))
}

fn part1(inputs: &Vec<Vec<bool>>, input_size: usize) -> u64 {
    let mut ones = (0..input_size).map(|_| 0).collect::<Vec<usize>>();
    let mut zeroes = (0..input_size).map(|_| 0).collect::<Vec<usize>>();

    for entry in inputs {
        for bit in 0..input_size {
            if entry[bit] {
                ones[bit] += 1;
            } else {
                zeroes[bit] += 1;
            }
        }
    }

    let gamma_bits = (0..input_size)
        .map(|bit| if ones[bit] > zeroes[bit] { true } else { false })
        .collect::<Vec<_>>();
    let epsilon_bits = gamma_bits.iter().map(|bit| !bit).collect::<Vec<_>>();

    let gamma = make_decimal_from_bit_vec(&gamma_bits);
    let epsilon = make_decimal_from_bit_vec(&epsilon_bits);

    gamma * epsilon
}

fn make_decimal_from_bit_vec(input: &Vec<bool>) -> u64 {
    let str = input
        .iter()
        .map(|bit| if *bit { '1' } else { '0' })
        .collect::<String>();
    u64::from_str_radix(&str, 2).expect("Expected a valid binary string")
}

fn most_common_bit_in_position(nums: &Vec<Vec<bool>>, pos: usize) -> Result<bool, String> {
    #[cfg(test)]
    {
        println!("Looking for most common bit in position {} in input", pos);
        print_input(nums);
    }

    let ones: usize = nums
        .iter()
        .map(|n| n.get(pos))
        .collect::<Option<Vec<&bool>>>()
        .ok_or_else(|| format!("One or more input values didn't have a position {}", pos))?
        .iter()
        .map(|b| if **b { 1 } else { 0 })
        .filter(|b| *b == 1)
        .sum();
    let zeroes = nums.len() - ones;
    #[cfg(test)]
    println!("{} zeroes, {} ones", zeroes, ones);
    if zeroes > ones {
        #[cfg(test)]
        println!("Most common is 0");
        Ok(false)
    } else {
        #[cfg(test)]
        println!("Most common is 1");
        Ok(true)
    }
}

#[cfg(test)]
fn print_input(input: &Vec<Vec<bool>>) {
    for line in input {
        println!("{:?}", line);
    }
}

fn keep_all_with_bit_in_position(
    nums: &Vec<Vec<bool>>,
    bit: bool,
    pos: usize,
) -> Result<Vec<Vec<bool>>, String> {
    let mut result = Vec::new();

    for n in nums.iter() {
        let found_bit = n
            .get(pos)
            .ok_or_else(|| format!("An input value didn't have a position {}", pos))?;
        if *found_bit == bit {
            result.push(n.clone());
        }
    }

    Ok(result)
}

fn filter_on_position(
    nums: &Vec<Vec<bool>>,
    pos: usize,
    want_most_common: bool,
) -> Result<Vec<Vec<bool>>, String> {
    let most_common = most_common_bit_in_position(nums, pos)?;
    keep_all_with_bit_in_position(
        nums,
        if want_most_common {
            most_common
        } else {
            !most_common
        },
        pos,
    )
}

fn find_rating(
    input: &Vec<Vec<bool>>,
    input_size: usize,
    want_most_common: bool,
) -> Result<u64, String> {
    let mut current_set = input.clone();
    for pos in 0..input_size {
        current_set = filter_on_position(&current_set, pos, want_most_common)?;
        if current_set.len() == 1 {
            return Ok(make_decimal_from_bit_vec(&current_set[0]));
        }
        if current_set.len() == 0 {
            return Err("Ended up with no numbers somehow".to_owned());
        }
    }
    return Err(format!(
        "Finished but had {} numbers left which must be wrong (looking for {})",
        current_set.len(),
        if want_most_common {
            "most common"
        } else {
            "least common"
        }
    ));
}

fn oxygen_generator_rating(input: &Vec<Vec<bool>>, input_size: usize) -> Result<u64, String> {
    find_rating(input, input_size, true)
}

fn co2_scrubber_rating(input: &Vec<Vec<bool>>, input_size: usize) -> Result<u64, String> {
    find_rating(input, input_size, false)
}

fn part2(input: &Vec<Vec<bool>>, input_size: usize) -> Result<u64, String> {
    let oxygen = oxygen_generator_rating(input, input_size)?;
    let co2 = co2_scrubber_rating(input, input_size)?;
    Ok(oxygen * co2)
}

#[test]
fn test_part1_example() {
    let (input, input_size) =
        parse_input(include_str!("inputs/day3-sample.txt")).expect("Input should parse");
    let part1 = part1(&input, input_size);
    assert_eq!(part1, 198);
}

#[test]
fn test_part2_example() {
    let (input, input_size) =
        parse_input(include_str!("inputs/day3-sample.txt")).expect("Input should parse");
    let o2 = oxygen_generator_rating(&input, input_size);
    let co2 = co2_scrubber_rating(&input, input_size);
    let part2 = part2(&input, input_size);
    assert_eq!(o2, Ok(23));
    assert_eq!(co2, Ok(10));
    assert_eq!(part2, Ok(230));
}

#[test]
fn test_part2_example_o2() {
    let (input, input_size) =
        parse_input(include_str!("inputs/day3-sample.txt")).expect("Input should parse");
    let o2 = oxygen_generator_rating(&input, input_size);
    assert_eq!(o2, Ok(23));
}

#[test]
fn test_part2_example_co2() {
    let (input, input_size) =
        parse_input(include_str!("inputs/day3-sample.txt")).expect("Input should parse");
    let co2 = co2_scrubber_rating(&input, input_size);
    assert_eq!(co2, Ok(10));
}
