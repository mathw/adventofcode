use std::fmt::{Debug, Formatter};

#[derive(Clone, Eq, PartialEq)]
pub struct Grid<T> {
    data: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Grid<T>
    where
        T: Default,
    {
        let data = (0..(width * height)).map(|_| T::default()).collect();
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn new_with_value(width: usize, height: usize, value: T) -> Grid<T>
    where
        T: Clone,
    {
        let data = (0..(width * height)).map(|_| value.clone()).collect();
        Grid {
            width,
            height,
            data,
        }
    }

    pub fn index_of(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        Some(self.data.get(self.index_of(x, y)?)?)
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) -> Option<()> {
        let index = self.index_of(x, y)?;

        self.data[index] = value;

        Some(())
    }

    pub fn surrounding_coords_no_diagonals<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut coords = [None; 8];
        coords[0] = if x == 0 { None } else { Some((x - 1, y)) };
        coords[1] = if x >= self.width - 1 {
            None
        } else {
            Some((x + 1, y))
        };
        coords[2] = if y == 0 { None } else { Some((x, y - 1)) };
        coords[3] = if y >= self.height - 1 {
            None
        } else {
            Some((x, y + 1))
        };
        coords.into_iter().filter_map(|c| c)
    }

    pub fn surrounding_coords<'a>(
        &'a self,
        x: usize,
        y: usize,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        let mut coords = [None; 8];
        coords[0] = if x == 0 { None } else { Some((x - 1, y)) };
        coords[1] = if x >= self.width - 1 {
            None
        } else {
            Some((x + 1, y))
        };
        coords[2] = if y == 0 { None } else { Some((x, y - 1)) };
        coords[3] = if y >= self.height - 1 {
            None
        } else {
            Some((x, y + 1))
        };
        coords[4] = if x == 0 {
            None
        } else {
            if y == 0 {
                None
            } else {
                Some((x - 1, y - 1))
            }
        };
        coords[5] = if x == 0 {
            None
        } else {
            if y >= self.height - 1 {
                None
            } else {
                Some((x - 1, y + 1))
            }
        };
        coords[6] = if y == 0 {
            None
        } else {
            if x >= self.width - 1 {
                None
            } else {
                Some((x + 1, y - 1))
            }
        };
        coords[7] = if y >= self.height + 1 {
            None
        } else {
            if x >= self.width + 1 {
                None
            } else {
                Some((x + 1, y + 1))
            }
        };
        coords.into_iter().filter_map(|c| c)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn mutate(&mut self, x: usize, y: usize, f: impl Fn(&T) -> T) -> bool {
        if let Some(index) = self.index_of(x, y) {
            self.data[index] = f(&self.data[index]);
            return true;
        }
        false
    }

    pub fn mutate_all(&mut self, f: impl Fn(&T) -> T) {
        for x in 0..self.width {
            for y in 0..self.height {
                let index = self.index_of(x, y).unwrap();
                self.data[index] = f(&self.data[index]);
            }
        }
    }

    pub fn coords_where<'a>(
        &'a self,
        p: impl Fn(&T) -> bool + 'a,
    ) -> impl Iterator<Item = (usize, usize)> + 'a {
        self.all_coords()
            .filter(move |(x, y)| p(self.get(*x, *y).unwrap()))
    }

    pub fn all_coords<'a>(&'a self) -> impl Iterator<Item = (usize, usize)> + 'a {
        (0..self.width).flat_map(|x| (0..self.height).map(move |y| (x, y)))
    }

    pub fn all_values<'a>(&'a self) -> impl Iterator<Item = &T> + 'a {
        self.data.iter()
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(f, "{:?}", self.get(x, y).unwrap())?;
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}
