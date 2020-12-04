use crate::dayerror::DayError;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let records = parse_input(input);
    let num_valid = records.iter().filter(|r| record_is_valid(r)).count();
    Ok(format!("{} records are valid", num_valid))
}

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let records = parse_input(input);
    let num_valid = records.iter().filter(|r| record_is_fully_valid(r)).count();
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

fn record_is_valid(record: &HashMap<&str, &str>) -> bool {
    lazy_static! {
        static ref REQUIRED_FIELDS: [&'static str; 7] =
            ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    }
    REQUIRED_FIELDS.iter().all(|f| record.contains_key(f))
}

fn record_is_fully_valid(record: &HashMap<&str, &str>) -> bool {
    record_is_valid(record)
        && byr_is_valid(record["byr"])
        && iyr_is_valid(record["iyr"])
        && eyr_is_valid(record["eyr"])
        && hgt_is_valid(record["hgt"])
        && hcl_is_valid(record["hcl"])
        && ecl_is_valid(record["ecl"])
        && pid_is_valid(record["pid"])
}

fn byr_is_valid(input: &str) -> bool {
    year_valid_helper(input, 1920, 2002)
}

fn iyr_is_valid(input: &str) -> bool {
    year_valid_helper(input, 2010, 2020)
}

fn eyr_is_valid(input: &str) -> bool {
    year_valid_helper(input, 2020, 2030)
}

fn hgt_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref HEIGHT_RE: Regex = Regex::new(r"(\d+)(cm|in)").expect("REgex should parse");
    }

    if let Some(c) = HEIGHT_RE.captures(input) {
        if let Ok(num) = u32::from_str(&c[1]) {
            if &c[2] == "cm" {
                num >= 150 && num <= 193
            } else {
                num >= 59 && num <= 76
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn hcl_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref HCL_RE: Regex =
            Regex::new(r"^#([a-z]|[0-9]){6}$").expect("Regex should compile");
    }

    HCL_RE.is_match(input)
}

fn ecl_is_valid(input: &str) -> bool {
    matches!(input, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
}

fn pid_is_valid(input: &str) -> bool {
    lazy_static! {
        static ref PID_RE: Regex = Regex::new(r"^\d\d\d\d\d\d\d\d\d$").expect("Regex should parse");
    }

    PID_RE.is_match(input)
}

fn year_valid_helper(input: &str, min: u32, max: u32) -> bool {
    if input.len() != 4 {
        return false;
    }
    match u32::from_str(input) {
        Ok(year) => year >= min && year <= max,
        _ => false,
    }
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

#[cfg(test)]
static TEST_INPUT_2: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

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

#[test]
fn test_part2_sample() {
    let records = parse_input(TEST_INPUT_2);
    let num_valid = records.iter().filter(|r| record_is_fully_valid(r)).count();

    assert_eq!(num_valid, 4);
}

#[test]
fn test_byr_validity() {
    for year in 1920..=2002 {
        assert!(byr_is_valid(&year.to_string()));
    }
    assert!(byr_is_valid("1920"));
    assert!(byr_is_valid("2002"));
    assert!(!byr_is_valid("200"));
    assert!(!byr_is_valid("2003"));
    assert!(!byr_is_valid("1919"));
    assert!(!byr_is_valid("Bob"));
}

#[test]
fn test_iyr_validity() {
    for year in 2010..=2020 {
        assert!(iyr_is_valid(&year.to_string()));
    }
    assert!(!iyr_is_valid("2009"));
    assert!(!iyr_is_valid("2021"));
    assert!(!iyr_is_valid("2"));
    assert!(!iyr_is_valid(""));
}

#[test]
fn test_eyr_validity() {
    for year in 2020..=2030 {
        assert!(eyr_is_valid(&year.to_string()));
    }
    assert!(!eyr_is_valid("2019"));
    assert!(!eyr_is_valid("2031"));
    assert!(!eyr_is_valid("2"));
    assert!(!eyr_is_valid(""));
}

#[test]
fn test_hgt_validity() {
    assert!(hgt_is_valid("150cm"));
    assert!(hgt_is_valid("193cm"));
    assert!(!hgt_is_valid("149cm"));
    assert!(!hgt_is_valid("194cm"));
    assert!(hgt_is_valid("59in"));
    assert!(hgt_is_valid("76in"));
    assert!(!hgt_is_valid("77in"));
    assert!(!hgt_is_valid("55in"));
    assert!(!hgt_is_valid("s"));
}
