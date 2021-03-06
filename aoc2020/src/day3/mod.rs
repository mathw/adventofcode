use crate::dayerror::DayError;
use image::{Rgb, RgbImage};
use rayon::prelude::*;
use std::{iter, str::FromStr};

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let trees = Trees::from_str(input)?;
    let count = trees_for_gradient(&trees, 3, 1);
    Ok(format!("You encounter {} trees", count))
}

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let trees = Trees::from_str(input)?;

    let gradients = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let result = gradients
        .par_iter()
        .map(|&(right, down)| {
            render_with_gradient(&trees, right, down);
            let result = trees_for_gradient(&trees, right, down);
            println!("Rendered {} by {}: {} trees", right, down, result);
            result
        })
        .product::<usize>();

    Ok(format!("The answer is {}", result))
}

struct Trees {
    width: usize,
    height: usize,
    data: Vec<bool>,
}

impl Trees {
    fn is_tree_at(&self, x: usize, y: usize) -> bool {
        self.data[self.index_of(x, y)]
    }

    fn trim_x(&self, x: usize) -> usize {
        x % self.width
    }

    fn trim_y(&self, y: usize) -> usize {
        y % self.height
    }
    fn index_of(&self, x: usize, y: usize) -> usize {
        self.trim_x(x) + (self.trim_y(y) * self.width)
    }
}

fn coords_for_gradient(right: usize, down: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut x = 0;
    let mut y = 0;
    iter::repeat_with(move || {
        x += right;
        y += down;
        (x, y)
    })
}

fn trees_for_gradient(trees: &Trees, right: usize, down: usize) -> usize {
    coords_for_gradient(right, down)
        .take_while(|&(_, y)| y < trees.height)
        .map(move |(x, y)| trees.is_tree_at(x, y))
        .filter(|&t| t)
        .count()
}

impl FromStr for Trees {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s
            .lines()
            .map(|l| l.trim())
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => true,
                    })
                    .collect()
            })
            .collect::<Vec<Vec<bool>>>();
        let height = lines.len();
        if height < 1 {
            return Err(DayError::InputParseError("No lines in input".to_owned()));
        }
        let width = lines[0].len();
        let data = lines
            .into_iter()
            .flat_map(|l| l.into_iter())
            .collect::<Vec<bool>>();
        Ok(Trees {
            width,
            height,
            data,
        })
    }
}

fn render_with_gradient(trees: &Trees, right: usize, down: usize) {
    let coords = coords_for_gradient(right, down)
        .take_while(|&(_, y)| y < trees.height)
        .collect::<Vec<_>>();
    let max_x = coords[coords.len() - 1].0;

    let mut img = RgbImage::new(max_x as u32 + 1, trees.height as u32);

    for y in 0..trees.height {
        for x in 0..=max_x {
            let is_visited = coords.contains(&(x, y));
            let has_tree = trees.is_tree_at(x, y);
            let c = match (is_visited, has_tree) {
                (true, true) => Rgb([255, 0, 0]),
                (true, false) => Rgb([128, 128, 128]),
                (false, true) => Rgb([0, 128, 0]),
                (false, false) => Rgb([0, 0, 0]),
            };
            img.put_pixel(x as u32, y as u32, c);
        }
    }

    let filename = format!("trees-{}-{}.png", right, down);
    img.save(filename).expect("Couldn't save image");
}

#[cfg(test)]
static TEST_INPUT: &str = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

#[test]
fn test_coords_for_gradient() {
    let coords = coords_for_gradient(3, 1).take(3).collect::<Vec<_>>();
    let expected = vec![(3, 1), (6, 2), (9, 3)];
    assert_eq!(coords, expected);
}

#[test]
fn test_sample_pt1() {
    let trees = Trees::from_str(TEST_INPUT).expect("Input should parse");
    assert_eq!(trees.width, 11);
    assert_eq!(trees.height, 11);
    let count = trees_for_gradient(&trees, 3, 1);
    assert_eq!(count, 7, "Should meet 7 trees");
}

#[test]
fn test_sample_parse() {
    let trees = Trees::from_str(TEST_INPUT).expect("Input should parse");
    assert_eq!(trees.width, 11);
    assert_eq!(trees.height, 11);
    assert!(!trees.is_tree_at(0, 0), "Should not be tree at 0,0");
    assert!(trees.is_tree_at(6, 2), "Should be tree at 6,2");
}

#[test]
fn test_sample_pt2() {
    let trees = Trees::from_str(TEST_INPUT).expect("Input should parse");
    let count11 = trees_for_gradient(&trees, 1, 1);
    assert_eq!(count11, 2);
    let count31 = trees_for_gradient(&trees, 3, 1);
    assert_eq!(count31, 7);
    let count51 = trees_for_gradient(&trees, 5, 1);
    assert_eq!(count51, 3);
    let count71 = trees_for_gradient(&trees, 7, 1);
    assert_eq!(count71, 4);
    let count12 = trees_for_gradient(&trees, 1, 2);
    assert_eq!(count12, 2);
    let answer = count31 * count11 * count51 * count71 * count12;
    assert_eq!(answer, 336);
}

#[test]
fn test_overlaps() {
    let trees = Trees::from_str(TEST_INPUT).expect("Input should parse");
    assert_eq!(trees.is_tree_at(10, 0), false);
    assert_eq!(trees.is_tree_at(11, 0), false);
    assert_eq!(trees.is_tree_at(12, 0), false);
    assert_eq!(trees.is_tree_at(13, 0), true);
    assert_eq!(trees.is_tree_at(14, 0), true);
    assert_eq!(trees.is_tree_at(25, 0), true);
    assert_eq!(trees.is_tree_at(0, 10), false);
    assert_eq!(trees.is_tree_at(1, 10), true);
    assert_eq!(trees.is_tree_at(1, 11), false);
    assert_eq!(trees.is_tree_at(2, 11), true);
    assert_eq!(trees.is_tree_at(2, 22), true);
}
