use std::ops::Add;
use std::str::FromStr;
use regex::Regex;

pub fn go() {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new(x: i32, y: i32, z: i32) -> Vector {
        Vector { x: x, y: y, z: z }
    }
}

impl Add for Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Vector {
        Vector::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Particle {
    position: Vector,
    acceleration: Vector,
    velocity: Vector,
}

impl Particle {
    fn new(pos: Vector, acc: Vector, vel: Vector) -> Particle {
        Particle {
            position: pos,
            acceleration: acc,
            velocity: vel,
        }
    }

    fn tick(&self) -> Particle {
        let velocity = self.velocity + self.acceleration;
        let position = self.position + self.velocity;

        Particle::new(position, self.acceleration, velocity)
    }

    fn distance_from_origin(&self) -> i32 {
        self.position.x.abs() + self.position.y.abs() + self.position.z.abs()
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Particle, ()> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
        }

        let caps = RE.captures(s);
        match caps {
            None => Err(()),
            Some(c) => {
                let px = i32::from_str(&c[0]).map_err(|_| ())?;
                let py = i32::from_str(&c[1]).map_err(|_| ())?;
                let pz = i32::from_str(&c[2]).map_err(|_| ())?;

                let vx = i32::from_str(&c[3]).map_err(|_| ())?;
                let vy = i32::from_str(&c[4]).map_err(|_| ())?;
                let vz = i32::from_str(&c[5]).map_err(|_| ())?;

                let ax = i32::from_str(&c[6]).map_err(|_| ())?;
                let ay = i32::from_str(&c[7]).map_err(|_| ())?;
                let az = i32::from_str(&c[8]).map_err(|_| ())?;

                Ok(Particle::new(Vector::new(px, py, pz), Vector::new(ax, ay, az), Vector::new(vx, vy, vz)))
            }
        }
    }
}
