use crate::dayerror::DayError;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

//mod part2;

pub fn part1() -> Result<String, DayError> {
    let rules = include_str!("rules.txt");
    let messages = include_str!("messages.txt");

    let answer = run_part1(rules, messages)?;

    Ok(format!("There are {} matching messages", answer))
}

pub fn part2() -> Result<String, DayError> {
    let rules = include_str!("rules.txt");
    let messages = include_str!("messages.txt");

    let answer = run_part2(rules, messages)?;

    Ok(format!("There are {} matching messages", answer))
}

fn run_part2(rules: &str, messages: &str) -> Result<usize, DayError> {
    let mut rules = parse_rules(rules)?;

    rules.insert(8, Rule::OneOrMore(42));
    rules.insert(11, Rule::SameNumberOf(42, 31));

    let re = compile_rules(&rules, 0);
    Ok(messages.lines().filter(|l| re.is_match(l)).count())
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub(crate) enum Rule {
    Terminal(char),
    Sequence(Vec<usize>),
    Alternative(Vec<usize>, Vec<usize>),
    OneOrMore(usize),
    SameNumberOf(usize, usize),
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
        Rule::OneOrMore(i) => format!("({})+", rule_to_regex_text(&rules[i], rules)),
        Rule::SameNumberOf(first, second) => {
            // okay so we can't do this generically because it's not a regex
            // but maybe we can do up to a certain number
            let first = rule_to_regex_text(&rules[first], rules);
            let second = rule_to_regex_text(&rules[second], rules);
            format!(
                "(({f}{s})|({f}{{2}}{s}{{2}})|({f}{{3}}{s}{{3}})|({f}{{4}}{s}{{4}}))",
                f = first,
                s = second
            )
        }
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

#[test]
fn test_part2_sample() {
    let messages = "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    let answer = run_part2(part2_sample_rules(), messages);
    assert!(answer.is_ok());
    let answer = answer.unwrap();
    assert_eq!(answer, 12);
}

#[cfg(test)]
fn part2_sample_rules() -> &'static str {
    "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1"
}

#[test]
fn test_part2_first_sample() {
    let messages = "bbabbbbaabaabba";
    let answer = run_part2(part2_sample_rules(), messages);
    assert!(answer.is_ok());
    let answer = answer.unwrap();
    assert_eq!(answer, 1);
}

#[test]
fn test_part2_second_sample() {
    let messages = "babbbbaabbbbbabbbbbbaabaaabaaa";
    let answer = run_part2(part2_sample_rules(), messages);
    assert!(answer.is_ok());
    let answer = answer.unwrap();
    assert_eq!(answer, 1);
}
