use crate::day::Day;
use itertools::Itertools;
use num_rational::Rational;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

pub struct Day10 {
    map: Map,
}

impl Day10 {
    pub fn new() -> Result<Day10, String> {
        Ok(Day10 {
            map: Map::from_str(include_str!("input.txt"))?,
        })
    }
}

impl Day for Day10 {
    fn part1(&mut self) -> Result<String, String> {
        let (greatest, greatest_x, greatest_y) = self.map.location_seeing_most_asteroids();

        Ok(format!(
            "{} asteroids visible from {},{}",
            greatest, greatest_x, greatest_y
        ))
    }

    fn part2(&mut self) -> Result<String, String> {
        Err("Not implemented".into())
    }
}

struct Map {
    asteroids: HashSet<(usize, usize)>,
}

impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Map, String> {
        let lines = input.lines().map(|l| l.trim()).collect::<Vec<&str>>();

        let asteroids = lines
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
            })
            .collect::<HashSet<_>>();

        Ok(Map { asteroids })
    }
}

#[test]
fn test_parse_map() {
    let input = ".#
..";
    let map = Map::from_str(input);

    assert!(map.is_ok());
    let map = map.unwrap();
    assert_eq!(map.asteroids, vec![(1, 0)].into_iter().collect());
}

impl Map {
    fn asteroids_visible_from(&self, location: (usize, usize)) -> usize {
        visible_from(&self.asteroids, location)
    }

    fn location_seeing_most_asteroids_1(&self) -> (usize, usize, usize) {
        let mut greatest = 0;
        let mut greatest_x = 0;
        let mut greatest_y = 0;

        for (x, y) in self.asteroids.iter().cloned() {
            let visible_from = self.asteroids_visible_from((x, y));
            println!("1: {} visible", visible_from);
            if visible_from > greatest {
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
            // sets are unsorted, always find the top-most leftmost answer if there are equals
            if visible_from == greatest && (x < greatest_x || y < greatest_y) {
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
        }

        (greatest, greatest_x, greatest_y)
    }
    fn location_seeing_most_asteroids(&self) -> (usize, usize, usize) {
        let mut greatest = 0;
        let mut greatest_x = 0;
        let mut greatest_y = 0;

        for (x, y) in self.asteroids.iter().cloned() {
            let visible_from = asteroids_visible_from(&self.asteroids, (x, y)).len();
            println!("2: {} visible", visible_from);
            if visible_from > greatest {
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
            // sets are unsorted, always find the top-most leftmost answer if there are equals
            if visible_from == greatest && (x < greatest_x || y < greatest_y) {
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
        }

        (greatest, greatest_x, greatest_y)
    }
}

fn angle_between((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> f32 {
    let dx = x2 as isize - x1 as isize;
    let dy = y2 as isize - y1 as isize;
    let angle = (dy as f32).atan2(dx as f32) * 180f32 / std::f32::consts::PI;
    let angle = if angle < 0f32 {
        angle + 450.0
    } else {
        angle + 90.0
    };
    let angle = if angle >= 360.0 { angle - 360.0 } else { angle };
    angle
}

fn visible_from(asteroids: &HashSet<(usize, usize)>, (x, y): (usize, usize)) -> usize {
    let angles = asteroids
        .iter()
        .filter(|(ax, ay)| !(*ax == x && *ay == y))
        .map(|(ax, ay)| angle_between((x, y), (*ax, *ay)))
        .collect::<Vec<f32>>();

    let mut unique_angles = Vec::new();
    for angle in angles {
        if !unique_angles.contains(&angle) {
            unique_angles.push(angle);
        } else {
        }
    }

    unique_angles.len()
}

#[derive(PartialEq, Debug, Clone)]
struct AsteroidInfo {
    location: (usize, usize),
    angle: f32,
    distance: f32,
}

impl AsteroidInfo {
    fn new(asteroid: (usize, usize), relative_to: (usize, usize)) -> AsteroidInfo {
        AsteroidInfo {
            location: asteroid,
            angle: angle_between(relative_to, asteroid),
            distance: distance_between(asteroid, relative_to),
        }
    }
}

fn asteroids_visible_from(
    asteroids: &HashSet<(usize, usize)>,
    (x, y): (usize, usize),
) -> Vec<AsteroidInfo> {
    let all_asteroids = asteroids
        .iter()
        .cloned()
        .map(|asteroid| AsteroidInfo::new(asteroid, (x, y)))
        .collect::<Vec<AsteroidInfo>>();

    for asteroid in &all_asteroids {
        println!("{:?}", asteroid);
    }

    let mut done_angles = Vec::new();

    let mut visible = Vec::new();
    for asteroid in &all_asteroids {
        if asteroid.location == (x, y) {
            continue;
        }
        if done_angles.contains(&asteroid.angle) {
            continue;
        }
        let mut this_angle = all_asteroids
            .iter()
            .filter(|a| a.angle == asteroid.angle)
            .collect::<Vec<_>>();
        println!("this angle {} has {}", asteroid.angle, this_angle.len());
        this_angle.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .unwrap_or(Ordering::Equal)
        });
        visible.push(this_angle[0].clone());
        done_angles.push(asteroid.angle);
    }

    visible
}

fn distance_between((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> f32 {
    let dx = (x1 as isize - x2 as isize).abs() as f32;
    let dy = (y1 as isize - y2 as isize).abs() as f32;
    (dx * dx) + (dy * dy).sqrt()
}

fn asteroids_grouped_by_angle_from(asteroids: &HashSet<(usize, usize)>, (x, y): (usize, usize)) {
    let mut asteroids_with_angle_and_distance = asteroids
        .iter()
        .filter(|(ax, ay)| !(*ax == x && *ay == y))
        .map(|(ax, ay)| {
            (
                (ax, ay),
                angle_between((x, y), (*ax, *ay)),
                distance_between((x, y), (*ax, *ay)),
            )
        })
        .collect::<Vec<_>>();

    asteroids_with_angle_and_distance.sort_by(|(_, angle1, distance1), (_, angle2, distance2)| {
        match angle1.partial_cmp(angle2).unwrap() {
            Ordering::Equal => distance1.partial_cmp(distance2).unwrap(),
            o => o,
        }
    });
}

#[test]
fn test_first_sample() {
    let map = Map::from_str(
        "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####",
    )
    .unwrap();
    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (33, 5, 8));
}

#[test]
fn test_second_sample() {
    let map = Map::from_str(
        "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.",
    )
    .unwrap();

    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (35, 1, 2));
}

#[test]
fn test_third_sample() {
    let map = Map::from_str(
        ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..",
    )
    .unwrap();

    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (41, 6, 3));
}

#[test]
fn test_fourth_sample() {
    let map = Map::from_str(
        ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
    )
    .unwrap();

    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (210, 11, 13));
}

#[test]
fn test_simple_sample() {
    let map = Map::from_str(
        "#.
..",
    )
    .unwrap();

    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (0, 0, 0));
}

#[test]
fn test_zeroth_sample() {
    let map = Map::from_str(
        ".#..#
.....
#####
....#
...##",
    )
    .unwrap();

    assert_eq!(map.location_seeing_most_asteroids(), (8, 3, 4));
}
