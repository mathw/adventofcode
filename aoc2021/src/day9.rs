use crate::day::{DayResult, PartResult};
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let heightmap = HeightMap::from_str(include_str!("inputs/day9.txt"))?;
    let part1 = part1(&heightmap);
    Ok(DayResult::new(
        PartResult::Success(format!("Total risk level {}", part1)),
        PartResult::NotImplemented,
    ))
}

struct HeightMap {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<u8> {
        Some(*self.data.get(self.index_of(x, y)?)?)
    }

    fn surrounding<'a>(&'a self, x: usize, y: usize) -> impl Iterator<Item = u8> + 'a {
        self.surrounding_coords(x, y)
            .filter_map(|(cx, cy)| self.get(cx, cy))
    }

    fn surrounding_coords<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut coords = [None; 4];
        coords[0] = if x == 0 { None } else { Some((x - 1, y)) };
        coords[1] = if x >= self.width {
            None
        } else {
            Some((x + 1, y))
        };
        coords[2] = if y == 0 { None } else { Some((x, y - 1)) };
        coords[3] = if y >= self.height {
            None
        } else {
            Some((x, y + 1))
        };
        coords.into_iter().filter_map(|c| c)
    }

    fn is_low_point(&self, x: usize, y: usize) -> Option<bool> {
        let value = self.get(x, y)?;
        Some(self.surrounding(x, y).all(|v| value < v))
    }

    fn all_low_point_values<'a>(&'a self) -> impl Iterator<Item = u8> + 'a {
        self.all_point_coordinates().filter_map(|(x, y)| {
            if self.is_low_point(x, y)? {
                self.get(x, y)
            } else {
                None
            }
        })
    }

    fn all_point_coordinates<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    }
}

impl FromStr for HeightMap {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s
            .lines()
            .map(|l| l.trim())
            .filter(|l| l.len() > 0)
            .collect();
        if lines.len() == 0 {
            return Err(format!("No input"));
        }
        if lines.iter().any(|l| l.len() != lines[0].len()) {
            return Err(format!("Not all lines are the same length"));
        }

        let width = lines[0].len();
        let height = lines.len();
        let data = lines
            .into_iter()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '0' => Some(0),
                '1' => Some(1),
                '2' => Some(2),
                '3' => Some(3),
                '4' => Some(4),
                '5' => Some(5),
                '6' => Some(6),
                '7' => Some(7),
                '8' => Some(8),
                '9' => Some(9),
                _ => None,
            })
            .collect::<Option<Vec<u8>>>()
            .ok_or(format!("Invalid digit in input data"))?;

        Ok(HeightMap {
            width,
            height,
            data,
        })
    }
}

fn part1(heightmap: &HeightMap) -> u32 {
    heightmap.all_low_point_values().map(|h| h as u32 + 1).sum()
}

#[test]
fn test_part1_sample() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let heightmap = HeightMap::from_str(input).expect("Should parse");
    let risk_level = part1(&heightmap);
    assert_eq!(risk_level, 15);
}
