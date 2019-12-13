use crate::day::Day;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::ops::{Add, Mul};
use std::rc::Rc;
use std::str::FromStr;

const INPUT: &'static str = "<x=0, y=6, z=1>
<x=4, y=4, z=19>
<x=-11, y=1, z=8>
<x=2, y=19, z=15>";

pub struct Day12 {
    moons: Vec<Moon>,
}

impl Day12 {
    pub fn new() -> Result<Day12, String> {
        Ok(Day12 {
            moons: INPUT.lines().filter_map(parse_moon).collect::<Vec<_>>(),
        })
    }
}

impl Day for Day12 {
    fn part1(&mut self) -> Result<String, String> {
        let moons = self.moons.clone();

        let new_moons = (0..1000).fold(moons, |ms, _| run_step(ms));

        let total_energy: i32 = new_moons.iter().map(|m| m.energy()).sum();

        Ok(format!("Total energy {}", total_energy))
    }

    fn part2(&mut self) -> Result<String, String> {
        Err("not".into())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Moon {
    pos: Vector,
    velocity: Vector,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Moon {
        Moon {
            pos: Vector::new(x, y, z),
            velocity: Vector::new(0, 0, 0),
        }
    }

    #[cfg(test)]
    fn new_with_velocity(x: i32, y: i32, z: i32, vx: i32, vy: i32, vz: i32) -> Moon {
        Moon {
            pos: Vector::new(x, y, z),
            velocity: Vector::new(vx, vy, vz),
        }
    }

    fn adjust_velocity(self, change: &Vector) -> Moon {
        Moon {
            pos: self.pos,
            velocity: self.velocity + change,
        }
    }

    fn apply_velocity(self) -> Moon {
        Moon {
            pos: self.pos + &self.velocity,
            velocity: self.velocity,
        }
    }

    fn energy(&self) -> i32 {
        self.pos.abs_sum() * self.velocity.abs_sum()
    }
}

#[derive(Clone, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x, y, z }
    }

    fn abs_sum(&self) -> i32 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Debug for Vector {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Self) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, other: &Self) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, other: &Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, other: i32) -> Self {
        Vector::new(self.x * other, self.y * other, self.z * other)
    }
}

fn parse_moon(line: &str) -> Option<Moon> {
    lazy_static! {
        static ref RE: Regex = Regex::new("<x=(\\d+), y=(\\d+), z=(\\d+)>").unwrap();
    }

    let m = RE.captures_iter(line).next()?;
    let x = i32::from_str(&m[1]).ok()?;
    let y = i32::from_str(&m[2]).ok()?;
    let z = i32::from_str(&m[3]).ok()?;

    Some(Moon::new(x, y, z))
}

fn velocity(first: &Vector, second: &Vector) -> Vector {
    fn calc(a: i32, b: i32) -> i32 {
        match b.cmp(&a) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }

    Vector::new(
        calc(first.x, second.x),
        calc(first.y, second.y),
        calc(first.z, second.z),
    )
}

fn pair_velocity(first: &Moon, second: &Moon) -> (Vector, Vector) {
    let change = velocity(&first.pos, &second.pos);
    (change.clone(), change * -1)
}

fn adjust_velocities(moons: Vec<Moon>) -> Vec<Moon> {
    // Cheating by using interior mutability here
    // combinations requires something cloneable, which an &mut Moon isn't
    // can't think of a way to do it without this for now, but at least it's contained inside here
    let moon_refs = moons
        .iter()
        .map(|moon| Rc::new(RefCell::new(moon.clone())))
        .collect::<Vec<_>>();

    for pair in moon_refs.iter().combinations(2) {
        let (first, second) = (pair[0], pair[1]);
        let (change1, change2) = pair_velocity(&first.borrow(), &second.borrow());
        first.replace_with(|m| m.clone().adjust_velocity(&change1));
        second.replace_with(|m| m.clone().adjust_velocity(&change2));
    }

    moon_refs
        .into_iter()
        .map(|r| {
            Rc::try_unwrap(r)
                .expect("Shouldn't fail to try_unwrap the Rc")
                .into_inner()
        })
        .collect()
}

fn run_step(moons: Vec<Moon>) -> Vec<Moon> {
    adjust_velocities(moons)
        .into_iter()
        .map(|m| m.apply_velocity())
        .collect()
}

#[test]
fn test_parse_moon() {
    let s = "<x=5, y=8, z=9>";
    let m = parse_moon(s);
    assert_eq!(Some(Moon::new(5, 8, 9)), m);
}

