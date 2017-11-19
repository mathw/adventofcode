use std::cmp;
use std::collections::HashSet;
use regex::Regex;
use regex::Captures;
use std::str::FromStr;
use std::collections::HashMap;

pub fn run() {
    let input = include_str!("input.txt");

    let instructions = input.lines()
        .map(|l| Instruction::from_str(l).expect("All instructions should be valid"))
        .collect::<Vec<_>>();

    println!("{} instructions parsed", instructions.len());

    // let mut lights = HashSet::new();

    // for instruction in instructions.clone() {
    //     lights = instruction.apply_to(&lights);
    // }

    // println!("{} lights are lit", lights.len());

    let mut brightnesses = HashMap::new();

    for instruction in instructions {
        instruction.apply_brightess(&mut brightnesses);
    }

    let brightness: u32 = brightnesses.values().sum();

    println!("Total brightness {}", brightness);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos {
    x: u16,
    y: u16,
}

impl Pos {
    fn new(x: u16, y: u16) -> Pos {
        Pos { x: x, y: y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Toggle(Pos, Pos),
    On(Pos, Pos),
    Off(Pos, Pos),
}

impl Instruction {
    fn apply_to(&self, lights: &HashSet<Pos>) -> HashSet<Pos> {
        match *self {
            Instruction::Toggle(a, b) => toggle(lights, a, b),
            Instruction::On(a, b) => turn_on(lights, a, b),
            Instruction::Off(a, b) => turn_off(lights, a, b),
        }
    }

    fn apply_brightess(&self, brightnesses: &mut HashMap<Pos, u32>) {
        match *self {
            Instruction::Toggle(a, b) => {
                change_brightness(brightnesses, BrightnessOp::IncreaseTwo, a, b)
            }
            Instruction::On(a, b) => {
                change_brightness(brightnesses, BrightnessOp::IncreaseOne, a, b)
            }
            Instruction::Off(a, b) => {
                change_brightness(brightnesses, BrightnessOp::DecreaseOne, a, b)
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref TOGGLE: Regex = Regex::new(r"toggle (\d+),(\d+) through (\d+),(\d+)").unwrap();
            static ref ON: Regex = Regex::new(r"turn on (\d+),(\d+) through (\d+),(\d+)").unwrap();
            static ref OFF: Regex = Regex::new(r"turn off (\d+),(\d+) through (\d+),(\d+)").unwrap();
        }

        fn from_caps(caps: &Captures) -> (Pos, Pos) {
            let ax = u16::from_str(caps.get(1).unwrap().as_str()).unwrap();
            let ay = u16::from_str(caps.get(2).unwrap().as_str()).unwrap();
            let bx = u16::from_str(caps.get(3).unwrap().as_str()).unwrap();
            let by = u16::from_str(caps.get(4).unwrap().as_str()).unwrap();
            (Pos::new(ax, ay), Pos::new(bx, by))
        }

        match TOGGLE.captures(s) {
            Some(caps) => {
                let (a, b) = from_caps(&caps);
                Ok(Instruction::Toggle(a, b))
            }
            None => {
                match ON.captures(s) {
                    Some(caps) => {
                        let (a, b) = from_caps(&caps);
                        Ok(Instruction::On(a, b))
                    }
                    None => {
                        match OFF.captures(s) {
                            Some(caps) => {
                                let (a, b) = from_caps(&caps);
                                Ok(Instruction::Off(a, b))
                            }
                            None => Err(()),
                        }
                    }
                }
            }
        }
    }
}

fn expand_rectangle(Pos { x: ax, y: ay }: Pos, Pos { x: bx, y: by }: Pos) -> HashSet<Pos> {
    let start_x = cmp::min(ax, bx);
    let start_y = cmp::min(ay, by);
    let end_x = cmp::max(ax, bx) + 1;
    let end_y = cmp::max(ay, by) + 1;

    let mut result = HashSet::new();

    for x in start_x..end_x {
        for y in start_y..end_y {
            result.insert(Pos::new(x, y));
        }
    }

    result
}

fn toggle(lights: &HashSet<Pos>, a: Pos, b: Pos) -> HashSet<Pos> {
    let toggles = expand_rectangle(a, b);
    lights.symmetric_difference(&toggles).cloned().collect()
}

fn turn_on(lights: &HashSet<Pos>, a: Pos, b: Pos) -> HashSet<Pos> {
    lights.union(&expand_rectangle(a, b)).cloned().collect()
}

fn turn_off(lights: &HashSet<Pos>, a: Pos, b: Pos) -> HashSet<Pos> {
    lights.difference(&expand_rectangle(a, b)).cloned().collect()
}

#[derive(Copy, Clone, Debug)]
enum BrightnessOp {
    IncreaseOne,
    IncreaseTwo,
    DecreaseOne,
}

fn change_brightness(lights: &mut HashMap<Pos, u32>, op: BrightnessOp, a: Pos, b: Pos) {
    let affected_positions = expand_rectangle(a, b);
    for pos in affected_positions {
        let entry = lights.entry(pos).or_insert(0);
        match op {
            BrightnessOp::IncreaseOne => *entry += 1,
            BrightnessOp::IncreaseTwo => *entry += 2,
            BrightnessOp::DecreaseOne => {
                if *entry > 0 {
                    *entry -= 1;
                }
            }
        }
    }
}

#[test]
fn test_expand_rectangle() {
    let poses = expand_rectangle(Pos::new(0, 0), Pos::new(2, 2));
    assert_eq!(poses,
               [Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(0, 2),
                Pos::new(1, 0),
                Pos::new(1, 1),
                Pos::new(1, 2),
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(2, 2)]
                   .iter()
                   .cloned()
                   .collect());
}


#[test]
fn test_toggle() {
    let lights = HashSet::new();
    let new_lights = toggle(&lights, Pos::new(1, 1), Pos::new(1, 2));
    assert_eq!(new_lights.len(), 2, "Two lights should be lit");
    let new_lights = toggle(&new_lights, Pos::new(1, 1), Pos::new(2, 1));
    assert_eq!(new_lights.len(), 2, "Two lights should be lit");
}

#[test]
fn test_turn_on() {
    let lights = HashSet::new();
    let lights = turn_on(&lights, Pos::new(0, 0), Pos::new(0, 0));
    assert_eq!(lights.len(), 1, "One light should be lit");
    let lights = turn_on(&lights, Pos::new(0, 0), Pos::new(4, 1));
    assert_eq!(lights.len(), 10, "Ten lights should be lit");
}

#[test]
fn test_turn_off() {
    let lights = HashSet::new();
    let lights = turn_on(&lights, Pos::new(0, 0), Pos::new(0, 0));
    assert_eq!(lights.len(), 1, "One light should be lit");
    let lights = turn_off(&lights, Pos::new(0, 0), Pos::new(4, 1));
    assert_eq!(lights.len(), 0, "No lights should be lit");
}

#[test]
fn test_parse_toggle() {
    let input = "toggle 6,7 through 23,0";
    let result = Instruction::from_str(input);
    assert_eq!(result,
               Ok(Instruction::Toggle(Pos::new(6, 7), Pos::new(23, 0))));
}

#[test]
fn test_parse_on() {
    let input = "turn on 6,7 through 23,0";
    let result = Instruction::from_str(input);
    assert_eq!(result, Ok(Instruction::On(Pos::new(6, 7), Pos::new(23, 0))));
}

#[test]
fn test_parse_off() {
    let input = "turn off 6,7 through 23,0";
    let result = Instruction::from_str(input);
    assert_eq!(result,
               Ok(Instruction::Off(Pos::new(6, 7), Pos::new(23, 0))));
}
