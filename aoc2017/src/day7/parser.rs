use super::fact::Fact;
use std::str::FromStr;
use regex::Regex;
use std::collections::HashSet;

pub fn parse(input: &str) -> Option<Fact> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<name>\w+) \((?P<weight>\d+)\)( -> (?P<supporting>.+))?").unwrap();
    }

    if let Some(captures) = RE.captures(input) {

        let name = &captures.name("name").unwrap().as_str().to_owned();
        let weight = u32::from_str(&captures.name("weight").unwrap().as_str()).unwrap();

        let supporting = match &captures.name("supporting") {
            &Some(s) => s.as_str().split(", ").map(|name| name.to_owned()).collect(),
            &None => HashSet::new(),
        };

        Some(Fact::new(name.to_owned(), weight, supporting))
    } else {
        None
    }
}


#[test]
fn test_parse_leaf() {
    let input = "pbga (66)";
    assert_eq!(parse(input),
               Some(Fact::new("pbga".to_owned(), 66, HashSet::new())));
}

#[test]
fn test_parse_branch() {
    let input = "pbga (66) -> huyk, wwie, isow";
    let mut set = HashSet::new();
    set.insert("huyk".to_owned());
    set.insert("wwie".to_owned());
    set.insert("isow".to_owned());

    assert_eq!(parse(input), Some(Fact::new("pbga".to_owned(), 66, set)));
}
