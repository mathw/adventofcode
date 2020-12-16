use crate::dayerror::DayError;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    str::FromStr,
};

pub fn part1() -> Result<String, DayError> {
    let rules = parse_rules(include_str!("rules.txt"))?;
    let tickets = parse_tickets(include_str!("nearby_tickets.txt"))?;

    let rate = calculate_scanning_error_rate(&rules, &tickets);

    Ok(format!("Scanning error rate is {}", rate))
}

pub fn part2() -> Result<String, DayError> {
    let rules = parse_rules(include_str!("rules.txt"))?;
    let tickets = parse_tickets(include_str!("nearby_tickets.txt"))?;

    let valid_tickets = tickets
        .iter()
        .filter(|t| is_ticket_valid(&rules, t))
        .cloned()
        .collect::<Vec<_>>();

    let field_order = determine_field_order(&rules, &valid_tickets)?;

    let interesting_fields = field_order
        .into_iter()
        .filter(|(_, r)| r.name.starts_with("departure"))
        .map(|(i, _)| i);

    let my_ticket = vec![
        179, 101, 223, 107, 127, 211, 191, 61, 199, 193, 181, 131, 89, 109, 197, 59, 227, 53, 103,
        97,
    ];

    let answer = interesting_fields.map(|i| my_ticket[i]).product::<u64>();
    Ok(format!("The answer is {}", answer))
}

fn determine_field_order(
    rules: &Vec<ValidityRule>,
    tickets: &Vec<Vec<u32>>,
) -> Result<HashMap<usize, ValidityRule>, DayError> {
    let fields_count = tickets.get(0).ok_or(DayError::NoSolutionFoundError)?.len();
    let mut rule_candidates = HashMap::new();

    for rule in rules {
        for field in 0..fields_count {
            let entry = rule_candidates.entry(field).or_insert(HashSet::new());
            (*entry).insert(rule);
        }
    }

    let mut determined_order = HashMap::new();

    while rule_candidates.values().any(|x| x.len() > 1) {
        for field in 0..fields_count {
            if determined_order.contains_key(&field) {
                continue;
            }

            // figure out which rules are valid for each ticket
            let mut rules_to_keep = HashSet::new();
            'rule: for rule in &rule_candidates[&field] {
                if determined_order.values().any(|r| r == *rule) {
                    continue;
                }

                for ticket in tickets {
                    let value = ticket[field];
                    if !rule.validate(value) {
                        // rule is not valid - skip the rest of the tests for this rule and discard it
                        continue 'rule;
                    }
                }
                rules_to_keep.insert(*rule);
            }
            // replace with new known-valid rules
            if rules_to_keep.len() == 0 {
                return Err(DayError::NoSolutionFoundWithReasonError(format!(
                    "field {} has no valid rules left",
                    field
                )));
            }
            if rules_to_keep.len() == 1 {
                let rule = rules_to_keep.iter().next().unwrap().clone();
                determined_order.insert(field, rule.clone());
            }
            rule_candidates.insert(field, rules_to_keep);
        }
    }

    Ok(determined_order)
}

fn calculate_scanning_error_rate(rules: &Vec<ValidityRule>, tickets: &Vec<Vec<u32>>) -> u32 {
    tickets
        .iter()
        .flat_map(|ticket| get_invalid_fields(ticket, rules))
        .sum()
}

fn get_invalid_fields(ticket: &Vec<u32>, rules: &Vec<ValidityRule>) -> Vec<u32> {
    if ticket.len() != rules.len() {
        panic!("Ticket not valid: field count does not match rule count");
    }

    let mut invalid_fields = vec![];

    for field in ticket {
        let is_fully_invalid = rules.iter().map(|r| r.validate(*field)).all(|x| x == false);
        if is_fully_invalid {
            invalid_fields.push(*field)
        }
    }

    invalid_fields
}

fn is_ticket_valid(rules: &Vec<ValidityRule>, ticket: &Vec<u32>) -> bool {
    get_invalid_fields(ticket, rules).len() == 0
}

fn parse_ticket(ticket: &str) -> Result<Vec<u32>, DayError> {
    let nums = ticket
        .split(',')
        .map(|n| u32::from_str(n))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(nums)
}

fn parse_tickets(tickets: &str) -> Result<Vec<Vec<u32>>, DayError> {
    tickets.lines().map(|l| parse_ticket(l)).collect()
}

fn parse_rules(rules: &str) -> Result<Vec<ValidityRule>, DayError> {
    rules
        .lines()
        .map(|l| ValidityRule::from_str(l))
        .collect::<Result<Vec<_>, _>>()
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct ValidityRule {
    range1: Range<u32>,
    range2: Range<u32>,
    name: String,
}

impl FromStr for ValidityRule {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(.*): (\d+)-(\d+) or (\d+)-(\d+)").unwrap();
        }
        if let Some(captures) = RE.captures(s) {
            Ok(ValidityRule {
                name: String::from(&captures[1]),
                range1: Range {
                    start: captures[2].parse()?,
                    end: captures[3].parse::<u32>()? + 1,
                },
                range2: Range {
                    start: captures[4].parse()?,
                    end: captures[5].parse::<u32>()? + 1,
                },
            })
        } else {
            Err(DayError::InputParseError(format!(
                "Unable to parse input line {}",
                s
            )))
        }
    }
}

impl ValidityRule {
    fn validate(&self, num: u32) -> bool {
        self.range1.contains(&num) || self.range2.contains(&num)
    }
}

#[test]
fn test_rule_parse() {
    let rule = ValidityRule::from_str("departure location: 25-863 or 882-957")
        .expect("This rule should parse");
    // remembering that Range is half-open but the input data is fully closed
    assert_eq!(
        rule,
        ValidityRule {
            range1: Range {
                start: 25,
                end: 864
            },
            range2: Range {
                start: 882,
                end: 958
            },
            name: String::from("departure location")
        }
    )
}

#[test]
fn test_rule_validate() {
    let rule = ValidityRule::from_str("departure location: 25-863 or 882-957")
        .expect("This rule should parse");
    assert!(rule.validate(25));
    assert!(rule.validate(863));
    assert!(rule.validate(882));
    assert!(rule.validate(957));
    assert!(!rule.validate(864));
}

#[test]
fn test_part1_sample() {
    let rules = parse_rules(
        "class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50",
    )
    .unwrap();
    let tickets = parse_tickets(
        "7,3,47
40,4,50
55,2,20
38,6,12",
    )
    .unwrap();
    let result = calculate_scanning_error_rate(&rules, &tickets);
    assert_eq!(result, 71);
}
