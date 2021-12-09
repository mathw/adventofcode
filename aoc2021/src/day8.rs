use crate::day::{DayResult, PartResult};
#[cfg(test)]
use maplit::hashmap;
use maplit::hashset;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input = include_str!("inputs/day8.txt");
    let part1 = part1(input)?;
    let part2 = part2(input)?;
    Ok(DayResult::new(
        PartResult::Success(format!("{} unique segment numbers", part1)),
        PartResult::Success(format!("{} is the total number of all the readouts", part2)),
    ))
}
fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let parsed = parse_input(input)?;
    Ok(parsed
        .into_iter()
        .flat_map(|readout| readout.displayed.into_iter().map(|s| s.is_unique_pattern()))
        .filter(|&x| x)
        .count())
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let parsed = parse_input(input)?;
    let mut accumulator = 0;

    for (index, readout) in parsed.into_iter().enumerate() {
        accumulator += process_part2_item(&readout).ok_or_else(|| {
            format!(
                "Unable to determine a number for an input value on line {}",
                index
            )
        })?;
    }

    Ok(accumulator)
}

fn process_part2_item(readout: &Readout) -> Option<u64> {
    let constraints = compute_constraints(&readout.inputted);
    let mappings = all_possible_mappings(&constraints);
    let digits = translate_input_with_mappings(&mappings, &readout.displayed)?;
    let as_number: u64 = digits
        .into_iter()
        .rev()
        .enumerate()
        .map(|(i, d)| d as u64 * 10u64.pow(i as u32))
        .sum();
    Some(as_number)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Segment {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl TryFrom<char> for Segment {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'a' => Ok(Segment::A),
            'b' => Ok(Segment::B),
            'c' => Ok(Segment::C),
            'd' => Ok(Segment::D),
            'e' => Ok(Segment::E),
            'f' => Ok(Segment::F),
            'g' => Ok(Segment::G),
            _ => Err(format!("Unknown segment {}", c)),
        }
    }
}

fn compute_constraints(segments: &Vec<Segments>) -> HashMap<Segment, Constraint> {
    let mut constraints: HashMap<Segment, Constraint> = HashMap::new();
    let segments: Vec<Segments> = segments
        .iter()
        .filter(|s| s.is_unique_pattern())
        .chain(segments.iter().filter(|s| !s.is_unique_pattern()))
        .cloned()
        .collect();

    for s in segments {
        let lit_segments: HashSet<_> = s.lit_segments().collect();
        let unlit_segments: HashSet<_> = s.unlit_segments().collect();

        let true_segments: HashSet<Segment> = match s.num_segments_lit() {
            2 => hashset![Segment::C, Segment::F],
            3 => hashset![Segment::A, Segment::C, Segment::F],
            4 => hashset![Segment::B, Segment::C, Segment::D, Segment::F],
            5 => hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ],
            6 => hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G,
            ]
            .into_iter()
            .collect(),
            7 => hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
            ],
            _ => panic!("Impossible number of segments lit"),
        };
        let not_segments: HashSet<Segment> = match s.num_segments_lit() {
            2 => hashset![Segment::A, Segment::B, Segment::D, Segment::E, Segment::G],
            3 => hashset![Segment::B, Segment::D, Segment::E, Segment::G],
            4 => hashset![Segment::A, Segment::E, Segment::G],
            _ => hashset![],
        };

        for ts in true_segments {
            if constraints.contains_key(&ts) {
                let new_constraint = if s.is_unique_pattern() {
                    constraints[&ts]
                        .with(&lit_segments)
                        .without(&unlit_segments)
                } else {
                    constraints[&ts].with(&lit_segments)
                };
                constraints.insert(ts, new_constraint);
            } else {
                constraints.insert(
                    ts,
                    if s.is_unique_pattern() {
                        Constraint::default()
                            .with(&lit_segments)
                            .without(&unlit_segments)
                    } else {
                        Constraint::default().with(&lit_segments)
                    },
                );
            }
        }

        for ns in not_segments {
            if constraints.contains_key(&ns) {
                let new_constraint = constraints[&ns].without(&lit_segments);
                constraints.insert(ns, new_constraint);
            } else {
                constraints.insert(ns, Constraint::default().without(&lit_segments));
            }
        }
    }

    constraints
}

