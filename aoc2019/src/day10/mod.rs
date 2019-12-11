use crate::day::Day;
use itertools::Itertools;
use num_rational::Rational;
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

    fn location_seeing_most_asteroids(&self) -> (usize, usize, usize) {
        let mut greatest = 0;
        let mut greatest_x = 0;
        let mut greatest_y = 0;

        for (x, y) in self.asteroids.iter().cloned() {
            println!("Examining asteroid {},{}", x, y);
            let visible_from = self.asteroids_visible_from((x, y));
            println!("{} asteroids visible from here", visible_from);
            if visible_from > greatest {
                println!("New high visibility");
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
            // sets are unsorted, always find the top-most leftmost answer if there are equals
            if visible_from == greatest && (x < greatest_x || y < greatest_y) {
                println!("New superior visibility");
                greatest = visible_from;
                greatest_x = x;
                greatest_y = y;
            }
        }

        (greatest, greatest_x, greatest_y)
    }
}

fn gradient_between((x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> Rational {
    if y1 == y2 {
        if x2 >= x1 {
            Rational::from(0)
        } else {
            Rational::new(1, 2)
        }
    } else if x1 == x2 {
        if y2 >= y1 {
            Rational::new(1, 4)
        } else {
            Rational::new(3, 4)
        }
    } else {
        let dx = x2 as isize - x1 as isize;
        let dy = y2 as isize - y1 as isize;
        let quarter_ratio = Rational::new(dbg!(dx), dbg!(dy)) / 8;
        let addon = if dy < 0 {
            Rational::new(1, 2)
        } else {
            Rational::from(0)
        };
        quarter_ratio + addon
    }
}

#[test]
fn test_gradient_between() {
    assert_eq!(
        gradient_between((0, 0), (1, 1)),
        Rational::new(1, 8),
        "0, 0 -> 1, 1 == 1/8"
    );
    assert_eq!(
        gradient_between((0, 0), (1, 0)),
        Rational::from(0),
        "0, 0 -> 1, 0 == 0"
    );
    assert_eq!(
        gradient_between((1, 1), (0, 0)),
        Rational::new(5, 8),
        "1, 1 -> 0, 0: == 5/8"
    );
    assert_eq!(
        gradient_between((1, 1), (2, 0)),
        Rational::new(7, 8),
        "1, 1 -> 2, 0: 7/8"
    );
    assert!(false);
}

fn group_by_gradient_from(
    asteroids: &HashSet<(usize, usize)>,
    origin: (usize, usize),
) -> HashMap<Rational, Vec<(usize, usize)>> {
    asteroids
        .iter()
        .map(|a| (gradient_between(origin, *a), *a))
        .into_group_map()
}

fn visible_from(asteroids: &HashSet<(usize, usize)>, (x, y): (usize, usize)) -> usize {
    let groups = group_by_gradient_from(asteroids, (x, y));
    groups.values().map(|g| g.iter().skip(1).count()).sum()
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
    assert_eq!(result, (310, 11, 13));
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
fn test_simple_sample_2() {
    let map = Map::from_str(
        "#.#
.#.
...",
    )
    .unwrap();
    let result = map.location_seeing_most_asteroids();
    assert_eq!(result, (2, 0, 0));
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
