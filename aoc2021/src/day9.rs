use crate::day::{DayResult, PartResult};
use std::collections::HashSet;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let heightmap = HeightMap::from_str(include_str!("inputs/day9.txt"))?;
    let part1 = part1(&heightmap);
    let part2 = part2(&heightmap);
    Ok(DayResult::new(
        PartResult::Success(format!("Total risk level {}", part1)),
        PartResult::Success(format!("Product of three largest basins {}", part2)),
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
        coords[1] = if x >= self.width - 1 {
            None
        } else {
            Some((x + 1, y))
        };
        coords[2] = if y == 0 { None } else { Some((x, y - 1)) };
        coords[3] = if y >= self.height - 1 {
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
        self.all_low_point_coordinates()
            .filter_map(|(x, y)| self.get(x, y))
    }

    fn all_low_point_coordinates<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.all_point_coordinates().filter_map(|(x, y)| {
            if self.is_low_point(x, y)? {
                Some((x, y))
            } else {
                None
            }
        })
    }

    fn all_point_coordinates<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.height).flat_map(|y| (0..self.width).map(move |x| (x, y)))
    }

    fn basin_coords_from_low_point(&self, x: usize, y: usize) -> Option<HashSet<(usize, usize)>> {
        fn recurse(fc: &mut HashSet<(usize, usize)>, cx: usize, cy: usize, this: &HeightMap) {
            for (ccx, ccy) in this.surrounding_basin_candidates(cx, cy) {
                if fc.insert((ccx, ccy)) {
                    recurse(fc, ccx, ccy, this)
                }
            }
        }
        if !self.is_low_point(x, y)? {
            return None;
        }
        let mut found_coords = HashSet::new();
        found_coords.insert((x, y));

        for (cx, cy) in self.surrounding_basin_candidates(x, y) {
            if found_coords.insert((cx, cy)) {
                recurse(&mut found_coords, cx, cy, self);
            }
        }

        Some(found_coords)
    }

    fn surrounding_basin_candidates<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let value = self.get(x, y).expect("Valid coordinate must be provided");
        self.surrounding_coords(x, y).filter(move |(px, py)| {
            let v = self
                .get(*px, *py)
                .expect("Invalid coordinate from surrounding_coords");
            v >= value && v < 9
        })
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

fn part2(heightmap: &HeightMap) -> u64 {
    let mut all_basins: Vec<u64> = heightmap
        .all_low_point_coordinates()
        .map(|(x, y)| {
            heightmap
                .basin_coords_from_low_point(x, y)
                .expect("Got a low point that isn't a low point")
                .len() as u64
        })
        .collect();
    all_basins.sort();
    all_basins.into_iter().rev().take(3).product()
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

#[test]
fn test_part2_sample() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let heightmap = HeightMap::from_str(input).expect("Should parse");
    let topleft = heightmap.basin_coords_from_low_point(1, 0).unwrap();
    assert_eq!(topleft.len(), 3);
    let topright = heightmap.basin_coords_from_low_point(9, 0).unwrap();
    assert_eq!(topright.len(), 9);
    let middle = heightmap.basin_coords_from_low_point(2, 2).unwrap();
    assert_eq!(middle.len(), 14);
    let bottom = heightmap.basin_coords_from_low_point(6, 4).unwrap();
    assert_eq!(bottom.len(), 9);
}

#[test]
fn test_part2_sample_full() {
    let input = "2199943210
3987894921
9856789892
8767896789
9899965678";

    let heightmap = HeightMap::from_str(input).expect("Should parse");
    let result = part2(&heightmap);
    assert_eq!(result, 1134);
}
