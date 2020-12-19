use crate::dayerror::DayError;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let rules = include_str!("rules.txt");
    let messages = include_str!("messages.txt");

    let answer = run_part1(rules, messages)?;

    Ok(format!("There are {} matching messages", answer))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Rule {
    Terminal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<usize>, Vec<usize>),
}

fn parse_rule(s: &str) -> Result<(usize, Rule), DayError> {
    let mut parts = s.split(':');
    let index_part = parts.next().ok_or(DayError::InputParseError(format!(
        "Can't find an index part"
    )))?;
    let index = usize::from_str(index_part)?;
    let rest = parts
        .next()
        .ok_or(DayError::InputParseError(format!(
            "Can't find a definition part"
        )))?
        .trim();
    if rest.starts_with("\"") {
        if rest.ends_with("\"") {
            return Ok((
                index,
                Rule::Terminal(
                    rest.chars()
                        .nth(1)
                        .ok_or(DayError::InputParseError(format!(
                            "Couldn't find the middle of a terminal"
                        )))?,
                ),
            ));
        }
    }
    if rest.contains("|") {
        let mut alts = rest.split('|').map(|a| a.trim());
        let alt1 = alts
            .next()
            .ok_or(DayError::InputParseError(format!("")))?
            .split(" ")
            .map(|c| usize::from_str(c).unwrap())
            .collect::<Vec<_>>();
        let alt2 = alts
            .next()
            .ok_or(DayError::InputParseError(format!("")))?
            .split(" ")
            .map(|c| usize::from_str(c).unwrap())
            .collect::<Vec<_>>();
        return Ok((index, Rule::Alternative(alt1, alt2)));
    }
    let seq = rest
        .split(" ")
        .map(|c| usize::from_str(c).unwrap())
        .collect::<Vec<_>>();

    return Ok((index, Rule::Sequence(seq)));
}

fn parse_rules(input: &str) -> Result<HashMap<usize, Rule>, DayError> {
    input
        .lines()
        .map(|l| parse_rule(l))
        .collect::<Result<HashMap<_, _>, _>>()
}

fn compile_rules(rules: &HashMap<usize, Rule>, root_index: usize) -> Regex {
    let root_rule = &rules[&root_index];
    let source = format!("^{}$", rule_to_regex_text(root_rule, rules));
    Regex::new(&source).unwrap()
}

fn rule_to_regex_text(rule: &Rule, rules: &HashMap<usize, Rule>) -> String {
    match rule {
        Rule::Terminal(c) => format!("{}", c),
        Rule::Sequence(s) => s
            .iter()
            .map(|i| rule_to_regex_text(&rules[i], rules))
            .join(""),
        Rule::Alternative(l, r) => format!(
            "(({})|({}))",
            l.iter()
                .map(|i| rule_to_regex_text(&rules[i], rules))
                .join(""),
            r.iter()
                .map(|i| rule_to_regex_text(&rules[i], rules))
                .join(""),
        ),
    }
}

fn run_part1(rules: &str, messages: &str) -> Result<usize, DayError> {
    let rules = parse_rules(rules)?;
    let re = compile_rules(&rules, 0);
    Ok(messages.lines().filter(|l| re.is_match(l)).count())
}

#[test]
fn test_part1_sample() {
    let rules = parse_rules(
        "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\"",
    )
    .unwrap();
    let re = compile_rules(&rules, 0);
    assert!(re.is_match("ababbb"));
    assert!(re.is_match("abbbab"));
    assert!(!re.is_match("bababa"));
    assert!(!re.is_match("aaabbb"));
    assert!(!re.is_match("aaaabbb"));
}
