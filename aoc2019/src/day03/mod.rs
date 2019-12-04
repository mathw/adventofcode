use itertools::Itertools;
use std::collections::HashSet;
use std::str::FromStr;

const INPUT: &'static str = include_str!("input.txt");

pub fn run() -> Result<(), String> {
    let wires = parse_input(INPUT);
    let crossings = wire_coords_cross(&wire_to_coords(&wires[0]), &wire_to_coords(&wires[1]));
    let closest = closest_pos_to_origin(crossings);
    let closest_distance = closest.map(|p| p.distance_from_origin());
    println!(
        "Part 1: Closest crossing to origin is {:?} away",
        closest_distance
    );
    Ok(())
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
            Segment::Up(distance) => self.iterate_y(*distance as i32),
            Segment::Down(distance) => self.iterate_y(*distance as i32 * -1),
            Segment::Left(distance) => self.iterate_x(*distance as i32 * -1),
            Segment::Right(distance) => self.iterate_x(*distance as i32),
        }
    }
    fn iterate_x(&self, distance: i32) -> Vec<Pos> {
        let start = self.x;
        let end = self.x + distance;
        let result = if start < end {
            (start..=end)
        } else {
            (end..=start)
        }
        .map(|x| Pos { x: x, y: self.y });

        if end < start {
            result.rev().collect()
        } else {
            result.collect()
        }
    }
    fn iterate_y(&self, distance: i32) -> Vec<Pos> {
        let start = self.y;
        let end = self.y + distance;

        let result = if start < end {
            (start..=end)
        } else {
            (end..=start)
        }
        .map(|y| Pos { x: self.x, y: y });

        if end < start {
            result.rev().collect()
        } else {
            result.collect()
        }
    }

    fn distance_from_origin(&self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize
    }
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
