//! This module uses axial coordinates for a hex grid
//! As described at https://www.redblobgames.com/grids/hexagons/

use std::ops::Add;
use std::str::FromStr;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct HexCoordinate {
    q: isize,
    r: isize,
}

impl HexCoordinate {
    pub fn new(q: isize, r: isize) -> HexCoordinate {
        HexCoordinate { q: q, r: r }
    }

    pub fn northeast() -> HexCoordinate {
        HexCoordinate::new(1, -1)
    }

    pub fn north() -> HexCoordinate {
        HexCoordinate::new(0, -1)
    }

    pub fn northwest() -> HexCoordinate {
        HexCoordinate::new(-1, 0)
    }

    pub fn southeast() -> HexCoordinate {
        HexCoordinate::new(1, 0)
    }

    pub fn south() -> HexCoordinate {
        HexCoordinate::new(0, 1)
    }

    pub fn southwest() -> HexCoordinate {
        HexCoordinate::new(-1, 1)
    }

    pub fn q(&self) -> isize {
        self.q
    }

    pub fn r(&self) -> isize {
        self.r
    }

    pub fn s(&self) -> isize {
        -self.q - self.r
    }

    pub fn follow_direction(&self, direction: Direction) -> HexCoordinate {
        match direction {
            Direction::North => self + HexCoordinate::north(),
            Direction::NorthEast => self + HexCoordinate::northeast(),
            Direction::SouthEast => self + HexCoordinate::southeast(),
            Direction::South => self + HexCoordinate::south(),
            Direction::SouthWest => self + HexCoordinate::southwest(),
            Direction::NorthWest => self + HexCoordinate::northwest(),
        }
    }

    pub fn follow_directions<I>(&self, directions: I) -> HexCoordinate
        where I: IntoIterator<Item = Direction>
    {
        directions.into_iter().fold(*self, |r, d| r.follow_direction(d))
    }

    /// Manhattan distance from the origin
    pub fn distance_from_origin(&self) -> usize {
        self.distance_from(&HexCoordinate::new(0, 0))
    }

    /// Manhattan distance from the given other coordinate
    pub fn distance_from(&self, other: &HexCoordinate) -> usize {
        // calculate the absolute values of the distances
        // in cube coordinates
        let x = (self.q() - other.q()).abs() as usize;
        let y = (self.r() - other.r()).abs() as usize;
        let z = (self.s() - other.s()).abs() as usize;

        // the largest such value is the distance
        *([x, y, z].iter().max().unwrap())
    }
}

impl Add for HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add for &'a HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: &'a HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add<&'a HexCoordinate> for HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: &'a HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

impl<'a> Add<HexCoordinate> for &'a HexCoordinate {
    type Output = HexCoordinate;

    fn add(self, other: HexCoordinate) -> HexCoordinate {
        HexCoordinate::new(self.q + other.q, self.r + other.r)
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    North,
    NorthEast,
    SouthEast,
    South,
    SouthWest,
    NorthWest,
}

impl FromStr for Direction {
    type Err = ();
    fn from_str(s: &str) -> Result<Direction, ()> {
        match s.trim() {
            "n" => Ok(Direction::North),
            "ne" => Ok(Direction::NorthEast),
            "nw" => Ok(Direction::NorthWest),
            "s" => Ok(Direction::South),
            "se" => Ok(Direction::SouthEast),
            "sw" => Ok(Direction::SouthWest),
            _ => Err(()),
        }
    }
}


#[cfg(test)]
mod tests {
    mod follow {
        use super::super::*;

        #[test]
        fn follow_sample_1() {
            let directions = vec![Direction::NorthEast, Direction::NorthEast, Direction::NorthEast];
            let result = HexCoordinate::new(0, 0).follow_directions(directions);
            assert_eq!(result, HexCoordinate::new(3, -3));
        }

        #[test]
        fn follow_sample_2() {
            let directions = vec![Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::SouthWest,
                                  Direction::SouthWest];
            let result = HexCoordinate::new(0, 0).follow_directions(directions);
            assert_eq!(result, HexCoordinate::new(0, 0));
        }

        #[test]
        fn follow_sample_3() {
            let directions = vec![Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::South,
                                  Direction::South];
            let result = HexCoordinate::new(0, 0).follow_directions(directions);
            assert_eq!(result, HexCoordinate::new(2, 0));
        }
    }

    mod distance {
        use super::super::*;

        #[test]
        fn distance_sample_1() {
            let directions = vec![Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::NorthEast];
            let result =
                HexCoordinate::new(0, 0).follow_directions(directions).distance_from_origin();
            assert_eq!(result, 4);
        }

        #[test]
        fn distance_sample_2() {
            let directions = vec![Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::SouthWest,
                                  Direction::SouthWest];
            let result =
                HexCoordinate::new(0, 0).follow_directions(directions).distance_from_origin();
            assert_eq!(result, 0);
        }

        #[test]
        fn distance_sample_3() {
            let directions = vec![Direction::NorthEast,
                                  Direction::NorthEast,
                                  Direction::South,
                                  Direction::South];
            let result =
                HexCoordinate::new(0, 0).follow_directions(directions).distance_from_origin();
            assert_eq!(result, 2);
        }

        #[test]
        fn distance_sample_4() {
            let directions = vec![Direction::SouthEast,
                                  Direction::SouthWest,
                                  Direction::SouthEast,
                                  Direction::SouthWest,
                                  Direction::SouthWest];
            let result =
                HexCoordinate::new(0, 0).follow_directions(directions).distance_from_origin();
            assert_eq!(result, 3);
        }
    }
}