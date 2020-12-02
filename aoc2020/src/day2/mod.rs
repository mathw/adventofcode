use crate::dayerror::DayError;
use regex::Regex;
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let parsed = parse_input(input)?;
    let num_valid = parsed
        .iter()
        .map(|(rule, password)| rule.matches(&password))
        .filter(|result| *result)
        .count();
    Ok(format!(
        "{} out of {} passwords are valid",
        num_valid,
        parsed.len()
    ))
}

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let parsed = parse_input(input)?;
    let num_valid = parsed
        .iter()
        .map(|(rule, password)| rule.matches_new_rules(&password))
        .filter(|result| *result)
        .count();
    Ok(format!(
        "{} out of {} passwords are valid",
        num_valid,
        parsed.len()
    ))
}

fn parse_input(input: &str) -> Result<Vec<(PasswordRule, String)>, DayError> {
    input.lines().map(|line| parse_line(line)).collect()
}

fn parse_line(s: &str) -> Result<(PasswordRule, String), DayError> {
    lazy_static! {
        static ref RE: Regex =
            Regex::new("(?P<rule>.*): (?P<password>[a-z]+)").expect("Static regex should parse");
    }
    let caps = RE
        .captures(s)
        .ok_or_else(|| DayError::InputParseError(s.to_owned()))?;
    let rule = PasswordRule::from_str(&caps["rule"])?;
    let password = &caps["password"];
    Ok((rule, password.to_owned()))
}

#[derive(Debug)]
struct PasswordRule {
    min: usize,
    max: usize,
    what: char,
}

impl PasswordRule {
    fn new(min: usize, max: usize, what: char) -> PasswordRule {
        PasswordRule { min, max, what }
    }

    fn matches(&self, target: &str) -> bool {
        let times = target.chars().filter(|&c| c == self.what).count();
        times >= self.min && times <= self.max
    }

    fn matches_new_rules(&self, target: &str) -> bool {
        let chars = target.chars().collect::<Vec<char>>();
        let first = chars
            .get(self.min - 1)
            .map(|&c| c == self.what)
            .unwrap_or(false);
        let second = chars
            .get(self.max - 1)
            .map(|&c| c == self.what)
            .unwrap_or(false);
        !(first && second) && (first || second)
    }
}

impl FromStr for PasswordRule {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new("(?P<min>\\d+)-(?P<max>\\d+) (?P<what>[a-z])")
                .expect("Static regex should parse");
        }

        let caps = RE
            .captures(s)
            .ok_or_else(|| DayError::InputParseError(s.to_owned()))?;
        let min = caps["min"]
            .parse()
            .map_err(|_| DayError::InputParseError(s.to_owned()))?;
        let max = caps["max"]
            .parse()
            .map_err(|_| DayError::InputParseError(s.to_owned()))?;
        let what = caps["what"]
            .chars()
            .next()
            .ok_or_else(|| DayError::InputParseError(s.to_owned()))?;

        Ok(PasswordRule::new(min, max, what))
    }
}

#[test]
fn test_matches() {
    let rule = PasswordRule::new(2, 4, 'a');
    let valid1 = "abaab";
    let valid2 = "baabaab";
    let valid3 = "bbaab";
    let invalid1 = "b";
    let invalid2 = "aaaaa";

    assert!(rule.matches(valid1), "Three as is valid");
    assert!(rule.matches(valid2), "Four as is valid");
    assert!(rule.matches(valid3), "Two as is valid");
    assert!(!rule.matches(invalid1), "Zero as is invalid");
    assert!(!rule.matches(invalid2), "Five as is invalid");
}

#[test]
fn test_parses() {
    let rule = "3-11 k";
    let parsed = PasswordRule::from_str(rule);

    assert!(parsed.is_ok());
    let parsed = parsed.unwrap();
    assert_eq!(parsed.min, 3);
    assert_eq!(parsed.max, 11);
    assert_eq!(parsed.what, 'k');
}

#[test]
fn test_parse_line() {
    let line = "1-3 n: jakkwoon";
    let result = parse_line(line);
    println!("{:?}", result);
    assert!(result.is_ok());
    let (rule, password) = result.unwrap();
    assert_eq!(password, "jakkwoon");
    assert_eq!(rule.min, 1);
    assert_eq!(rule.max, 3);
    assert_eq!(rule.what, 'n');
    assert!(rule.matches(&password));
}

#[test]
fn test_matches_new() {
    let rule = PasswordRule::new(2, 4, 'a');
    let valid1 = "abaab";
    let valid2 = "baabaab";
    let valid3 = "bbaab";
    let invalid1 = "b";
    let invalid2 = "aaaaa";

    assert!(rule.matches_new_rules(valid1));
    assert!(rule.matches_new_rules(valid2));
    assert!(rule.matches_new_rules(valid3));
    assert!(!rule.matches_new_rules(invalid1));
    assert!(!rule.matches_new_rules(invalid2));
}
