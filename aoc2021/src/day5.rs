use crate::day::{DayResult, PartResult};
use bresenham::Bresenham;
use std::collections::HashMap;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error + 'static>> {
    let lines = parse_input(include_str!("inputs/day5.txt"))?;

    Ok(DayResult::new(
        PartResult::Success(format!("{} points overlap at least 2", part1(&lines))),
        PartResult::Success(format!("{} points overlap at least 2", part2(&lines))),
    ))
}

fn part1(input: &Vec<Line>) -> usize {
    let filtered: Vec<Line> = input
        .iter()
        .filter(|l| l.is_horizontal() || l.is_vertical())
        .cloned()
        .collect();
    let layered = layer_lines(&filtered);
    count_points_at_least(&layered, 2)
}

fn part2(input: &Vec<Line>) -> usize {
    let layered = layer_lines(&input);
    count_points_at_least(&layered, 2)
}

fn parse_input(input: &str) -> Result<Vec<Line>, Box<dyn Error>> {
    input
        .lines()
        .filter(|l| l.trim().len() > 0)
        .map(|l| Line::from_str(l.trim()))
        .collect::<Result<_, _>>()
}

fn layer_lines(lines: &Vec<Line>) -> HashMap<Pos, usize> {
    let mut m = HashMap::new();

    for line in lines {
        for pos in line.all_points() {
            let entry = m.entry(pos).or_insert(0);
            *entry += 1;
        }
    }

    m
}

fn count_points_at_least(points: &HashMap<Pos, usize>, threshold: usize) -> usize {
    points.values().filter(|v| **v >= threshold).count()
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Pos {
    x: isize, //not the type I'd chose but the Bresenham crate only works on isize where it should be generic
    y: isize,
}

#[derive(Clone, Debug, Hash)]
struct Line {
    first: Pos,
    second: Pos,
}

impl Line {
    fn is_horizontal(&self) -> bool {
        self.first.y == self.second.y
    }

    fn is_vertical(&self) -> bool {
        self.first.x == self.second.x
    }

    fn all_points(&self) -> Vec<Pos> {
        // I remembered Bresenham's algorithm is a thing, take that intro to computer graphics!
        // but why write my own
        let mut points: Vec<Pos> =
            Bresenham::new((self.first.x, self.first.y), (self.second.x, self.second.y))
                .map(|(x, y)| Pos { x, y })
                .collect();
        // the crate doesn't include the end point, but we need that
        points.push(self.second);

        points
    }
}

impl FromStr for Pos {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x_str, y_str) = s.split_once(",").ok_or_else(|| {
            format!(
                "Input string \"{}\" did not contain the expected delimiter",
                s
            )
        })?;
        let x = isize::from_str(x_str)?;
        let y = isize::from_str(y_str)?;
        Ok(Pos { x, y })
    }
}

impl FromStr for Line {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (first_str, second_str) = s.split_once(" -> ").ok_or_else(|| {
            format!(
                "Input string \"{}\" did not contain the expected delimiter",
                s
            )
        })?;
        let first = Pos::from_str(first_str)?;
        let second = Pos::from_str(second_str)?;

        Ok(Line { first, second })
    }
}

#[test]
fn test_part1_sample() {
    let lines = parse_input(include_str!("inputs/samples/day5.txt")).unwrap();
    let answer = part1(&lines);
    assert_eq!(answer, 5);
}

#[test]
fn test_part2_sample() {
    let lines = parse_input(include_str!("inputs/samples/day5.txt")).unwrap();
    let answer = part2(&lines);
    assert_eq!(answer, 12);
}
