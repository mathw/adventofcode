use crate::dayerror::DayError;
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;
    let start = (0, 0, Facing::East);
    let end = apply_instructions(start.0, start.1, start.2, instructions);

    Ok(format!(
        "Final position {},{} facing {:?} distance from origin {}",
        end.0,
        end.1,
        end.2,
        manhattan_distance(0, 0, end.0, end.1)
    ))
}

fn manhattan_distance(start_x: i64, start_y: i64, end_x: i64, end_y: i64) -> i64 {
    (start_x - end_x).abs() + (start_y - end_y).abs()
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
    Forward(u16),
    Left(u16),
    Right(u16),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Facing {
    North,
    South,
    East,
    West,
}

impl Facing {
    fn left(&self, angle: u16) -> Facing {
        match angle / 90 {
            0 => self.clone(),
            1 => match self {
                Facing::North => Facing::West,
                Facing::West => Facing::South,
                Facing::South => Facing::East,
                Facing::East => Facing::North,
            },
            2 => self.invert(),
            3 => self.right(90),
            _ => panic!("Unexpected left turn amount {}", angle),
        }
    }

    fn right(&self, angle: u16) -> Facing {
        match angle / 90 {
            0 => self.clone(),
            1 => match self {
                Facing::North => Facing::East,
                Facing::East => Facing::South,
                Facing::South => Facing::West,
                Facing::West => Facing::North,
            },
            2 => self.invert(),
            3 => self.left(90),
            _ => panic!("Unexpected right turn amount {}", angle),
        }
    }

    fn invert(&self) -> Facing {
        match self {
            Facing::North => Facing::South,
            Facing::South => Facing::North,
            Facing::East => Facing::West,
            Facing::West => Facing::East,
        }
    }
}

impl FromStr for Instruction {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err(DayError::InputParseError(format!(
                "{} is not a valid input string: too short",
                s
            )));
        }

        let first = &s[0..1];
        let rest = &s[1..s.len()];

        let arg = u16::from_str(rest)?;

        match first {
            "N" => Ok(Instruction::North(arg)),
            "S" => Ok(Instruction::South(arg)),
            "E" => Ok(Instruction::East(arg)),
            "W" => Ok(Instruction::West(arg)),
            "F" => Ok(Instruction::Forward(arg)),
            "L" => Ok(Instruction::Left(arg)),
            "R" => Ok(Instruction::Right(arg)),
            _ => Err(DayError::InputParseError(format!(
                "{} is not a valid instruction character",
                first
            ))),
        }
    }
}

fn apply_instruction(
    x: i64,
    y: i64,
    facing: Facing,
    instruction: Instruction,
) -> (i64, i64, Facing) {
    //println!("{},{} facing {:?} -> {:?}", x, y, facing, instruction);
    match instruction {
        Instruction::Left(angle) => (x, y, facing.left(angle)),
        Instruction::Right(angle) => (x, y, facing.right(angle)),
        Instruction::North(distance) => (x, y - distance as i64, facing),
        Instruction::South(distance) => (x, y + distance as i64, facing),
        Instruction::East(distance) => (x + distance as i64, y, facing),
        Instruction::West(distance) => (x - distance as i64, y, facing),
        Instruction::Forward(distance) => match facing {
            Facing::East => apply_instruction(x, y, facing, Instruction::East(distance)),
            Facing::West => apply_instruction(x, y, facing, Instruction::West(distance)),
            Facing::South => apply_instruction(x, y, facing, Instruction::South(distance)),
            Facing::North => apply_instruction(x, y, facing, Instruction::North(distance)),
        },
    }
}

fn apply_instructions(
    x: i64,
    y: i64,
    facing: Facing,
    instructions: impl IntoIterator<Item = Instruction>,
) -> (i64, i64, Facing) {
    let mut current = (x, y, facing);
    for instruction in instructions.into_iter() {
        current = apply_instruction(current.0, current.1, current.2, instruction);
    }

    current
}

