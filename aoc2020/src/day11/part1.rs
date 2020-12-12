#[cfg(test)]
use super::common::iterate;
use super::{
    common::iterate_until_stable,
    seating::{Seat, Seating},
    traits::*,
};
#[cfg(test)]
use itertools::Itertools;
#[cfg(test)]
use std::str::FromStr;
use std::{fmt, fmt::Display, time::Duration};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SeatingWithPart1Rules(pub Seating);

impl SolutionFinder for SeatingWithPart1Rules {
    fn find_solution(&self) -> (Self, usize) {
        iterate_until_stable(self, &mut |_, _| {}, None)
    }
    fn find_solution_visually<D>(&self, drawfunc: &mut D, delay: Option<Duration>) -> (Self, usize)
    where
        D: FnMut(&Self, usize) -> (),
    {
        iterate_until_stable(self, drawfunc, delay)
    }
}

impl SeatIterable for SeatingWithPart1Rules {
    fn iterate_seat(&self, x: usize, y: usize, index: usize) -> Seat {
        let seat = self.0.seats[index];
        match seat {
            Seat::Empty if self.count_adjacent_occupied(x, y) == 0 => Seat::Full,
            Seat::Full if self.count_adjacent_occupied(x, y) >= 4 => Seat::Empty,
            x => x,
        }
    }

    fn seating(&self) -> &Seating {
        &self.0
    }

    fn seating_mut(&mut self) -> &mut Seating {
        &mut self.0
    }
}

impl SeatingWithPart1Rules {
    pub fn count_adjacent_occupied(&self, x: usize, y: usize) -> usize {
        let xsub = x.checked_sub(1);
        let ysub = y.checked_sub(1);
        let xplus = if x + 1 < self.0.width {
            Some(x + 1)
        } else {
            None
        };
        let yplus = if y + 1 < self.0.height {
            Some(y + 1)
        } else {
            None
        };

        let locations = match (xsub, ysub, xplus, yplus) {
            (None, None, None, None) => vec![],
            (Some(xs), None, None, None) => vec![(xs, y)],
            (None, Some(ys), None, None) => vec![(x, ys)],
            (None, None, Some(xp), None) => vec![(xp, y)],
            (None, None, None, Some(yp)) => vec![(x, yp)],
            (Some(xs), Some(ys), None, None) => vec![(xs, ys), (xs, y), (x, ys)],
            (None, Some(ys), Some(xp), None) => vec![(x, ys), (xp, ys), (xp, y)],
            (None, None, Some(xp), Some(yp)) => vec![(xp, y), (xp, yp), (x, yp)],
            (None, Some(ys), None, Some(yp)) => vec![(x, ys), (x, yp)],
            (Some(xs), None, None, Some(yp)) => vec![(x, yp), (xs, yp), (xs, y)],
            (Some(xs), None, Some(xp), None) => vec![(xs, y), (xp, y)],
            (Some(xs), Some(ys), Some(xp), None) => {
                vec![(xp, y), (xp, ys), (xs, ys), (xs, y), (x, ys)]
            }
            (None, Some(ys), Some(xp), Some(yp)) => {
                vec![(x, ys), (xp, ys), (xp, y), (x, yp), (xp, yp)]
            }
            (Some(xs), None, Some(xp), Some(yp)) => {
                vec![(x, yp), (xs, yp), (xp, yp), (xs, y), (xp, y)]
            }
            (Some(xs), Some(ys), None, Some(yp)) => {
                vec![(xs, y), (xs, ys), (xs, yp), (x, ys), (x, yp)]
            }
            (Some(xs), Some(ys), Some(xp), Some(yp)) => vec![
                (xs, ys),
                (xs, y),
                (xs, yp),
                (x, ys),
                (x, yp),
                (xp, ys),
                (xp, y),
                (xp, yp),
            ],
        };

        locations
            .iter()
            .filter_map(|&(x, y)| self.0.seats.get(self.0.to_index(x, y)?))
            .filter(|s| **s == Seat::Full)
            .count()
    }
}

