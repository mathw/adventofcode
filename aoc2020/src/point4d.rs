use std::ops::Add;

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub struct Point4D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub w: i64,
}

impl Point4D {
    pub fn new(x: i64, y: i64, z: i64, w: i64) -> Point4D {
        Point4D { x, y, z, w }
    }

    pub fn add_x(self, amount: i64) -> Point4D {
        Point4D {
            x: self.x + amount,
            ..self
        }
    }
    pub fn add_y(self, amount: i64) -> Point4D {
        Point4D {
            y: self.y + amount,
            ..self
        }
    }
    pub fn add_z(self, amount: i64) -> Point4D {
        Point4D {
            z: self.z + amount,
            ..self
        }
    }
    pub fn add_w(self, amount: i64) -> Point4D {
        Point4D {
            w: self.w + amount,
            ..self
        }
    }
    pub fn with_x(self, x: i64) -> Point4D {
        Point4D { x, ..self }
    }
    pub fn with_y(self, y: i64) -> Point4D {
        Point4D { y, ..self }
    }
    pub fn with_z(self, z: i64) -> Point4D {
        Point4D { z, ..self }
    }
    pub fn with_w(self, w: i64) -> Point4D {
        Point4D { w, ..self }
    }
    pub fn surrounding_points(&self) -> Vec<Point4D> {
        let mut points = Vec::new();
        for x in &[-1, 0, 1] {
            for y in &[-1, 0, 1] {
                for z in &[-1, 0, 1] {
                    for w in &[-1, 0, 1] {
                        if (*x, *y, *z, *w) == (0, 0, 0, 0) {
                            continue;
                        }
                        points.push(self.clone() + Point4D::new(*x, *y, *z, *w))
                    }
                }
            }
        }
        points
    }
}

impl Add for Point4D {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point4D {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

#[test]
fn test_surrounding_points() {
    let points = Point4D::new(0, 0, 0, 0).surrounding_points();
    assert_eq!(points.len(), 80);
    // yes this still needs to test what those points actually are
}
