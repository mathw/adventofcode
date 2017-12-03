use util::timed_repeatedly;
use std::ops::Add;
use std::collections::HashMap;

pub fn go(reps: usize) {
    // input is a 1-indexed point number
    let input = 325489;

    let work1 = || part1(input - 1);
    let (result1, time1) = timed_repeatedly(reps, work1);
    println!("[{}ms] Distance is {}", time1, result1);

    let work2 = || part2(input as u32);
    let (result2, time2) = timed_repeatedly(reps, work2);
    println!("[{}ms] First filled value greater than input is {}",
             time2,
             result2);
}

fn part1(input: usize) -> i32 {
    get_coords_to(input).manhattan_distance_to_origin()
}

fn part2(input: u32) -> u32 {
    let mut grid = FilledGrid::new();
    grid.first_fill_beyond_value(input)
}

/// Calculates the x, y coordinates for the point `i`, indexed from 0
fn get_coords_to(i: usize) -> Point {
    // Algorithm from https://stackoverflow.com/a/3715915/241544
    let i = i as i32;
    let segment_length = (i as f64).sqrt().round() as i32;
    let steps_to_diagonal = (segment_length.pow(2) - i).abs() - segment_length;
    let calc = |l| {
        ((l + segment_length.pow(2) - i - (segment_length % 2)) as f64 * 0.5f64 *
         (-1i32).pow(segment_length as u32) as f64)
            .floor()
    };

    let x = calc(steps_to_diagonal) as i32;
    let y = calc(-steps_to_diagonal) as i32;

    Point { x: x, y: y }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0, y: 0 }
    }

    fn manhattan_distance_to_origin(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn adjacent_points(&self) -> Vec<Point> {
        let deltas = [Point { x: -1, y: 0 },
                      Point { x: 1, y: 0 },
                      Point { x: -1, y: 1 },
                      Point { x: 0, y: 1 },
                      Point { x: 1, y: 1 },
                      Point { x: -1, y: -1 },
                      Point { x: 0, y: -1 },
                      Point { x: 1, y: -1 }];
        deltas.iter().map(|d| self.clone() + d.clone()).collect()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

struct FilledGrid {
    points: HashMap<Point, u32>,
}

impl FilledGrid {
    fn new() -> FilledGrid {
        FilledGrid { points: HashMap::new() }
    }

    fn first_fill_beyond_value(&mut self, n: u32) -> u32 {
        let mut step = 0;
        loop {
            let value = self.fill_value_for(step);
            if value > n {
                return value;
            } else {
                step += 1;
            }
        }
    }

    fn fill_value_for(&mut self, step: usize) -> u32 {
        let coords_for_this_step = get_coords_to(step);

        if let Some(x) = self.points.get(&coords_for_this_step) {
            return *x;
        }

        if coords_for_this_step == Point::origin() {
            let entry = self.points.entry(Point::origin()).or_insert(1);
            *entry = 1;
            return 1;
        } else {
            self.fill_value_for(step - 1);
            let adjacent_values_sum = coords_for_this_step.adjacent_points()
                .iter()
                .filter_map(|p| self.points.get(p))
                .sum();
            self.points.insert(coords_for_this_step, adjacent_values_sum);
            return adjacent_values_sum;
        }
    }
}

#[test]
fn test_fill_value_for() {
    let mut grid = FilledGrid::new();
    grid.fill_value_for(10);
    assert_eq!(grid.points.get(&Point::origin()), Some(&1), "0");
    assert_eq!(grid.points.get(&Point { x: 1, y: 0 }), Some(&1), "1");
    assert_eq!(grid.points.get(&Point { x: 1, y: 1 }), Some(&2), "2");
    assert_eq!(grid.points.get(&Point { x: 0, y: 1 }), Some(&4), "3");
    assert_eq!(grid.points.get(&Point { x: -1, y: 1 }), Some(&5), "4");
    assert_eq!(grid.points.get(&Point { x: -1, y: 0 }), Some(&10), "5");
    assert_eq!(grid.points.get(&Point { x: -1, y: -1 }), Some(&11), "6");
}

#[test]
fn test_get_coords_for() {
    assert_eq!(get_coords_to(0), Point { x: 0, y: 0 });
    assert_eq!(get_coords_to(1), Point { x: 1, y: 0 });
    assert_eq!(get_coords_to(2), Point { x: 1, y: 1 });
    assert_eq!(get_coords_to(3), Point { x: 0, y: 1 });
    assert_eq!(get_coords_to(4), Point { x: -1, y: 1 });
    assert_eq!(get_coords_to(5), Point { x: -1, y: 0 });
    assert_eq!(get_coords_to(6), Point { x: -1, y: -1 });
    assert_eq!(get_coords_to(7), Point { x: 0, y: -1 });
    assert_eq!(get_coords_to(8), Point { x: 1, y: -1 });
    assert_eq!(get_coords_to(9), Point { x: 2, y: -1 });
    assert_eq!(get_coords_to(10), Point { x: 2, y: 0 });
    assert_eq!(get_coords_to(11), Point { x: 2, y: 1 });
    assert_eq!(get_coords_to(12), Point { x: 2, y: 2 });
}
