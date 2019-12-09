use std::fmt::Display;

/// An abstraction over a contiguous array of memory which can auto-grow when necessary
pub struct Memory<N>(Vec<N>);

impl<N> Memory<N>
where
    N: From<i32> + Copy + Display,
{
    pub fn get(&self, index: usize) -> N {
        if self.0.len() < index + 1 {
            0i32.into()
        } else {
            self.0[index]
        }
    }

    pub fn set(&mut self, index: usize, value: N) {
        if self.0.len() < index + 1 {
            self.0.resize(index + 1, 0i32.into());
        }
        self.0[index] = value;
    }

    pub fn as_vector(self) -> Vec<N> {
        self.0
    }
}

impl<N> From<Vec<N>> for Memory<N> {
    fn from(v: Vec<N>) -> Memory<N> {
        Memory(v)
    }
}
