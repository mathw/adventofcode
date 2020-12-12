use crate::dayerror::DayError;
use std::{fmt, fmt::Display, str::FromStr};

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Seat {
    Full,
    Empty,
    Floor,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Seating {
    pub width: usize,
    pub height: usize,
    pub seats: Vec<Seat>,
}

impl Seating {
    pub fn new(width: usize, height: usize, seats: Vec<Seat>) -> Seating {
        Seating {
            width,
            height,
            seats,
        }
    }

    pub fn seat_at(&self, x: usize, y: usize) -> Option<Seat> {
        self.to_index(x, y).map(|i| self.seats[i])
    }

    pub fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        let index = self.to_index_unchecked(x, y);
        if index >= self.seats.len() {
            None
        } else {
            Some(index)
        }
    }

    pub fn to_index_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn count_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|s| **s == Seat::Full).count()
    }

    #[cfg(test)]
    pub fn count_empty_seats(&self) -> usize {
        self.seats.iter().filter(|s| **s == Seat::Empty).count()
    }
}

impl FromStr for Seating {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut seats = Vec::new();

        for line in s.lines().filter(|l| l.len() > 0) {
            width = line.len();
            height += 1;

            for c in line.chars() {
                match c {
                    'L' => seats.push(Seat::Empty),
                    '.' => seats.push(Seat::Floor),
                    '#' => seats.push(Seat::Full),
                    _ => {
                        return Err(DayError::InputParseError(format!(
                            "Unrecognised seat character '{}'",
                            c
                        )))
                    }
                };
            }
        }

        Ok(Seating::new(width, height, seats))
    }
}

impl Display for Seating {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.seat_at(x, y) {
                        Some(Seat::Full) => '#',
                        Some(Seat::Empty) => 'L',
                        Some(Seat::Floor) => '.',
                        None => '!',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_parse() {
    let result = Seating::from_str(
        "LL
..
L.
L#
",
    );
    let seating = result.expect("Should parse");
    assert_eq!(seating.width, 2);
    assert_eq!(seating.height, 4);
    assert_eq!(
        seating.seats,
        vec![
            Seat::Empty,
            Seat::Empty,
            Seat::Floor,
            Seat::Floor,
            Seat::Empty,
            Seat::Floor,
            Seat::Empty,
            Seat::Full
        ]
    );
}

#[test]
fn test_index() {
    let seating = Seating {
        height: 97,
        width: 99,
        seats: Vec::new(),
    };
    let index = seating.to_index_unchecked(0, 1);
    assert_eq!(index, 99);
}
