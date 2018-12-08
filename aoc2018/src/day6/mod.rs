use crate::day::Day;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::mpsc::Sender;
use std::usize;

pub struct Day6 {
    points: Vec<Point>,
}

impl Day6 {
    pub fn new() -> Day6 {
        Day6 {
            points: include_str!("input.txt")
                .lines()
                .map(Point::from_str)
                .collect::<Result<Vec<Point>, ParseError>>()
                .expect("Points should all parse"),
        }
    }
}

impl Day for Day6 {
    fn part1(&mut self, sender: &Sender<String>) {
        let grid = Grid::new(&self.points);

        if let Some(largest_area) = grid.compute_largest_area() {
            sender
                .send(format!("Largest area is {}", largest_area))
                .unwrap();
        } else {
            sender.send("No largest area found".into()).unwrap();
        }
    }

    fn part2(&mut self, sender: &Sender<String>) {}
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn distance_from(&self, other: &Point) -> usize {
        fn distance_between(a: usize, b: usize) -> usize {
            usize::max(a, b) - usize::min(a, b)
        }

        distance_between(self.x, other.x) + distance_between(self.y, other.y)
    }
}

impl FromStr for Point {
    type Err = ParseError;
    fn from_str(input: &str) -> Result<Point, Self::Err> {
        match input
            .split(',')
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
            .as_slice()
        {
            &[x, y] => Ok(Point::new(usize::from_str(x)?, usize::from_str(y)?)),
            _ => Err(ParseError {}),
        }
    }
}

#[derive(Debug)]
struct ParseError;

impl From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> ParseError {
        ParseError {}
    }
}

struct Grid {
    points_x: HashMap<usize, Vec<Point>>,
    points_y: HashMap<usize, Vec<Point>>,
    points: Vec<Point>,
}

impl Grid {
    fn new(i: &Vec<Point>) -> Grid {
        let mut points_x = HashMap::new();
        let mut points_y = HashMap::new();
        let mut points = Vec::new();

        for point in i.into_iter() {
            let entry = points_x.entry(point.x).or_insert(vec![]);
            (*entry).push(point.clone());
            let entry = points_y.entry(point.y).or_insert(vec![]);
            (*entry).push(point.clone());
            points.push(point.clone());
        }

        Grid {
            points_x,
            points_y,
            points,
        }
    }

    fn compute_largest_area(&self) -> Option<usize> {
        let x_max = self.points_x.keys().max()?;
        let y_max = self.points_y.keys().max()?;

        let mut points_areas = HashMap::new();

        for x in 0..x_max + 1 {
            for y in 0..y_max + 1 {
                // if there is a nearest point to this point, record that point's area as going up by one
                if let Some(nearest) = self.find_nearest_point_to(&Point::new(x, y)) {
                    let entry = points_areas.entry(nearest).or_insert(Some(0));
                    if x == 0 || x == *x_max || y == 0 || y == *y_max {
                        // this is an edge point so this area doesn't actually count
                        *entry = None;
                    } else if let Some(a) = *entry {
                        *entry = Some(a + 1);
                    } else {
                        *entry = Some(1);
                    }
                }
            }
        }

        points_areas.values().filter_map(|a| *a).max()
    }

    fn find_nearest_point_to(&self, point: &Point) -> Option<Point> {
        // returns point closest to the given point, or None if there isn't a closest point (i.e. more than one the same distance)
        let mut shortest_distance = usize::MAX;
        let mut current_candidate = None;
        let mut candidates_at_distance = 0;

        for candidate in self.points.iter() {
            let distance = candidate.distance_from(point);
            if distance < shortest_distance {
                shortest_distance = distance;
                current_candidate = Some(candidate);
                candidates_at_distance = 1;
            } else if distance == shortest_distance {
                candidates_at_distance += 1;
            }
        }

        if let Some(candidate) = current_candidate {
            if candidates_at_distance == 1 {
                Some(candidate.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_distance() {
        let a = Point::new(1, 1).distance_from(&Point::new(2, 2));
        assert_eq!(a, 2, "1, 1 to 2, 2 should be 2");
        let a = Point::new(1, 1).distance_from(&Point::new(3, 2));
        assert_eq!(a, 3, "1, 1 to 3, 2 should be 3");
    }

    #[test]
    fn part_one_example() {
        let points = vec![
            Point::new(1, 1),
            Point::new(1, 6),
            Point::new(8, 3),
            Point::new(3, 4),
            Point::new(5, 5),
            Point::new(8, 9),
        ];

        let grid = Grid::new(&points);

        let area = grid.compute_largest_area();

        assert_eq!(area, Some(17), "Largest area in the example should be 17");
    }
}
