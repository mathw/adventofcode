use crate::point4d::Point4D;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Clone)]
pub struct HashMapGrid4D<T> {
    storage: HashMap<Point4D, T>,
}

impl<T> HashMapGrid4D<T> {
    pub fn new() -> HashMapGrid4D<T> {
        HashMapGrid4D {
            storage: HashMap::new(),
        }
    }
}

pub trait Grid4D<T> {
    fn get(&self, p: &Point4D) -> Option<&T>;
    fn set(&mut self, p: Point4D, value: T) -> Option<T>;
    fn get_neighbours<'a, 'b: 'a>(&'a self, p: &Point4D, default_value: &'b T) -> Vec<&'a T>;
    fn simultaneous_apply<F>(&self, f: F) -> Self
    where
        Self: Clone,
        F: Fn(&Point4D) -> T;
    fn get_bounds(
        &self,
    ) -> (
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
        RangeInclusive<i64>,
    );
    fn count_set_elements_equal_to(&self, element: &T) -> usize
    where
        T: Eq;
}

impl<T> Grid4D<T> for HashMapGrid4D<T> {
    fn get(&self, p: &Point4D) -> Option<&T> {
        self.storage.get(p)
    }
    fn set(&mut self, p: Point4D, value: T) -> Option<T> {
        self.storage.insert(p, value)
    }
    fn get_neighbours<'a, 'b: 'a>(&'a self, p: &Point4D, default_value: &'b T) -> Vec<&'a T> {
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
        RangeInclusive<i64>,
    ) {
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        let mut min_z = i64::MAX;
        let mut max_z = i64::MIN;
        let mut min_w = i64::MAX;
        let mut max_w = i64::MIN;
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
            if point.w < min_w {
                min_w = point.w;
            }
            if point.w > max_w {
                max_w = point.w;
            }
        }
        (min_x..=max_x, min_y..=max_y, min_z..=max_z, min_w..=max_w)
    }
    fn simultaneous_apply<F>(&self, f: F) -> Self
    where
        Self: Clone,
        F: Fn(&Point4D) -> T,
    {
        let mut output = self.clone();
        let (xbounds, ybounds, zbounds, wbounds) = self.get_bounds();
        let xbounds = (xbounds.start() - 1)..=(xbounds.end() + 1);
        let ybounds = (ybounds.start() - 1)..=(ybounds.end() + 1);
        let zbounds = (zbounds.start() - 1)..=(zbounds.end() + 1);
        let wbounds = (wbounds.start() - 1)..=(wbounds.end() + 1);

        for x in xbounds {
            for y in ybounds.clone() {
                for z in zbounds.clone() {
                    for w in wbounds.clone() {
                        let here = Point4D::new(x, y, z, w);
                        let new_value_here = f(&here);
                        output.set(here, new_value_here);
                    }
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