// part 2 below

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn north(self, distance: i64) -> Point {
        Point {
            x: self.x,
            y: self.y - distance,
        }
    }
    fn south(self, distance: i64) -> Point {
        self.north(distance * -1)
    }
    fn east(self, distance: i64) -> Point {
        Point {
            x: self.x + distance,
            y: self.y,
        }
    }
    fn west(self, distance: i64) -> Point {
        self.east(distance * -1)
    }
    fn rotate_left(self, angle: u16) -> Point {
        let steps = angle / 90;
        match steps {
            0 => self,
            1 => Point {
                x: self.y,
                y: self.x * -1,
            },
            2 => Point {
                x: self.x * -1,
                y: self.y * -1,
            },
            3 => self.rotate_right(90),
            _ => panic!("Unexpected left rotation angle {}", angle),
        }
    }
    fn rotate_right(self, angle: u16) -> Point {
        let steps = angle / 90;
        match steps {
            0 => self,
            1 => Point {
                x: self.y * -1,
                y: self.x,
            },
            2 => self.rotate_left(180),
            3 => self.rotate_left(90),
            _ => panic!("Unexpected right rotation angle{}", angle),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ship {
    location: Point,
    waypoint: Point, // relative to ship location
}

impl Ship {
    fn apply_instruction(&self, instruction: Instruction) -> Ship {
        match instruction {
            Instruction::North(distance) => self
                .clone()
                .with_waypoint(self.waypoint.north(distance as i64)),
            Instruction::South(distance) => self
                .clone()
                .with_waypoint(self.waypoint.south(distance as i64)),
            Instruction::East(distance) => self
                .clone()
                .with_waypoint(self.waypoint.east(distance as i64)),
            Instruction::West(distance) => self
                .clone()
                .with_waypoint(self.waypoint.west(distance as i64)),
            Instruction::Forward(multiples) => self.to_waypoint(multiples as i64),
            Instruction::Left(degrees) => self
                .clone()
                .with_waypoint(self.waypoint.rotate_left(degrees)),
            Instruction::Right(degrees) => self
                .clone()
                .with_waypoint(self.waypoint.rotate_right(degrees)),
        }
    }

    fn with_waypoint(self, waypoint: Point) -> Ship {
        Ship {
            location: self.location,
            waypoint,
        }
    }

    fn with_location(self, location: Point) -> Ship {
        Ship {
            location: location,
            waypoint: self.waypoint,
        }
    }

    fn to_waypoint(self, multiple: i64) -> Ship {
        let new_location = Point {
            x: self.location.x + (self.waypoint.x * multiple),
            y: self.location.y + (self.waypoint.y * multiple),
        };
        self.with_location(new_location)
    }
}

pub fn part2() -> Result<String, DayError> {
    let instructions = include_str!("input.txt")
        .lines()
        .map(|l| Instruction::from_str(l))
        .collect::<Result<Vec<_>, _>>()?;

    let start = Ship {
        location: Point { x: 0, y: 0 },
        waypoint: Point { x: 10, y: -1 },
    };
    let mut end = start.clone();
    for instruction in instructions {
        end = end.apply_instruction(instruction);
    }

    Ok(format!(
        "End: {:?} distance from origin {}",
        end,
        manhattan_distance(
            start.location.x,
            start.location.y,
            end.location.x,
            end.location.y
        )
    ))
}

#[test]
fn test_rotate() {
    let point = Point { x: 4, y: -10 };
    let rotated = point.rotate_right(90);
    assert_eq!(rotated, Point { x: 10, y: 4 }, "Right 90");

    let rotated = point.rotate_right(180);
    assert_eq!(rotated, Point { x: -4, y: 10 }, "Right 180");

    let rotated = point.rotate_right(270);
    assert_eq!(rotated, Point { x: -10, y: -4 }, "Right 270");

    assert_eq!(point.rotate_left(90), point.rotate_right(270), "Left 90");
    assert_eq!(point.rotate_left(180), point.rotate_right(180), "Left 180");
    assert_eq!(point.rotate_left(270), point.rotate_right(90), "Left 270");
}
