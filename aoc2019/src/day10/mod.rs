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
        let (_, greatest_x, greatest_y) = self.map.location_seeing_most_asteroids();
        let order = self.map.order_of_destruction((greatest_x, greatest_y));
        let answer = order.get(199);
        answer
            .ok_or(format!(
                "there was no 200th answer only {} of them",
                order.len()
            ))
            .map(|a| format!("The 200th asteroid destroyed is ({}, {})", a.0, a.1))
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
        asteroids_visible_from(&self.asteroids, location).len()
    }

    fn location_seeing_most_asteroids(&self) -> (usize, usize, usize) {
        let mut greatest = 0;
        let mut greatest_x = 0;
        let mut greatest_y = 0;

        for (x, y) in self.asteroids.iter().cloned() {
            let visible_from = asteroids_visible_from(&self.asteroids, (x, y)).len();
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

    fn order_of_destruction(&self, location: (usize, usize)) -> Vec<(usize, usize)> {
        asteroids_destroyed_from(&self.asteroids, location)
    }

    fn render_with_location_and_highlight(
        &self,
        location: (usize, usize),
        highlight: (usize, usize),
    ) -> String {
        let max_x = self.asteroids.iter().map(|a| a.0).max().unwrap();
        let max_y = self.asteroids.iter().map(|a| a.1).max().unwrap();
        let mut string = String::new();
        for y in 0..=max_y {
            let mut line = String::new();
            for x in 0..=max_x {
                if location == (x, y) {
                    line.push('X');
                } else if highlight == (x, y) {
                    line.push('*');
                } else if self.asteroids.contains(&(x, y)) {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            line.push('\n');
            string.push_str(&line);
        }
        string
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

fn asteroids_destroyed_from(
    asteroids: &HashSet<(usize, usize)>,
    location: (usize, usize),
) -> Vec<(usize, usize)> {
    let mut remaining_asteroids = asteroids.clone();
    let mut destroyed = Vec::new();

    let mut visible = asteroids_visible_from(&remaining_asteroids, location);
    while !visible.is_empty() {
        visible.sort_by(|a, b| a.angle.partial_cmp(&b.angle).unwrap());
        for v in &visible {
            destroyed.push(v.location.clone());
            remaining_asteroids.remove(&v.location);
        }
        visible = asteroids_visible_from(&remaining_asteroids, location);
    }

    destroyed
}

fn distance_between((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> f32 {
    let dx = (x1 as isize - x2 as isize).abs() as f32;
    let dy = (y1 as isize - y2 as isize).abs() as f32;
    (dx * dx) + (dy * dy).sqrt()
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

#[test]
fn test_small_destruction() {
    let map = Map::from_str(
        ".#....#####...#..
    ##...##.#####..##
    ##...#...#.#####.
    ..#.....X...###..
    ..#.#.....#....##",
    )
    .unwrap();

    let order = map.order_of_destruction((8, 3));
    assert_eq!(order[0], (8, 1));
    assert_eq!(order[1], (9, 0));
    assert_eq!(order[8], (15, 1));
}

#[test]
fn test_angle_between() {
    let angle1 = angle_between((8, 3), (10, 0));
    let angle2 = angle_between((8, 3), (9, 1));
    assert_ne!(angle1, angle2, "These two angles must be different");
}
