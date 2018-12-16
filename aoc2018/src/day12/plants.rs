use std::collections::{HashMap, HashSet};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Plants {
    plants: HashSet<i64>,
}

impl Plants {
    pub fn sum_all_pots_with_plant(&self) -> i64 {
        self.plants.iter().sum()
    }

    pub fn lowest_pot_with_plant(&self) -> Option<i64> {
        self.plants.iter().min().map(|x| *x)
    }

    pub fn apply_rules(&self, rules: &HashMap<Vec<bool>, bool>) -> Plants {
        let mut new_plants = HashSet::new();
        let lowest_pot = *self.plants.iter().min().unwrap_or(&0) - 2;
        let highest_pot = *self.plants.iter().max().unwrap_or(&0) + 2;

        for pot in lowest_pot..=highest_pot {
            let surround = self.get_pot_surround(pot);
            if rules.contains_key(&surround) && rules[&surround] {
                new_plants.insert(pot);
            }
        }

        Plants { plants: new_plants }
    }

    pub fn shift(&self, shift: i64) -> Plants {
        let mut new_plants = HashSet::new();

        for pot in self.plants.iter() {
            new_plants.insert(pot + shift);
        }

        Plants { plants: new_plants }
    }

    fn get_pot_surround(&self, pot: i64) -> Vec<bool> {
        let mut result = vec![];
        for pot_number in pot - 2..=pot + 2 {
            result.push(self.plants.contains(&pot_number));
        }

        result
    }
}

impl FromStr for Plants {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut plants = HashSet::new();

        for (pot, c) in input.chars().enumerate() {
            if c == '#' {
                plants.insert(pot as i64);
            }
        }

        Ok(Plants { plants })
    }
}

impl fmt::Display for Plants {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let lowest_pot = self.plants.iter().min().unwrap_or(&0);
        let highest_pot = self.plants.iter().max().unwrap_or(&0);

        for pot in *lowest_pot..=*highest_pot {
            write!(
                fmt,
                "{}",
                if self.plants.contains(&pot) { '#' } else { '.' }
            )?;
        }

        Ok(())
    }
}

#[test]
fn initial_state_parses() {
    let input = "#..#.#..##......###...###";
    let plants = Plants::from_str(input);

    assert_eq!(
        plants,
        Ok(Plants {
            plants: vec![0, 3, 5, 8, 9, 16, 17, 18, 22, 23, 24]
                .iter()
                .cloned()
                .collect(),
        })
    );
}

#[test]
fn sum_pots() {
    // this is the input from the part one example, minus pots -1 and -2 and -3
    let input = "...##....#####...#######....#.#..##.";
    let mut plants = Plants::from_str(input).expect("Should parse!");
    plants.plants.insert(-2); // set pot -2 to be true also because the parser can't

    assert_eq!(plants.sum_all_pots_with_plant(), 325);
}

#[test]
fn pot_surround_all_positive_inside() {
    let plants = Plants {
        plants: vec![0, 1, 3, 4].iter().cloned().collect(),
    };

    let surround = plants.get_pot_surround(2);
    assert_eq!(surround, vec![true, true, false, true, true]);

    let surround = plants.get_pot_surround(3);
    assert_eq!(surround, vec![true, false, true, true, false]);
}

#[test]
fn pot_surround_all_positive_past_end() {
    let plants = Plants {
        plants: vec![0, 1, 2].iter().cloned().collect(),
    };

    let surround = plants.get_pot_surround(2);
    assert_eq!(surround, vec![true, true, true, false, false]);
    let surround = plants.get_pot_surround(3);
    assert_eq!(surround, vec![true, true, false, false, false]);
}

#[test]
fn pot_surround_negative() {
    let plants = Plants {
        plants: vec![0, 1, 2].iter().cloned().collect(),
    };

    let surround = plants.get_pot_surround(-2);
    assert_eq!(surround, vec![false, false, false, false, true]);
    let surround = plants.get_pot_surround(-1);
    assert_eq!(surround, vec![false, false, false, true, true]);
}

#[test]
fn simple_rules_test_no_matches() {
    let mut rules = HashMap::new();
    rules.insert(vec![false, true, false, true, false], true);

    let plants = Plants {
        plants: vec![1].iter().cloned().collect(),
    };

    let new_plants = plants.apply_rules(&rules);

    assert_eq!(
        new_plants,
        Plants {
            plants: HashSet::new()
        }
    );
}

#[test]
fn simple_rules_test_one_rule_matches() {
    let mut rules = HashMap::new();
    rules.insert(vec![false, true, false, true, false], true);

    let plants = Plants {
        plants: vec![1, 3].iter().cloned().collect(),
    };

    let new_plants = plants.apply_rules(&rules);

    assert_eq!(
        new_plants,
        Plants {
            plants: vec![2].iter().cloned().collect()
        }
    );
}

#[test]
fn simple_rules_test_two_rules_match() {
    let mut rules = HashMap::new();
    rules.insert(vec![false, true, false, true, false], true);
    rules.insert(vec![false, true, false, false, false], true);

    let plants = Plants {
        plants: vec![1, 3].iter().cloned().collect(),
    };

    let new_plants = plants.apply_rules(&rules);

    assert_eq!(
        new_plants,
        Plants {
            plants: vec![2, 4].iter().cloned().collect()
        }
    );
}
