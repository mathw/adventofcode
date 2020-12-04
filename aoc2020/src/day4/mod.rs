use crate::dayerror::DayError;
use std::collections::HashMap;

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let records = parse_input(input);
    let num_valid = records.iter().filter(|r| record_is_valid(r)).count();
    Ok(format!("{} records are valid", num_valid))
}

fn split_records<'a>(input: &'a str) -> impl Iterator<Item = &'a str> {
    input
        .split("\n\n")
        .map(|l| l.trim())
        .filter(|l| l.len() != 0)
}

fn record_fields<'a>(record: &'a str) -> impl Iterator<Item = (&'a str, &'a str)> {
    record
        .split(|c| c == ' ' || c == '\n')
        .map(|r| r.split(':').collect::<Vec<_>>())
        .map(|r| (r[0], r[1]))
}

fn parse_record<'a>(record: &'a str) -> HashMap<&'a str, &'a str> {
    record_fields(record).collect()
}

fn parse_input<'a>(input: &'a str) -> Vec<HashMap<&'a str, &'a str>> {
    split_records(input).map(|r| parse_record(r)).collect()
}

static REQUIRED_FIELDS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
fn record_is_valid(record: &HashMap<&str, &str>) -> bool {
    REQUIRED_FIELDS.iter().all(|f| record.contains_key(f))
}

#[cfg(test)]
static TEST_INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

#[test]
fn test_parse_record() {
    let record = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm";

    let parsed = parse_record(record);

    assert!(parsed.contains_key("ecl"));
    assert!(parsed.contains_key("pid"));
    assert!(parsed.contains_key("eyr"));
    assert!(parsed.contains_key("hcl"));
    assert!(parsed.contains_key("byr"));
    assert!(parsed.contains_key("iyr"));
    assert!(parsed.contains_key("cid"));
    assert!(parsed.contains_key("hgt"));
}

#[test]
fn test_record_valid() {
    let record = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm other:notrelevant";

    let parsed = parse_record(record);

    assert!(record_is_valid(&parsed));
}

#[test]
fn test_record_invalid() {
    let record = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 hgt:183cm other:notrelevant";

    let parsed = parse_record(record);

    assert!(record_is_valid(&parsed));
}

#[test]
fn test_part1_sample() {
    let records = parse_input(TEST_INPUT);
    let num_valid = records.iter().filter(|r| record_is_valid(r)).count();

    assert_eq!(num_valid, 2);
}

#[test]
fn test_split_records() {
    let input = "split
overline

not split over line";

    let records = split_records(input).collect::<Vec<_>>();

    assert_eq!(records.len(), 2);
    assert_eq!(
        records[0],
        "split
overline"
    );
    assert_eq!(records[1], "not split over line");
}
