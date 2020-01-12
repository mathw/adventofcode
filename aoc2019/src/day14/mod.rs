use crate::day::Day;
use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    str::FromStr,
};

pub struct Day14 {
    refinery: Refinery<'static>,
}

impl Day14 {
    pub fn new() -> Result<Day14, String> {
        Ok(Day14 {
            refinery: parse_refinery(include_str!("input.txt"))?,
        })
    }
}

impl Day for Day14 {
    fn part1(&mut self) -> Result<String, String> {
        let ore = self.refinery.run()?;
        Ok(format!("I need {} ore", ore))
    }
    fn part2(&mut self) -> Result<String, String> {
        let fuel = self.refinery.make_fuel_from_ore(1_000_000_000_000)?;
        Ok(format!("I can make {} fuel", fuel))
    }
}

type Reagent<'a> = (&'a str, u64);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Reaction<'a> {
    inputs: HashSet<Reagent<'a>>,
    output: Reagent<'a>,
}

impl<'a> Display for Reaction<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            fmt,
            "{} => {} {}",
            self.inputs
                .iter()
                .map(|r| format!("{} {}", r.1, r.0))
                .intersperse(", ".to_owned())
                .collect::<String>(),
            self.output.1,
            self.output.0
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Refinery<'a> {
    reactions: HashMap<&'a str, (u64, HashMap<&'a str, u64>)>,
}

impl<'a> Refinery<'a> {
    fn run(&self) -> Result<u64, String> {
        let mut leftovers = HashMap::new();
        let (_, ore) = self.produce_ingredient("FUEL", 1, &mut leftovers)?;
        Ok(ore)
    }

    fn make_fuel_from_ore(&self, available_ore: u64) -> Result<u64, String> {
        let (mut low, mut high) = self.find_range(available_ore)?;

        loop {
            if high - low < 2 {
                return Ok(low);
            }

            let mid = (high - low) / 2 + low;
            let (_, mid_ore) = self.produce_ingredient("FUEL", mid, &mut HashMap::new())?;
            match mid_ore.cmp(&available_ore) {
                Ordering::Less => {
                    low = mid;
                }
                Ordering::Equal => return Ok(mid),
                Ordering::Greater => high = mid,
            }
        }
    }

    fn find_range(&self, ore_target: u64) -> Result<(u64, u64), String> {
        let mut low = 0;
        let mut high = 1;

        loop {
            let (_, ore) = self.produce_ingredient("FUEL", high, &mut HashMap::new())?;
            match ore.cmp(&ore_target) {
                Ordering::Less => {
                    low = high;
                    high *= 4;
                }
                Ordering::Equal => return Ok((high, high + 1)),
                Ordering::Greater => return Ok((low, high)),
            }
        }
    }

    fn produce_ingredient(
        &self,
        what: &'a str,
        quantity: u64,
        leftovers: &mut HashMap<&'a str, u64>,
    ) -> Result<(u64, u64), String> {
        if what == "ORE" {
            return Ok((quantity, quantity));
        }

        let recipe = self
            .reactions
            .get(what)
            .ok_or(format!("Unable to find recipe for {}", what))?;

        let multiplier = if quantity <= recipe.0 as u64 {
            1
        } else {
            let mut m = quantity / recipe.0;
            while m * (recipe.0 as u64) < quantity {
                m += 1;
            }
            m
        };

        let multiplied_quantity = recipe.0 * multiplier;
        let multiplied_ingredients = recipe.1.iter().map(|(w, q)| (*w, q * multiplier));

        let mut ore_used = 0;

        for ingredient in multiplied_ingredients {
            if ingredient.0 == "ORE" {
                ore_used += ingredient.1;
                continue;
            }

            let leftover_option = leftovers.get(ingredient.0).map(|q| *q);
            if let Some(leftover_quantity) = leftover_option {
                let required_from_leftovers = u64::min(leftover_quantity, ingredient.1);
                let required_to_make = ingredient.1 - required_from_leftovers;
                *(leftovers.entry(ingredient.0).or_default()) -= required_from_leftovers;
                if required_to_make > 0 {
                    let (made, i_ore_used) =
                        self.produce_ingredient(ingredient.0, required_to_make, leftovers)?;
                    ore_used += i_ore_used;
                    let leftover = made - required_to_make;
                    if leftover > 0 {
                        *(leftovers.entry(ingredient.0).or_default()) += leftover;
                    }
                }
            } else {
                let (made, i_ore_used) =
                    self.produce_ingredient(ingredient.0, ingredient.1, leftovers)?;
                ore_used += i_ore_used;
                let leftover = made - ingredient.1;
                if leftover > 0 {
                    *(leftovers.entry(ingredient.0).or_default()) += leftover;
                }
            }
        }

        Ok((multiplied_quantity, ore_used))
    }
}