fn all_possible_mappings(
    constraints: &HashMap<Segment, Constraint>,
) -> Vec<HashMap<Segment, Segment>> {
    let mut mappings: HashMap<Segment, HashSet<Segment>> = HashMap::new();

    for (source, constraint) in constraints {
        for possible_target in constraint.possibilities() {
            mappings.entry(*source).or_default().insert(possible_target);
        }
    }
    let mut result = Vec::new();

    for pas in mappings.get(&Segment::A).expect("Must have A mappings") {
        for pbs in mappings.get(&Segment::B).expect("Must have B mappings") {
            for pcs in mappings.get(&Segment::C).expect("Must have C mappings") {
                for pds in mappings.get(&Segment::D).expect("Must have D mappings") {
                    for pes in mappings.get(&Segment::E).expect("Must have E mappings") {
                        for pfs in mappings.get(&Segment::F).expect("Must have F mappings") {
                            for pgs in mappings.get(&Segment::G).expect("Must have G mappings") {
                                let mut this_map = HashMap::new();
                                this_map.insert(pgs.clone(), Segment::G);
                                this_map.insert(pfs.clone(), Segment::F);
                                this_map.insert(pes.clone(), Segment::E);
                                this_map.insert(pds.clone(), Segment::D);
                                this_map.insert(pcs.clone(), Segment::C);
                                this_map.insert(pbs.clone(), Segment::B);
                                this_map.insert(pas.clone(), Segment::A);
                                result.push(this_map);
                            }
                        }
                    }
                }
            }
        }
    }
    // drop anything without 7 keys, they were invalid combos
    result.into_iter().filter(|m| m.len() == 7).collect()
}

fn try_mapping_input(
    mapping: &HashMap<Segment, Segment>,
    input: &Vec<Segments>,
) -> Option<Vec<u8>> {
    input
        .iter()
        .map(|i| i.map_with(mapping).as_digit())
        .collect()
}

fn translate_input_with_mappings(
    mappings: &Vec<HashMap<Segment, Segment>>,
    input: &Vec<Segments>,
) -> Option<Vec<u8>> {
    #[cfg(test)]
    {
        println!("translate: I have {} mappings to try", mappings.len());
        for mapping in mappings.iter() {
            println!("{:?}", mapping);
        }
    }
    for mapping in mappings {
        #[cfg(test)]
        println!("Trying another mapping");
        if let Some(digits) = try_mapping_input(mapping, input) {
            return Some(digits);
        }
    }
    None
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Constraint {
    is_one_of: HashSet<Segment>,
    is_not_one_of: HashSet<Segment>,
}

impl Default for Constraint {
    fn default() -> Self {
        Constraint {
            is_one_of: HashSet::new(),
            is_not_one_of: HashSet::new(),
        }
    }
}

impl Constraint {
    fn combine_with(&self, other: &Constraint) -> Option<Constraint> {
        let new_is_one_of: HashSet<_> = self
            .is_one_of
            .intersection(&other.is_one_of)
            .cloned()
            .collect();

        let new_is_not_one_of: HashSet<_> = self
            .is_not_one_of
            .union(&other.is_not_one_of)
            .cloned()
            .collect();
        Some(Constraint {
            is_one_of: new_is_one_of,
            is_not_one_of: new_is_not_one_of,
        })
    }

    fn with(&self, potentials: &HashSet<Segment>) -> Constraint {
        let allowed_new: HashSet<_> = potentials
            .difference(&self.is_not_one_of)
            .cloned()
            .collect();
        Self {
            is_one_of: self.is_one_of.union(&allowed_new).cloned().collect(),
            is_not_one_of: self.is_not_one_of.clone(),
        }
    }

    fn without(&self, exclusions: &HashSet<Segment>) -> Constraint {
        let allowed_new: HashSet<_> = self.is_one_of.difference(&exclusions).cloned().collect();
        let disallowed_new: HashSet<_> = self.is_not_one_of.union(&exclusions).cloned().collect();
        Self {
            is_one_of: allowed_new,
            is_not_one_of: disallowed_new,
        }
    }

    fn could_be(&self, segment: Segment) -> bool {
        self.is_one_of.contains(&segment) && !self.is_not_one_of.contains(&segment)
    }

    fn definitely_is_not(&self, segment: Segment) -> bool {
        self.is_not_one_of.contains(&segment)
    }

    fn possibilities(&self) -> HashSet<Segment> {
        self.is_one_of
            .difference(&self.is_not_one_of)
            .cloned()
            .collect()
    }

    fn normalised(self) -> Self {
        let new_is_one_of = self
            .is_one_of
            .difference(&self.is_not_one_of)
            .cloned()
            .collect();
        Self {
            is_one_of: new_is_one_of,
            ..self
        }
    }
}

#[derive(Debug, Clone)]
struct Segments(HashSet<Segment>);

impl Segments {
    fn is_unique_pattern(&self) -> bool {
        match self.num_segments_lit() {
            2 | 3 | 4 | 7 => true,
            _ => false,
        }
    }

    fn num_segments_lit(&self) -> usize {
        self.0.len()
    }

    fn lit_segments<'a>(&'a self) -> impl Iterator<Item = Segment> + 'a {
        self.0.iter().cloned()
    }
    fn unlit_segments<'a>(&'a self) -> impl Iterator<Item = Segment> + 'a {
        [
            Segment::A,
            Segment::B,
            Segment::C,
            Segment::D,
            Segment::E,
            Segment::F,
            Segment::G,
        ]
        .into_iter()
        .filter(|s| !self.0.contains(s))
    }

    fn as_digit(&self) -> Option<u8> {
        if self.0
            == hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::E,
                Segment::F,
                Segment::G
            ]
        {
            return Some(0);
        }
        if self.0 == hashset![Segment::C, Segment::F] {
            return Some(1);
        }
        if self.0 == hashset![Segment::A, Segment::C, Segment::D, Segment::E, Segment::G] {
            return Some(2);
        }
        if self.0 == hashset![Segment::A, Segment::C, Segment::D, Segment::F, Segment::G] {
            return Some(3);
        }
        if self.0 == hashset![Segment::B, Segment::C, Segment::D, Segment::F] {
            return Some(4);
        }
        if self.0 == hashset![Segment::A, Segment::B, Segment::D, Segment::F, Segment::G] {
            return Some(5);
        }
        if self.0
            == hashset![
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G
            ]
        {
            return Some(6);
        }
        if self.0 == hashset![Segment::A, Segment::C, Segment::F] {
            return Some(7);
        }
        if self.0
            == hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G
            ]
        {
            return Some(8);
        }
        if self.0
            == hashset![
                Segment::A,
                Segment::B,
                Segment::C,
                Segment::D,
                Segment::F,
                Segment::G
            ]
        {
            return Some(9);
        }

        #[cfg(test)]
        println!("Segments::as_digit(): I am not a valid digit: {:?}", self.0);
        None
    }

    fn map_with(&self, mapping: &HashMap<Segment, Segment>) -> Self {
        Self(self.0.iter().map(|s| mapping[s]).collect())
    }
}

