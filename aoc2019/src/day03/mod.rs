use crate::day::Day;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

pub struct Day3 {
    wires: Vec<Wire>,
}

impl Day3 {
    pub fn new() -> Day3 {
        Day3 {
            wires: parse_input(include_str!("input.txt")),
        }
    }
}

impl Day for Day3 {
    fn part1(&mut self) -> Result<String, String> {
        let crossings = wire_coords_cross(
            &wire_to_coords(&self.wires[0]),
            &wire_to_coords(&self.wires[1]),
        );
        let closest = closest_pos_to_origin(crossings);
        let closest_distance = closest
            .map(|p| p.distance_from_origin())
            .ok_or("No closest crossing found".to_owned())?;
        Ok(format!(
            "Closest crossing to origin is {} away",
            closest_distance
        ))
    }

    fn part2(&mut self) -> Result<String, String> {
        let crossings = combine_wires2(&self.wires[0], &self.wires[1]);
        let chosen = intersection_with_shortest_steps(&crossings)
            .map(|(_, d)| d)
            .ok_or("No crossings found".to_owned())?;
        Ok(format!("Shortest steps to a crossing is {}", chosen))
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Wire(Vec<Segment>);

#[derive(Debug, PartialEq, Eq)]
enum Segment {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize),
}

fn parse_input(input: &str) -> Vec<Wire> {
    input
        .lines()
        .map(|line| {
            let segments = line
                .split(',')
                .map(|s| {
                    (
                        s.chars().nth(0),
                        usize::from_str(&s[1..]).expect("Distance should parse to usize"),
                    )
                })
                .filter_map(|(direction, distance)| match direction {
                    Some('U') => Some(Segment::Up(distance)),
                    Some('D') => Some(Segment::Down(distance)),
                    Some('L') => Some(Segment::Left(distance)),
                    Some('R') => Some(Segment::Right(distance)),
                    _ => None,
                })
                .collect();
            Wire(segments)
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }

    fn to(&self, segment: &Segment) -> Vec<Pos> {
        match segment {
            Segment::Up(distance) => self.points_up(*distance),
            Segment::Down(distance) => self.points_down(*distance),
            Segment::Left(distance) => self.points_left(*distance),
            Segment::Right(distance) => self.points_right(*distance),
        }
    }
    fn points_left(&self, distance: usize) -> Vec<Pos> {
        (1..=distance)
            .map(|d| Pos::new(self.x - d as i32, self.y))
            .collect()
    }
    fn points_right(&self, distance: usize) -> Vec<Pos> {
        (1..=distance)
            .map(|d| Pos::new(self.x + d as i32, self.y))
            .collect()
    }
    fn points_up(&self, distance: usize) -> Vec<Pos> {
        (1..=distance)
            .map(|d| Pos::new(self.x, self.y + d as i32))
            .collect()
    }
    fn points_down(&self, distance: usize) -> Vec<Pos> {
        (1..=distance)
            .map(|d| Pos::new(self.x, self.y - d as i32))
            .collect()
    }

    fn distance_from_origin(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Stepped<T> {
    steps: usize,
    value: T,
}

fn wire_to_coords(wire: &Wire) -> HashSet<Pos> {
    let mut positions = HashSet::new();
    let mut current_position = Pos::new(0, 0);
    positions.insert(current_position);

    for segment in &wire.0 {
        for new in current_position.to(segment) {
            current_position = new;
            positions.insert(new);
        }
    }

    positions
}

fn wire_to_coords2(wire: &Wire) -> HashMap<Pos, usize> {
    let mut result = HashMap::new();

    let mut current_position = Pos::new(0, 0);
    let mut current_step = 0;
    result.insert(current_position, current_step);

    for segment in &wire.0 {
        for new in current_position.to(segment) {
            current_position = new;
            current_step += 1;
            result.insert(new, current_step);
        }
    }

    result
}

fn combine_wires2(wire1: &Wire, wire2: &Wire) -> HashMap<Pos, usize> {
    let mut result = HashMap::new();
    let wire1positions = wire_to_coords2(wire1);
    let wire2positions = wire_to_coords2(wire2);

    for w1p in wire1positions.keys() {
        if wire2positions.contains_key(w1p) {
            result.insert(*w1p, wire1positions[w1p] + wire2positions[w1p]);
        }
    }

    result
}

fn wire_coords_cross(first: &HashSet<Pos>, second: &HashSet<Pos>) -> HashSet<Pos> {
    first.intersection(second).cloned().collect()
}

fn closest_pos_to_origin<T>(poses: T) -> Option<Pos>
where
    T: IntoIterator<Item = Pos>,
{
    poses
        .into_iter()
        .sorted_by_key(|p| p.distance_from_origin())
        .filter(|p| p.distance_from_origin() > 0)
        .next()
}

fn intersection_with_shortest_steps(poses: &HashMap<Pos, usize>) -> Option<(Pos, usize)> {
    poses
        .iter()
        .sorted_by_key(|&(_, steps)| steps)
        .filter(|&(_, s)| *s > 0)
        .map(|(&a, &b)| (a, b))
        .next()
}

#[test]
fn test_parse() {
    let input = "R8,U5,L5,D3";
    let lines = parse_input(input);
    assert_eq!(
        vec![Wire(vec![
            Segment::Right(8),
            Segment::Up(5),
            Segment::Left(5),
            Segment::Down(3)
        ])],
        lines
    );
}

#[test]
fn test_cross() {
    let input = "R8,U5,L5,D3
U7,R6,D4,L4";
    let wires = parse_input(input);
    let crossings = wire_coords_cross(&wire_to_coords(&wires[0]), &wire_to_coords(&wires[1]));
    assert_eq!(
        crossings,
        vec![Pos::new(0, 0), Pos::new(3, 3), Pos::new(6, 5)]
            .into_iter()
            .collect::<HashSet<Pos>>()
    )
}

#[test]
fn test_closest_cross() {
    let input = "R8,U5,L5,D3
U7,R6,D4,L4";
    let wires = parse_input(input);
    let crossings = wire_coords_cross(&wire_to_coords(&wires[0]), &wire_to_coords(&wires[1]));
    let closest = closest_pos_to_origin(crossings);
    assert_eq!(closest, Some(Pos::new(3, 3)))
}

#[test]
fn test_part_2_sample_1() {
    let input = "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83";
    let wires = parse_input(input);
    let crossings = combine_wires2(&wires[0], &wires[1]);
    let chosen = intersection_with_shortest_steps(&crossings).map(|(_, d)| d);
    assert_eq!(chosen, Some(610));
}