fn parse_refinery<'a>(input: &'a str) -> Result<Refinery<'a>, String> {
    let reactions = input
        .lines()
        .map(parse_reaction)
        .collect::<Result<HashMap<_, _>, String>>()?;
    Ok(Refinery { reactions })
}

fn parse_reaction<'a>(line: &'a str) -> Result<(&'a str, (u64, HashMap<&'a str, u64>)), String> {
    let mut sides = line.split("=>").map(|x| x.trim());
    let input_side = sides
        .next()
        .ok_or(format!("No input side of reaction \"{}\"", line))?;
    let output_side = sides
        .next()
        .ok_or(format!("No output side of reaction \"{}\"", line))?;
    let inputs = input_side
        .split(",")
        .map(|x| x.trim())
        .map(parse_reagent)
        .collect::<Result<HashMap<&'a str, u64>, _>>()?;
    let output = parse_reagent(output_side)?;
    Ok((output.0, (output.1, inputs)))
}

fn parse_reagent<'a>(input: &'a str) -> Result<Reagent<'a>, String> {
    let mut parts = input.split(" ").map(|x| x.trim());
    let quantity_part = parts.next().ok_or(format!("no quantity in {}", input))?;
    let what = parts.next().ok_or(format!("no what part in {}", input))?;
    let quantity = u64::from_str(quantity_part).map_err(|e| e.to_string())?;
    Ok((what, quantity))
}

#[test]
fn test_parse_reaction() {
    let input = "7 A, 1 E => 1 FUEL
    ";
    let reaction = parse_reaction(input);
    assert!(reaction.is_ok());
    let reaction = reaction.unwrap();
    assert_eq!(reaction.0, "FUEL",);
    assert_eq!((reaction.1).0, 1);
    assert_eq!(
        (reaction.1).1,
        vec![("A", 7), ("E", 1)]
            .into_iter()
            .collect::<HashMap<_, _>>()
    )
}

#[test]
fn test_how_much_ore_simple_1() {
    let refinery = parse_refinery("1 ORE => 1 FUEL").unwrap();
    let ore = refinery.run().unwrap();
    assert_eq!(ore, 1);
}

#[test]
fn test_how_much_ore_for_example_1() {
    let refinery = parse_refinery(
        "10 ORE => 10 A
    1 ORE => 1 B
    7 A, 1 B => 1 C
    7 A, 1 C => 1 D
    7 A, 1 D => 1 E
    7 A, 1 E => 1 FUEL",
    )
    .unwrap();
    let ore = refinery.run().unwrap();
    assert_eq!(ore, 31);
}

#[test]
fn test_how_much_ore_for_example_2() {
    let refinery = parse_refinery(
        "9 ORE => 2 A
        8 ORE => 3 B
        7 ORE => 5 C
        3 A, 4 B => 1 AB
        5 B, 7 C => 1 BC
        4 C, 1 A => 1 CA
        2 AB, 3 BC, 4 CA => 1 FUEL",
    )
    .unwrap();
    let ore = refinery.run().unwrap();
    assert_eq!(ore, 165);
}

#[test]
fn test_part1_example1() {
    let refinery = parse_refinery(
        "157 ORE => 5 NZVS
    165 ORE => 6 DCFZ
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
    179 ORE => 7 PSHF
    177 ORE => 5 HKGWZ
    7 DCFZ, 7 PSHF => 2 XJWVT
    165 ORE => 2 GPVTF
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT",
    )
    .unwrap();
    let answer = refinery.make_fuel_from_ore(1_000_000_000_000).unwrap();
    assert_eq!(answer, 82892753);
}
