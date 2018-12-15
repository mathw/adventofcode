use crate::util::ErrString;
use lazy_static::lazy_static;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

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
}

impl Points {
    fn max_x(&self) -> i32 {
        self.points.iter().map(|p| p.position.x).max().unwrap_or(0)
    }

    fn max_y(&self) -> i32 {
        self.points.iter().map(|p| p.position.y).max().unwrap_or(0)
    }

    fn has_point_at(&self, x: i32, y: i32) -> bool {
        self.points
            .iter()
            .any(|p| p.position.x == x && p.position.y == y)
    }
}

impl fmt::Display for Points {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let x_size = self.max_x();
        let y_size = self.max_y();

        for y in 0..=y_size {
            for x in 0..=x_size {
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
