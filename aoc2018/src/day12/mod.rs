mod plants;
mod rule;

use self::plants::Plants;
use self::rule::Rule;
use crate::day::Day;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day12 {
    rules: HashMap<Vec<bool>, bool>,
    initial_state: Plants,
}

impl Default for Day12 {
    fn default() -> Self {
        let input = include_str!("rules.txt");
        let mut rules = HashMap::new();
        for rule in input.lines().map(Rule::from_str) {
            match rule {
                Ok(r) => {
                    rules.insert(r.pattern(), r.plant());
                }
                _ => {}
            };
        }

        let input = include_str!("initial_state.txt");
        let plants = Plants::from_str(input).expect("Unable to parse initial state");

        Day12 {
            rules,
            initial_state: plants,
        }
    }
}

impl Day for Day12 {
    fn part1(&mut self, sender: &Sender<String>) {
        let result = run_until_generation(&self.initial_state, &self.rules, 20, sender)
            .sum_all_pots_with_plant();

        sender
            .send(format!("Sum of all pots with plant {}", result))
            .unwrap();
    }

    fn part2(&mut self, sender: &Sender<String>) {
        let result = run_until_generation(&self.initial_state, &self.rules, 50_000_000_000, sender)
            .sum_all_pots_with_plant();

        sender
            .send(format!("Sum of all pots with plant {}", result))
            .unwrap();
    }
}

fn run_until_generation(
    plants: &Plants,
    rules: &HashMap<Vec<bool>, bool>,
    generation: usize,
    sender: &Sender<String>,
) -> Plants {
    let mut seen_at_generation: HashMap<String, (usize, Option<i64>)> = HashMap::new();
    let mut new_plants: Plants = plants.clone();
    let mut g: usize = 0;

    loop {
        new_plants = new_plants.apply_rules(rules);

        let new_plants_str = new_plants.to_string();
        let lowest_pot = new_plants.lowest_pot_with_plant();
        if seen_at_generation.contains_key(&new_plants_str) {
            let (last_seen, last_lowest_pot) = seen_at_generation[&new_plants_str];
            let distance = g - last_seen;
            let skip = usize::max(((generation - g) / distance) * distance, 1);
            let shift = lowest_pot.unwrap_or(0) - last_lowest_pot.unwrap_or(0);
            let total_shift = (shift * skip as i64) - 1;

            sender
                .send(format!(
                    "Hey! I've seen this (generation {}) before! It was back in generation {}
Then it started at {} but now it starts at {}
I speculate that there's a {}-generation cycle with a {}-pot shift
Skipping forward {} generations and shifting {}",
                    g,
                    last_seen,
                    last_lowest_pot.unwrap_or(0),
                    lowest_pot.unwrap_or(0),
                    distance,
                    shift,
                    skip,
                    total_shift
                ))
                .unwrap();

            g += skip;
            new_plants = new_plants.shift(total_shift);
        } else {
            seen_at_generation.insert(new_plants_str, (g, lowest_pot));
            g += 1;
        }

        if g % 100_000 == 0 {
            sender
                .send(format!(
                    "Generation {}/{}: {}%",
                    g,
                    generation,
                    (g * 100) / generation
                ))
                .unwrap();
        }

        if g >= generation {
            break;
        }
    }

    new_plants
}
