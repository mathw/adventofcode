use super::{common::iterate_until_stable, seating::*, traits::*};
use std::{fmt, fmt::Display, time::Duration};

#[cfg(test)]
use super::common::iterate;
#[cfg(test)]
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SeatingWithPart2Rules(pub Seating);

impl SolutionFinder for SeatingWithPart2Rules {
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

impl SeatIterable for SeatingWithPart2Rules {
    fn iterate_seat(&self, x: usize, y: usize, index: usize) -> Seat {
        let seat = self.0.seats[index];
        if seat == Seat::Floor {
            return Seat::Floor;
        }

        let visible = self.visible_occupied_seats_from(x, y);
        match seat {
            Seat::Empty if visible == 0 => Seat::Full,
            Seat::Full if visible >= 5 => Seat::Empty,
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

impl SeatingWithPart2Rules {
    pub fn can_see_occupied_north_of(&self, x: usize, y: usize) -> bool {
        self.0
            .seats
            .chunks_exact(self.0.width)
            .take(y)
            .map(|chunk| chunk[x])
            .rev()
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    pub fn can_see_occupied_south_of(&self, x: usize, y: usize) -> bool {
        self.0
            .seats
            .chunks_exact(self.0.width)
            .skip(y + 1)
            .map(|chunk| chunk[x])
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    pub fn can_see_occupied_east_of(&self, x: usize, y: usize) -> bool {
        if x + 1 == self.0.width {
            return false;
        }
        let start_index = self.0.to_index_unchecked(x + 1, y);
        let end_index = self.0.to_index_unchecked(self.0.width - 1, y);
        self.0.seats[start_index..=end_index]
            .iter()
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    pub fn can_see_occupied_west_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.0.to_index_unchecked(0, y);
        let end_index = self.0.to_index_unchecked(x, y);
        (start_index..end_index)
            .rev()
            .map(|i| self.0.seats[i])
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    pub fn can_see_occupied_southeast_of(&self, x: usize, y: usize) -> bool {
        if x + 1 == self.0.width {
            return false;
        }
        let start_index = self.0.to_index_unchecked(x, y);
        let mut current_index = start_index + self.0.width + 1;
        while current_index < self.0.seats.len() {
            if current_index % self.0.width == 0 {
                // we've wrapped into column 0! won't find anything now
                return false;
            }
            match self.0.seats[current_index] {
                Seat::Full => return true,
                Seat::Empty => return false,
                Seat::Floor => {}
            }
            current_index += self.0.width + 1;
        }
        return false;
    }

    pub fn can_see_occupied_southwest_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.0.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            current_index += self.0.width - 1;

            if current_index >= self.0.seats.len() {
                // off the bottom
                return false;
            }
            match self.0.seats[current_index] {
                Seat::Full => return true,
                Seat::Empty => return false,
                Seat::Floor => {}
            }
            if current_index % self.0.width == 0 {
                // start of line, not going to find anything now
                return false;
            }
        }
    }

    pub fn can_see_occupied_northeast_of(&self, x: usize, y: usize) -> bool {
        let start_index = self.0.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            match current_index.checked_sub(self.0.width - 1) {
                Some(i) => {
                    if i % self.0.width == 0 {
                        // ran over into next line!
                        return false;
                    }

                    current_index = i;
                    match self.0.seats[current_index] {
                        Seat::Full => return true,
                        Seat::Empty => return false,
                        Seat::Floor => {}
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    pub fn can_see_occupied_northwest_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.0.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            match current_index.checked_sub(self.0.width + 1) {
                Some(i) => {
                    current_index = i;
                    match self.0.seats[current_index] {
                        Seat::Full => return true,
                        Seat::Empty => return false,
                        Seat::Floor => {}
                    };
                    if i % self.0.width == 0 {
                        // start of line, not going to find anything now
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    pub fn visible_occupied_seats_from(&self, x: usize, y: usize) -> usize {
        let north = self.can_see_occupied_north_of(x, y);
        let south = self.can_see_occupied_south_of(x, y);
        let east = self.can_see_occupied_east_of(x, y);
        let west = self.can_see_occupied_west_of(x, y);
        let southeast = self.can_see_occupied_southeast_of(x, y);
        let southwest = self.can_see_occupied_southwest_of(x, y);
        let northeast = self.can_see_occupied_northeast_of(x, y);
        let northwest = self.can_see_occupied_northwest_of(x, y);

        [
            north, south, east, west, southeast, southwest, northeast, northwest,
        ]
        .iter()
        .filter(|x| **x)
        .count()
    }
}

impl Display for SeatingWithPart2Rules {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.0.fmt(f)
    }
}

#[test]
fn test_visible_occupied_seats_from() {
    let seat = SeatingWithPart2Rules(
        Seating::from_str(
            ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
        )
        .unwrap(),
    );
    assert_eq!(seat.visible_occupied_seats_from(3, 4), 8);

    let seat = SeatingWithPart2Rules(
        Seating::from_str(
            ".............
.L.L.#.#.#.#.
.............",
        )
        .unwrap(),
    );
    assert_eq!(seat.visible_occupied_seats_from(1, 1), 0);

    let seat = SeatingWithPart2Rules(
        Seating::from_str(
            ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
        )
        .unwrap(),
    );
    assert_eq!(seat.visible_occupied_seats_from(3, 3), 0);
}

#[test]
fn test_iterate_2() {
    let seating = SeatingWithPart2Rules(
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
    assert_eq!(
        seating.to_string(),
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
"
    );

    let seating = iterate(&seating);
    assert_eq!(
        seating.to_string(),
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
"
    );

    let seating = iterate(&seating);
    assert_eq!(
        seating.to_string(),
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
"
    );

    let seating = iterate(&seating);
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );

    let seating = iterate(&seating);
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );

    let seating = iterate(&seating);
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );
}
