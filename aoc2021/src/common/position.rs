#[derive(Copy, Clone, Debug)]
pub struct Position {
    horizontal: i64,
    depth: i64,
}

impl Default for Position {
    fn default() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }
}

impl Position {
    pub fn x(&self) -> i64 {
        self.horizontal
    }
    pub fn y(&self) -> i64 {
        self.depth
    }
    pub fn down(self, distance: u32) -> Self {
        Position {
            depth: self.depth + distance as i64,
            ..self
        }
    }
    pub fn up(self, distance: u32) -> Self {
        Position {
            depth: self.depth - distance as i64,
            ..self
        }
    }
    pub fn forward(self, distance: u32) -> Self {
        Position {
            horizontal: self.horizontal + distance as i64,
            ..self
        }
    }
    pub fn forward_with_aim(self, aim: i64, distance: u32) -> Self {
        Position {
            horizontal: self.horizontal + distance as i64,
            depth: self.depth + (distance as i64 * aim),
        }
    }
}