impl Default for Segments {
    fn default() -> Self {
        Segments(HashSet::new())
    }
}

impl FromStr for Segments {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Segments::default();
        for c in s.chars() {
            match c {
                'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' => {
                    this.0.insert(c.try_into()?);
                }
                _ => {
                    return Err(format!(
                        "Character '{}' is not recognised as a display segment",
                        c
                    ))
                }
            }
        }
        Ok(this)
    }
}

struct Readout {
    inputted: Vec<Segments>,
    displayed: Vec<Segments>,
}

fn parse_input_line(input: &str) -> Result<Readout, Box<dyn Error>> {
    let parts = input.split("|").collect::<Vec<_>>();
    if parts.len() != 2 {
        return Err(format!("No delimiter found").into());
    }
    Ok(Readout {
        displayed: parts[1]
            .split_whitespace()
            .map(|s| Segments::from_str(s))
            .collect::<Result<Vec<_>, _>>()?,
        inputted: parts[0]
            .split_whitespace()
            .map(|s| Segments::from_str(s))
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn parse_input(input: &str) -> Result<Vec<Readout>, Box<dyn Error>> {
    input.lines().map(|l| parse_input_line(l)).collect()
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/samples/day8.txt");
    let result = part1(input).unwrap();
    assert_eq!(result, 26);
}

#[test]
fn test_part2_determination_1() {
    let input =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";
    let readout = parse_input_line(input).unwrap();
    let constraints = compute_constraints(&readout.inputted);
    let all = all_possible_mappings(&constraints);
    let digits = translate_input_with_mappings(&all, &readout.displayed);
    assert_eq!(digits, Some(vec![5, 3, 5, 3]));
}

#[test]
fn test_part2_digit_guessing() {
    let mapping1 = hashmap! {Segment::D => Segment::A,
    Segment::E => Segment::B, Segment::A => Segment::C, Segment::F => Segment::D, Segment::G => Segment::E, Segment::B => Segment::F, Segment::C => Segment::G};
    let mapping2 = hashmap! {Segment::D => Segment::B,
    Segment::E => Segment::A, Segment::A => Segment::C, Segment::F => Segment::D, Segment::G => Segment::E, Segment::B => Segment::F, Segment::C => Segment::G};
    let mappings = vec![mapping1, mapping2];
    let input = vec![
        Segments(hashset!(Segment::D, Segment::A, Segment::B)),
        Segments(hashset![
            Segment::C,
            Segment::D,
            Segment::F,
            Segment::B,
            Segment::E
        ]),
    ];
    let result = translate_input_with_mappings(&mappings, &input);
    assert_eq!(result, Some(vec![7, 5]));
}

#[test]
fn test_part2_digit_mapping() {
    let mapping = hashmap! {Segment::D => Segment::A,
    Segment::E => Segment::B, Segment::A => Segment::C, Segment::F => Segment::D, Segment::G => Segment::E, Segment::B => Segment::F, Segment::C => Segment::G};
    let input = Segments(hashset!(Segment::D, Segment::A, Segment::B));
    let digit = input.map_with(&mapping).as_digit();
    assert_eq!(digit, Some(7));
}

#[test]
fn test_compute_constraints_simplest() {
    let input = vec![Segments(hashset![Segment::A, Segment::B])];
    let constraints = compute_constraints(&input);
    assert_eq!(
        constraints.get(&Segment::C),
        Some(&Constraint {
            is_one_of: hashset![Segment::A, Segment::B],
            is_not_one_of: hashset![Segment::D, Segment::C, Segment::E, Segment::F, Segment::G]
        })
    );
    assert_eq!(
        constraints.get(&Segment::F),
        Some(&Constraint {
            is_one_of: hashset![Segment::A, Segment::B],
            is_not_one_of: hashset![Segment::D, Segment::C, Segment::E, Segment::F, Segment::G]
        })
    );
    assert_eq!(
        constraints.get(&Segment::A),
        Some(&Constraint {
            is_not_one_of: hashset![Segment::A, Segment::B],
            is_one_of: hashset![]
        })
    );
    assert_eq!(
        constraints.get(&Segment::B),
        Some(&Constraint {
            is_not_one_of: hashset![Segment::A, Segment::B],
            is_one_of: hashset![]
        })
    );
    assert_eq!(
        constraints.get(&Segment::D),
        Some(&Constraint {
            is_not_one_of: hashset![Segment::A, Segment::B],
            is_one_of: hashset![]
        })
    );
    assert_eq!(
        constraints.get(&Segment::E),
        Some(&Constraint {
            is_not_one_of: hashset![Segment::A, Segment::B],
            is_one_of: hashset![]
        })
    );
    assert_eq!(
        constraints.get(&Segment::G),
        Some(&Constraint {
            is_not_one_of: hashset![Segment::A, Segment::B],
            is_one_of: hashset![]
        })
    );
}

#[test]
fn test_compute_constraints_less_simple() {
    let input = vec![
        Segments(hashset![Segment::A, Segment::B]),
        Segments(hashset![Segment::A, Segment::B, Segment::C]),
    ];
    let constraints = compute_constraints(&input);
    assert_eq!(
        constraints.get(&Segment::A),
        Some(&Constraint {
            is_one_of: hashset![Segment::C],
            is_not_one_of: hashset![
                Segment::A,
                Segment::B,
                Segment::D,
                Segment::E,
                Segment::F,
                Segment::G
            ]
        })
    );
    assert_eq!(
        constraints.get(&Segment::C),
        Some(&Constraint {
            is_one_of: hashset![Segment::A, Segment::B],
            is_not_one_of: hashset![Segment::D, Segment::C, Segment::E, Segment::F, Segment::G]
        })
    );
    assert_eq!(
        constraints.get(&Segment::F),
        Some(&Constraint {
            is_one_of: hashset![Segment::A, Segment::B],
            is_not_one_of: hashset![Segment::D, Segment::C, Segment::E, Segment::F, Segment::G]
        })
    );
}

#[test]
fn test_failing_real_input_line() {
    let line =
        "gfadb fbagcd cagf agecdb adg fdbcg bdcfaeg bcgfde ga efbda | cbfdge dfcebga aedcgb dgbfa";
    let parsed = parse_input_line(line).unwrap();
    let answer = process_part2_item(&parsed);
    assert!(answer.is_some());
}
