use crate::day::Day;
use crate::util::ErrString;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::i32;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day10 {
    iteration: usize,
}

impl Default for Day10 {
    fn default() -> Self {
        Day10 { iteration: 0 }
    }
}

impl Day for Day10 {
    fn part1(&mut self, sender: &Sender<String>) {
        let input = include_str!("input.txt");
        let mut points = Points::new(
            input
                .trim()
                .lines()
                .map(Point::from_str)
                .collect::<Result<Vec<Point>, String>>()
                .expect("POints should parse"),
        );

        let mut iteration = 0;
        let mut last_width = i32::MAX;
        let mut last_render = "".to_owned();

        loop {
            points.step();
            if points.width() > last_width {
                break;
            }
            // no point wasting time rendering when it's still way too wide
            if points.width() < 80 {
                last_render = points.to_string();
            }
            iteration += 1;
            last_width = points.width();
        }

        sender.send(format!("{}", last_render)).unwrap();

        self.iteration = iteration;
    }

    fn part2(&mut self, sender: &Sender<String>) {
        sender.send(format!("{} seconds", self.iteration)).unwrap();
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn new(x: i32, y: i32) -> Vector {
        Vector { x, y }
    }

    fn translate_by(&self, velocity: &Vector) -> Vector {
        Vector {
            x: self.x + velocity.x,
            y: self.y + velocity.y,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Point {
    position: Vector,
    velocity: Vector,
}

impl Point {
    fn new(x: i32, y: i32, vx: i32, vy: i32) -> Point {
        Point {
            position: Vector::new(x, y),
            velocity: Vector::new(vx, vy),
        }
    }

    fn step(&mut self) {
        self.position = self.position.translate_by(&self.velocity);
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(input: &str) -> Result<Point, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"position=<\s*(-?\d+),\s*(-?\d+)> velocity=<\s*(-?\d+),\s*(-?\d+)>")
                    .unwrap();
        }

        if let Some(cap) = RE.captures_iter(input).next() {
            let x = i32::from_str(&cap[1]).err_string()?;
            let y = i32::from_str(&cap[2]).err_string()?;
            let vx = i32::from_str(&cap[3]).err_string()?;
            let vy = i32::from_str(&cap[4]).err_string()?;

            Ok(Point::new(x, y, vx, vy))
        } else {
            Err("Unable to match line".into())
        }
    }
}

struct Points {
    points: Vec<Point>,
    max_x: i32,
    max_y: i32,
    min_x: i32,
    min_y: i32,
}

impl Points {
    fn new<I: IntoIterator<Item = Point>>(i: I) -> Points {
        let mut points = vec![];

        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut min_y = i32::MAX;
        let mut min_x = i32::MAX;

        for point in i.into_iter() {
            if max_x < point.position.x {
                max_x = point.position.x;
            }
            if max_y < point.position.y {
                max_y = point.position.y;
            }
            if min_x > point.position.x {
                min_x = point.position.x;
            }
            if min_y > point.position.y {
                min_y = point.position.y;
            }
            points.push(point);
        }

        Points {
            points,
            max_x,
            max_y,
            min_x,
            min_y,
        }
    }

    fn max_x(&self) -> i32 {
        self.max_x
    }

    fn min_x(&self) -> i32 {
        self.min_x
    }

    fn max_y(&self) -> i32 {
        self.max_y
    }

    fn min_y(&self) -> i32 {
        self.min_y
    }

    fn width(&self) -> i32 {
        (self.max_x - self.min_x).abs()
    }

    fn has_point_at(&self, x: i32, y: i32) -> bool {
        self.points
            .iter()
            .any(|p| p.position.x == x && p.position.y == y)
    }

    fn step(&mut self) {
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        let mut min_y = i32::MAX;
        let mut min_x = i32::MAX;

        for point in self.points.iter_mut() {
            point.step();

            if max_x < point.position.x {
                max_x = point.position.x;
            }
            if max_y < point.position.y {
                max_y = point.position.y;
            }
            if min_x > point.position.x {
                min_x = point.position.x;
            }
            if min_y > point.position.y {
                min_y = point.position.y;
            }
        }

        self.max_x = max_x;
        self.max_y = max_y;
        self.min_x = min_x;
        self.min_y = min_y;
    }
}

impl fmt::Display for Points {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let x_size = self.max_x();
        let y_size = self.max_y();
        let min_x = self.min_x();
        let min_y = self.min_y();

        for y in min_y..=y_size {
            for x in min_x..=x_size {
                if self.has_point_at(x, y) {
                    write!(fmt, "#")?;
                } else {
                    write!(fmt, ".")?;
                }
            }
            writeln!(fmt, "")?;
        }

        Ok(())
    }
}

#[test]
fn points_parse() {
    let input = "position=<-3,  6> velocity=< 2, -1>";
    let point = Point::from_str(input);

    assert_eq!(point, Ok(Point::new(-3, 6, 2, -1)));
}

#[test]
fn part_one_example() {
    let input = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";
    let points = input
        .trim()
        .lines()
        .map(Point::from_str)
        .collect::<Result<Vec<Point>, String>>();

    assert_eq!(points.is_ok(), true);

    let mut points = Points::new(points.unwrap());

    println!("{}", points);

    points.step();
    points.step();
    points.step();

    let goal = "#...#..###
#...#...#.
#...#...#.
#####...#.
#...#...#.
#...#...#.
#...#...#.
#...#..###
";

    assert_eq!(points.to_string(), goal);
}
