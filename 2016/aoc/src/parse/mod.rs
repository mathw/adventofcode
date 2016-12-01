use regex::Regex;
use instructions::{Step, Turn};

pub fn parse(input: &str) -> Vec<Step> {
    input.split(|c| c == ',' || c == ' ')
        .filter(|bit| bit.len() > 0)
        .map(|bit| bit_to_step(bit))
        .filter(|maybe_step| maybe_step.is_some())
        .map(|some_step| some_step.unwrap())
        .collect()
}

// turn a bit into a Step
fn bit_to_step(bit: &str) -> Option<Step> {
    lazy_static! {
        static ref BIT_REGEX: Regex = Regex::new(r"^(L|R)(\d+)").unwrap();
    }

    let caps = BIT_REGEX.captures_iter(bit).collect::<Vec<_>>();

    if caps.len() != 1 {
        return None;
    }

    let turn = match caps[0].at(1) {
        Some("L") => Some(Turn::Left),
        Some("R") => Some(Turn::Right),
        _ => None,
    };

    let blocks = caps[0].at(2).map(|b| b.parse::<u32>());

    match (turn, blocks) {
        (Some(t), Some(Ok(s))) => Some(Step::new(t, s)),
        _ => None,
    }
}

#[test]
fn test_right_bit() {
    let bit = "R8";
    let step = bit_to_step(bit);
    assert!(step.is_some());
    let s = step.unwrap();
    assert_eq!(s.turn, Turn::Right);
    assert_eq!(s.blocks, 8);
}

#[test]
fn test_left_bit() {
    let bit = "L788";
    let step = bit_to_step(bit);
    assert!(step.is_some());
    let s = step.unwrap();
    assert_eq!(s.turn, Turn::Left);
    assert_eq!(s.blocks, 788);
}

#[test]
fn test_bad_bit() {
    let bit = "p9eqoihfq";
    let step = bit_to_step(bit);
    assert!(step.is_none());
}

#[test]
fn test_parse_one() {
    let input = "L67";

    let result = parse(input);

    assert_eq!(result.len(), 1);
    assert_eq!(result[0].turn, Turn::Left);
    assert_eq!(result[0].blocks, 67);
}

#[test]
fn test_parse_two() {
    let input = "R78, L89";
    let result = parse(input);

    assert_eq!(result.len(), 2);
    assert_eq!(result[0].turn, Turn::Right);
    assert_eq!(result[0].blocks, 78);

    assert_eq!(result[1].turn, Turn::Left);
    assert_eq!(result[1].blocks, 89);
}
