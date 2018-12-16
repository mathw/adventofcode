use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct Rule {
    pattern: Vec<bool>,
    plant: bool,
}

impl Rule {
    pub fn pattern(&self) -> Vec<bool> {
        self.pattern.clone()
    }

    pub fn plant(&self) -> bool {
        self.plant.clone()
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"([\.#])([\.#])([\.#])([\.#])([\.#]) => ([\.#])").unwrap();
        }

        fn to_bool(c: &str) -> Result<bool, String> {
            if c == "#" {
                Ok(true)
            } else if c == "." {
                Ok(false)
            } else {
                Err(format!("{} was neither # nor .", c))
            }
        }

        if let Some(cap) = RE.captures_iter(input).next() {
            let mut pattern = vec![];
            pattern.push(to_bool(&cap[1])?);
            pattern.push(to_bool(&cap[2])?);
            pattern.push(to_bool(&cap[3])?);
            pattern.push(to_bool(&cap[4])?);
            pattern.push(to_bool(&cap[5])?);

            let plant = to_bool(&cap[6])?;

            Ok(Rule { pattern, plant })
        } else {
            Err(format!("Unable to match input \"{}\"", input))
        }
    }
}

#[test]
fn rule_parses_1() {
    let input = "....# => .";
    let rule = Rule::from_str(input);

    assert_eq!(
        rule,
        Ok(Rule {
            pattern: vec![false, false, false, false, true],
            plant: false
        })
    );
}

#[test]
fn rule_parses_2() {
    let input = "##### => #";
    let rule = Rule::from_str(input);

    assert_eq!(
        rule,
        Ok(Rule {
            pattern: vec![true, true, true, true, true],
            plant: true
        })
    );
}

#[test]
fn rule_parses_3() {
    let input = "#.### => #";
    let rule = Rule::from_str(input);

    assert_eq!(
        rule,
        Ok(Rule {
            pattern: vec![true, false, true, true, true],
            plant: true
        })
    );
}

#[test]
fn rule_parses_4() {
    let input = "#.#.# => .";
    let rule = Rule::from_str(input);

    assert_eq!(
        rule,
        Ok(Rule {
            pattern: vec![true, false, true, false, true],
            plant: false
        })
    );
}
