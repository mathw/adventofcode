use crate::day::{DayResult, PartResult};
use itertools::Itertools;
use maplit::hashmap;
use rayon::prelude::*;
use regex::Regex;
use std::{collections::HashMap, error::Error};

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let sequence = parse_sequence(include_str!("inputs/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/day14/rules.txt"))?;

    let part1 = part1(&sequence, &rules);
    let part2 = part2(&sequence, &rules);

    Ok(DayResult::new(
        PartResult::Success(format!("The answer is {}", part1)),
        PartResult::Success(format!("The answer is {}", part2)),
    ))
}

type Rules = HashMap<(char, char), [char; 3]>;

fn apply_rules(sequence: &Vec<char>, rules: &Rules, iterations: usize) -> HashMap<char, u64> {
    let last_char = *sequence
        .iter()
        .skip(sequence.len() - 1)
        .next()
        .expect("Can't apply rules to an empty sequence");
    let pairs: Vec<(char, char)> = sequence.clone().into_iter().tuple_windows().collect();
    let expanded_frequencies: Vec<HashMap<char, u64>> = pairs
        .par_iter()
        .map(|pair| apply_rules_to_pair(*pair, rules, iterations, &mut HashMap::new()))
        .collect();
    let mut final_frequencies = HashMap::new();
    for fs in expanded_frequencies {
        final_frequencies = combine_frequencies(final_frequencies, fs);
    }
    *final_frequencies.entry(last_char).or_default() += 1;
    final_frequencies
}

fn apply_rules_to_pair(
    (a, b): (char, char),
    rules: &Rules,
    iterations: usize,
    cache: &mut HashMap<(char, char, usize), HashMap<char, u64>>,
) -> HashMap<char, u64> {
    if let Some(r) = cache.get(&(a, b, iterations)) {
        return r.clone();
    }
    if iterations == 0 {
        return hashmap! {a => 1};
    }
    if let Some(&[a, insert, b]) = rules.get(&(a, b)) {
        let first = apply_rules_to_pair((a, insert), rules, iterations - 1, cache);
        let second = apply_rules_to_pair((insert, b), rules, iterations - 1, cache);
        let result = combine_frequencies(first, second);
        cache.insert((a, b, iterations), result.clone());
        return result;
    } else {
        let result = hashmap! {a => 1};
        cache.insert((a, b, iterations), result.clone());
        result
    }
}

fn combine_frequencies(
    first: HashMap<char, u64>,
    second: HashMap<char, u64>,
) -> HashMap<char, u64> {
    let mut output = first;
    for (c, f) in second {
        *output.entry(c).or_default() += f;
    }
    output
}

fn parse_rule(line: &str) -> Result<((char, char), [char; 3]), String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.)(.) -> (.)").unwrap();
    }

    let caps = RE.captures(line).ok_or(format!("Bad input string"))?;
    let a = caps[1].chars().next().unwrap();
    let b = caps[2].chars().next().unwrap();
    let insert = caps[3].chars().next().unwrap();
    Ok(((a, b), [a, insert, b]))
}

fn parse_rules(rules: &str) -> Result<Rules, String> {
    rules.lines().map(|l| parse_rule(l.trim())).collect()
}

fn parse_sequence(sequence: &str) -> Vec<char> {
    sequence.trim().chars().collect()
}

fn find_answer_from_frequencies(frequencies: &HashMap<char, u64>) -> u64 {
    let mut max_frequency = 0;
    let mut min_frequency = u64::MAX;

    for (_, &f) in frequencies {
        if f < min_frequency {
            min_frequency = f;
        }
        if f > max_frequency {
            max_frequency = f;
        }
    }

    max_frequency - min_frequency
}

fn part1(sequence: &Vec<char>, rules: &Rules) -> u64 {
    part_x(sequence, rules, 10)
}

fn part2(sequence: &Vec<char>, rules: &Rules) -> u64 {
    part_x(sequence, rules, 40)
}

fn part_x(sequence: &Vec<char>, rules: &Rules, iterations: usize) -> u64 {
    let frequencies = apply_rules(sequence, rules, iterations);

    find_answer_from_frequencies(&frequencies)
}

#[test]
fn test_part1_step1() {
    let sequence = parse_sequence(include_str!("inputs/samples/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/samples/day14/rules.txt")).unwrap();

    let result = apply_rules(&sequence, &rules, 1);

    assert_eq!(result, hashmap! {'N' => 2, 'C' => 2, 'H' => 1, 'B' => 2});
}

#[test]
fn test_part1() {
    let sequence = parse_sequence(include_str!("inputs/samples/day14/sequence.txt"));
    let rules = parse_rules(include_str!("inputs/samples/day14/rules.txt")).unwrap();
    let result = part1(&sequence, &rules);
    assert_eq!(result, 1588);
}