impl Display for SeatingWithPart1Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

#[test]
fn test_surrounds() {
    let seating = SeatingWithPart1Rules(
        Seating::from_str(
            "L.LL.LL.LL
LLLL#LL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
        .unwrap(),
    );

    assert_eq!(seating.count_adjacent_occupied(0, 0), 0);
    assert_eq!(seating.count_adjacent_occupied(3, 1), 1);
    assert_eq!(seating.count_adjacent_occupied(0, 1), 0);
}

#[test]
fn test_iterate() {
    let seating = SeatingWithPart1Rules(
        Seating::from_str(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
        .unwrap(),
    );

    let seating = iterate(&seating);
    let target = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
";
    assert_eq!(seating.seating().to_string(), target);

    let seating = iterate(&seating);
    let target = "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##
";

    assert_eq!(seating.seating().to_string(), target);

    let seating = iterate(&seating);
    let target = "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##
";

    assert_eq!(seating.seating().to_string(), target);

    let seating = iterate(&seating);
    let target = "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##
";
    assert_eq!(seating.seating().to_string(), target);
    let seating = iterate(&seating);
    let target = "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##
";

    assert_eq!(seating.seating().to_string(), target);
}

#[test]
fn test_iterate_until_stable() {
    let seating = SeatingWithPart1Rules(
        Seating::from_str(
            "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
        )
        .unwrap(),
    );
    let (fin, iterations) = iterate_until_stable(&seating, &mut |_, _| {}, None);
    assert_eq!(fin.seating().count_occupied_seats(), 37);
    assert_eq!(iterations, 6);
}

#[test]
fn test_many_surround_counts() {
    let scenarios = vec![1, 0, 0, 1, 0, 1, 0, 1, 0]
        .into_iter()
        .permutations(9)
        .map(|seats| (seats.clone(), if seats[4] == 1 { 3 } else { 4 }, (1, 1)));

    fn scenario_to_seating(template: &Vec<u8>) -> SeatingWithPart1Rules {
        SeatingWithPart1Rules(Seating::new(
            3,
            3,
            template
                .iter()
                .map(|x| match x {
                    0 => Seat::Empty,
                    1 => Seat::Full,
                    _ => Seat::Floor,
                })
                .collect(),
        ))
    }

    for (scenario, expected_count, (x, y)) in scenarios {
        let seating = scenario_to_seating(&scenario);
        assert_eq!(
            seating.count_adjacent_occupied(x, y),
            expected_count,
            "Failed \n{} expecting {} around {},{}",
            seating,
            expected_count,
            x,
            y
        );
    }
}

#[test]
fn test_real_input_first_line() {
    let line = "LLLLLLLLLLLLLLL.L..LLLLLL.LLLLLLLLL.LLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLL.LLLL...LLLLLLLLLLL\n";
    let seating = Seating::from_str(line).unwrap();
    assert_eq!(seating.width, 94);
    assert_eq!(seating.height, 1);
    assert_eq!(seating.seats.len(), 94);
    assert_eq!(
        seating.seats,
        line.trim()
            .chars()
            .map(|c| match c {
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => Seat::Full,
            })
            .collect::<Vec<Seat>>()
    );
    assert_eq!(seating.to_string(), line);
}

#[test]
fn test_hayward_input() {
    let input = include_str!("hayward_input.txt");
    let seating = SeatingWithPart1Rules(Seating::from_str(input).unwrap());
    let (fin, _) = iterate_until_stable(&seating, &mut |_, _| {}, None);
    assert_eq!(fin.seating().count_occupied_seats(), 2354);
}

#[test]
fn test_iterate_hayward_1() {
    let input = include_str!("hayward_input.txt");
    let seating = SeatingWithPart1Rules(Seating::from_str(input).unwrap());
    let new = iterate(&seating);
    assert_eq!(
        seating.seating().count_empty_seats(),
        new.seating().count_occupied_seats(),
        "First iteration: all empty seats should always become occupied"
    );
}
