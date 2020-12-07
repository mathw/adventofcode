use crate::dayerror::DayError;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn part1() -> Result<String, DayError> {
    let rules = include_str!("input.txt")
        .lines()
        .map(|l| ContainsRule::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;
    let tree = ContainsTree::from_rules("shiny gold", rules.iter());
    let colours = tree.unique_colours();
    Ok(format!(
        "Your shiny gold bag can be contained by {} other colours: {}",
        colours.len(),
        colours.iter().join(", ")
    ))
}

pub fn part2() -> Result<String, DayError> {
    let rules = include_str!("input.txt")
        .lines()
        .map(|l| ContainsRule::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;
    let result = num_bags_inside("shiny gold", &rules);
    Ok(format!(
        "There must be {} bags inside your shiny gold bag",
        result
    ))
}

struct ContainsRule<'a> {
    container: &'a str,
    contains: HashMap<&'a str, usize>,
}

impl<'a> ContainsRule<'a> {
    fn from_str(s: &'a str) -> Result<ContainsRule<'a>, DayError> {
        let bagindex = s
            .find("bags")
            .ok_or_else(|| DayError::InputParseError(s.to_string()))?;
        let (container, rest) = s.split_at(bagindex);
        let rest = rest.trim_start_matches("bags contain ");
        let container = container.trim_end();

        let mut contains = HashMap::new();
        if !rest.contains("no other bags") {
            let rest = rest.trim_end_matches(".");
            for c in rest.split(",").map(|s| s.trim()) {
                let space = c
                    .find(" ")
                    .ok_or_else(|| DayError::InputParseError(c.to_string()))?;
                let (quantitystr, descriptor) = c.split_at(space);
                let quantity = usize::from_str(quantitystr)?;
                let descriptor = descriptor
                    .trim_end_matches(" bags")
                    .trim_end_matches(" bag")
                    .trim_start();
                contains.insert(descriptor, quantity);
            }
        }

        Ok(ContainsRule {
            container,
            contains,
        })
    }
}

#[test]
fn test_parse_rule() {
    let rule = "clear purple bags contain 5 faded indigo bags, 1 muted purple bag.";
    let parsed = ContainsRule::from_str(rule).expect("this rule should parse");
    assert_eq!(parsed.container, "clear purple");
    assert!(parsed.contains.contains_key("faded indigo"));
    assert!(parsed.contains.contains_key("muted purple"));
    assert_eq!(parsed.contains["faded indigo"], 5);
    assert_eq!(parsed.contains["muted purple"], 1);
}

#[test]
fn test_parse_empty_rule() {
    let rule = "clear purple bags contain no other bags.";
    let parsed = ContainsRule::from_str(rule).expect("this rule should parse");
    assert_eq!(parsed.container, "clear purple");
    assert_eq!(parsed.contains.len(), 0);
}

struct ContainsTree {
    contained: String,
    containers: Box<Vec<ContainsTree>>,
}

impl ContainsTree {
    fn from_rules<'a>(
        target: &str,
        rules: impl Iterator<Item = &'a ContainsRule<'a>> + Clone,
    ) -> ContainsTree {
        let everything_containing_target = rules
            .clone()
            .filter(|r| r.contains.contains_key(target))
            .map(|r| {
                println!("found container {}", r.container);
                r.container
            })
            .map(|c| ContainsTree::from_rules(c, rules.clone()))
            .collect::<Vec<_>>();
        if everything_containing_target.len() == 0 {
            ContainsTree {
                contained: target.to_string(),
                containers: Box::new(vec![]),
            }
        } else {
            ContainsTree {
                contained: target.to_string(),
                containers: Box::new(everything_containing_target),
            }
        }
    }

    #[cfg(test)]
    fn render(&self) -> String {
        let mut output = String::new();
        for o in self
            .containers
            .iter()
            .map(|c| c.render())
            .flat_map(|s| s.lines().map(|l| format!("  {}\n", l)).collect::<Vec<_>>())
        {
            output += &o;
        }
        format!("{} ->\n{}", self.contained, output)
    }

    fn unique_colours(&self) -> HashSet<String> {
        let mut set = HashSet::new();
        for container in self.containers.iter() {
            set.insert(container.contained.clone());
        }
        for colour in self.containers.iter().flat_map(|c| c.unique_colours()) {
            set.insert(colour);
        }

        set
    }
}

#[test]
fn test_tree_from_rules() {
    let rules_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let rules = rules_str
        .lines()
        .map(|line| ContainsRule::from_str(line))
        .collect::<Result<Vec<_>, DayError>>()
        .expect("didn't expect a parse error");
    let tree = ContainsTree::from_rules("shiny gold", rules.iter());
    println!("{}", tree.render());
    let result = tree.unique_colours();
    assert_eq!(result.len(), 4);
}

fn num_bags_inside<'a>(target: &str, rules: &Vec<ContainsRule<'a>>) -> usize {
    let mut total = 0;
    for rule in rules.iter().filter(|r| r.container == target) {
        total += rule
            .contains
            .iter()
            .map(|c| c.1 + (num_bags_inside(c.0, rules) * c.1))
            .sum::<usize>();
    }
    total
}

#[test]
fn test_bags_inside() {
    let rules_str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    let rules = rules_str
        .lines()
        .map(|line| ContainsRule::from_str(line))
        .collect::<Result<Vec<_>, DayError>>()
        .expect("didn't expect a parse error");
    let result = num_bags_inside("shiny gold", &rules);
    assert_eq!(result, 32);
}

#[test]
fn test_bags_inside_2() {
    let rules_str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
    let rules = rules_str
        .lines()
        .map(|line| ContainsRule::from_str(line))
        .collect::<Result<Vec<_>, DayError>>()
        .expect("didn't expect a parse error");
    let result = num_bags_inside("shiny gold", &rules);
    assert_eq!(result, 126);
}
