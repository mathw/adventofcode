use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Point3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[allow(unused)]
impl Point3D {
    pub fn new(x: i64, y: i64, z: i64) -> Point3D {
        Point3D { x, y, z }
    }

    pub fn add_x(self, amount: i64) -> Point3D {
        Point3D {
            x: self.x + amount,
            ..self
        }
    }
    pub fn add_y(self, amount: i64) -> Point3D {
        Point3D {
            y: self.y + amount,
            ..self
        }
    }
    pub fn add_z(self, amount: i64) -> Point3D {
        Point3D {
            z: self.z + amount,
            ..self
        }
    }
    pub fn with_x(self, x: i64) -> Point3D {
        Point3D { x, ..self }
    }
    pub fn with_y(self, y: i64) -> Point3D {
        Point3D { y, ..self }
    }
    pub fn with_z(self, z: i64) -> Point3D {
        Point3D { z, ..self }
    }
    pub fn surrounding_points(&self) -> Vec<Point3D> {
        let mut points = Vec::new();
        for x in &[-1, 0, 1] {
            for y in &[-1, 0, 1] {
                for z in &[-1, 0, 1] {
                    if (*x, *y, *z) == (0, 0, 0) {
                        continue;
                    }
                    points.push(self.clone() + Point3D::new(*x, *y, *z))
                }
            }
        }
        points
    }
}

impl Add for Point3D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[test]
fn test_surrounding_points() {
    let points = Point3D::new(0, 0, 0).surrounding_points();
    assert_eq!(points.len(), 26);
    // yes this still needs to test what those points actually are
}