#[test]
fn test_add_vector() {
    let v1 = Vector::new(1, 2, 3);
    let v2 = Vector::new(2, 3, 4);

    let v3 = v1 + v2;

    assert_eq!(v3, Vector::new(3, 5, 7));
}

#[test]
fn test_sample_step_1() {
    let moons = vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];

    let new_moons = run_step(moons);

    assert_eq!(
        new_moons,
        vec![
            Moon::new_with_velocity(2, -1, 1, 3, -1, -1),
            Moon::new_with_velocity(3, -7, -4, 1, 3, 3),
            Moon::new_with_velocity(1, -7, 5, -3, 1, -3),
            Moon::new_with_velocity(2, 2, 0, -1, -3, 1)
        ]
    );
}

#[test]
fn test_sample_full() {
    let moons = vec![
        Moon::new(-1, 0, 2),
        Moon::new(2, -10, -7),
        Moon::new(4, -8, 8),
        Moon::new(3, 5, -1),
    ];

    let new_moons = (0..10).fold(moons, |ms, _| run_step(ms));

    assert_eq!(
        new_moons,
        vec![
            Moon::new_with_velocity(2, 1, -3, -3, -2, 1),
            Moon::new_with_velocity(1, -8, 0, -1, 1, 3),
            Moon::new_with_velocity(3, -6, 1, 3, 2, -3),
            Moon::new_with_velocity(2, 0, 4, 1, -1, -1),
        ]
    );

    let energy: i32 = new_moons.into_iter().map(|e| e.energy()).sum();

    assert_eq!(energy, 179);
}

#[test]
fn test_big_sample_full() {
    let moons = vec![
        Moon::new(-8, 10, 0),
        Moon::new(5, 5, 10),
        Moon::new(2, -7, 3),
        Moon::new(9, -8, -3),
    ];
    let new_moons = (0..100).fold(moons, |ms, _| run_step(ms));

    assert_eq!(
        new_moons,
        vec![
            Moon::new_with_velocity(8, -12, -9, -7, 3, 0),
            Moon::new_with_velocity(13, 16, -3, 3, -11, -5),
            Moon::new_with_velocity(-29, -11, -1, -3, 7, 4),
            Moon::new_with_velocity(16, -13, 23, 7, 1, 1)
        ]
    );

    let energy: i32 = new_moons.into_iter().map(|m| m.energy()).sum();

    assert_eq!(energy, 1940);
}

#[test]
fn test_abs_sum() {
    let v = Vector::new(-5, 6, 7);
    let e = v.abs_sum();
    assert_eq!(e, 5 + 6 + 7);
}

#[test]
fn test_energy() {
    let m = Moon::new_with_velocity(2, 1, -3, -3, -2, 1);
    let e = m.energy();
    assert_eq!(e, 36);
}

#[cfg(test)]
mod propertytests {
    use super::*;
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn velocity_always_bounded(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> bool {
        let v1 = Vector::new(x1, y1, z1);
        let v2 = Vector::new(x2, y2, z2);

        let change = velocity(&v1, &v2);

        (change.x == 1 || change.x == 0 || change.x == -1)
            && (change.y == 1 || change.y == 0 || change.y == -1)
            && (change.z == 1 || change.z == 0 || change.z == -1)
    }

    #[quickcheck]
    fn test_add_velocities(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> bool {
        let v1 = Vector::new(x1, y1, z1);
        let v2 = Vector::new(x2, y2, z2);

        let v3 = v1 + v2;

        v3.x == x1 + x2 && v3.y == y1 + y2 && v3.z == z1 + z2
    }

    #[quickcheck]
    fn test_add_velocities_ref(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> bool {
        let v1 = Vector::new(x1, y1, z1);
        let v2 = Vector::new(x2, y2, z2);

        let v3 = v1 + &v2;

        v3.x == x1 + x2 && v3.y == y1 + y2 && v3.z == z1 + z2
    }

    #[quickcheck]
    fn test_add_velocities_ref2(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> bool {
        let v1 = Vector::new(x1, y1, z1);
        let v2 = Vector::new(x2, y2, z2);

        let v3 = &v1 + &v2;

        v3.x == x1 + x2 && v3.y == y1 + y2 && v3.z == z1 + z2
    }

    #[quickcheck]
    fn test_inverse_change(x1: i32, y1: i32, z1: i32, x2: i32, y2: i32, z2: i32) -> bool {
        let v1 = Vector::new(x1, y1, z1);
        let v2 = Vector::new(x2, y2, z2);

        let change = velocity(&v1, &v2);
        let changeinverse = velocity(&v2, &v1);

        changeinverse == change * -1
    }
}
