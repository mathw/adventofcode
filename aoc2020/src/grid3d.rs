use crate::point3d::Point3D;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone)]
pub struct HashMapGrid3D<T> {
    storage: HashMap<Point3D, T>,
}

impl<T> HashMapGrid3D<T> {
    pub fn new() -> HashMapGrid3D<T> {
        HashMapGrid3D {
            storage: HashMap::new(),
        }
    }
}

pub trait Grid3D<T> {
    fn get(&self, p: &Point3D) -> Option<&T>;
    fn set(&mut self, p: Point3D, value: T) -> Option<T>;
    fn get_neighbours<'a, 'b: 'a>(&'a self, p: &Point3D, default_value: &'b T) -> Vec<&'a T>;
    fn simultaneous_apply<F>(&self, f: F) -> Self
    where
        Self: Clone,
        F: Fn(&Point3D) -> T;
    fn get_bounds(
        &self,
    ) -> (
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    );
    fn count_set_elements_equal_to(&self, element: &T) -> usize
    where
        T: Eq;
}

impl<T> Grid3D<T> for HashMapGrid3D<T> {
    fn get(&self, p: &Point3D) -> Option<&T> {
        self.storage.get(p)
    }
    fn set(&mut self, p: Point3D, value: T) -> Option<T> {
        self.storage.insert(p, value)
    }
    fn get_neighbours<'a, 'b: 'a>(&'a self, p: &Point3D, default_value: &'b T) -> Vec<&'a T> {
        p.surrounding_points()
            .into_iter()
            .map(move |point| self.get(&point).unwrap_or_else(|| default_value))
            .collect()
    }
    fn get_bounds(
        &self,
    ) -> (
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    ) {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        let mut min_z = i64::MAX;
        let mut max_z = i64::MIN;
        for point in self.storage.keys() {
            if point.x < min_x {
                min_x = point.x;
            }
            if point.x > max_x {
                max_x = point.x;
            }
            if point.y < min_y {
                min_y = point.y;
            }
            if point.y > max_y {
                max_y = point.y;
            }
            if point.z < min_z {
                min_z = point.z;
            }
            if point.z > max_z {
                max_z = point.z;
            }
        }
        (min_x..=max_x, min_y..=max_y, min_z..=max_z)
    }
    fn simultaneous_apply<F>(&self, f: F) -> Self
    where
        Self: Clone,
        F: Fn(&Point3D) -> T,
    {
        let mut output = self.clone();
        let (xbounds, ybounds, zbounds) = self.get_bounds();
        let xbounds = (xbounds.start() - 1)..=(xbounds.end() + 1);
        let ybounds = (ybounds.start() - 1)..=(ybounds.end() + 1);
        let zbounds = (zbounds.start() - 1)..=(zbounds.end() + 1);

        for x in xbounds {
            for y in ybounds.clone() {
                for z in zbounds.clone() {
                    let here = Point3D::new(x, y, z);
                    let new_value_here = f(&here);
                    output.set(here, new_value_here);
                }
            }
        }
        output
    }
    fn count_set_elements_equal_to(&self, element: &T) -> usize
    where
        T: Eq,
    {
        self.storage.values().filter(|v| *v == element).count()
    }
}

#[test]
fn test_get_neighbours() {
    let mut grid = HashMapGrid3D::new();
    grid.set(Point3D::new(0, 0, 0), true);
    assert_eq!(
        grid.get_neighbours(&Point3D::new(0, 0, 0), &false),
        vec![&false; 26]
    );
    grid.set(Point3D::new(1, 0, 0), true);
    let neighbours = grid.get_neighbours(&Point3D::new(0, 0, 0), &false);
    assert_eq!(neighbours.iter().filter(|e| ***e).count(), 1);
    grid.set(Point3D::new(1, 0, 1), true);
    let neighbours = grid.get_neighbours(&Point3D::new(0, 0, 0), &false);
    assert_eq!(neighbours.iter().filter(|e| ***e).count(), 2);
    grid.set(Point3D::new(1, -1, 1), true);
    let neighbours = grid.get_neighbours(&Point3D::new(0, 0, 0), &false);
    assert_eq!(neighbours.iter().filter(|e| ***e).count(), 3);
    grid.set(Point3D::new(2, -1, 1), true);
    let neighbours = grid.get_neighbours(&Point3D::new(0, 0, 0), &false);
    assert_eq!(neighbours.iter().filter(|e| ***e).count(), 3);
    grid.set(Point3D::new(1, -1, 1), false);
    let neighbours = grid.get_neighbours(&Point3D::new(0, 0, 0), &false);
    assert_eq!(neighbours.iter().filter(|e| ***e).count(), 2);
}
