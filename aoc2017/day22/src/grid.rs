use std::collections::HashMap;
use std::str::FromStr;

/// A coordinate
/// We consider north to be -y, south to be +y, east to be +x, west to be -x
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    pub fn new(x: isize, y: isize) -> Coord {
        Coord { x: x, y: y }
    }

    pub fn north(&self) -> Coord {
        Coord::new(self.x, self.y - 1)
    }

    pub fn south(&self) -> Coord {
        Coord::new(self.x, self.y + 1)
    }

    pub fn east(&self) -> Coord {
        Coord::new(self.x + 1, self.y)
    }

    pub fn west(&self) -> Coord {
        Coord::new(self.x - 1, self.y)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum NodeState {
    Clean,
    Weakened,
    Infected,
    Flagged,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    squares: HashMap<Coord, NodeState>,
}

impl Grid {
    pub fn is_infected(&self, what: Coord) -> bool {
        if self.squares.contains_key(&what) {
            match self.squares[&what] {
                NodeState::Clean => false,
                NodeState::Infected => true,
                NodeState::Weakened => false,
                NodeState::Flagged => true,
            }
        } else {
            false
        }
    }

    pub fn find_centre(&self) -> Option<Coord> {
        let max_x = self.squares.keys().map(|c| c.x).max();
        let max_y = self.squares.keys().map(|c| c.y).max();

        match (max_x, max_y) {
            (Some(x), Some(y)) => Some(Coord::new(x / 2, y / 2)),
            _ => None,
        }
    }

    pub fn infect(&mut self, what: Coord) {
        self.set_at(what, NodeState::Infected);
    }

    pub fn clean(&mut self, what: Coord) {
        self.set_at(what, NodeState::Clean);
    }

    pub fn weaken(&mut self, what: Coord) {
        self.set_at(what, NodeState::Weakened);
    }

    pub fn flag(&mut self, what: Coord) {
        self.set_at(what, NodeState::Flagged);
    }

    fn set_at(&mut self, what: Coord, to: NodeState) {
        let entry = self.squares.entry(what).or_insert(to);
        *entry = to;
    }

    pub fn state_at(&self, what: Coord) -> NodeState {
        if self.squares.contains_key(&what) {
            self.squares[&what]
        } else {
            NodeState::Clean
        }
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                match cell {
                    '#' => map.insert(Coord::new(x as isize, y as isize), NodeState::Infected),
                    '.' => map.insert(Coord::new(x as isize, y as isize), NodeState::Clean),
                    _ => return Err(()),
                };
            }
        }
        Ok(Grid { squares: map })
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let input = "#.
.#";
        let grid = Grid::from_str(input);

        let mut expected_map = HashMap::new();
        expected_map.insert(Coord::new(0, 0), NodeState::Infected);
        expected_map.insert(Coord::new(1, 0), NodeState::Clean);
        expected_map.insert(Coord::new(0, 1), NodeState::Clean);
        expected_map.insert(Coord::new(1, 1), NodeState::Infected);

        assert_eq!(
            grid,
            Ok(Grid {
                squares: expected_map,
            })
        );
    }

    #[test]
    fn test_grid_is_infected() {
        let input = "#.
.#";
        let grid = Grid::from_str(input).unwrap();

        assert_eq!(grid.is_infected(Coord::new(0, 0)), true);
        assert_eq!(grid.is_infected(Coord::new(1, 0)), false);
        assert_eq!(grid.is_infected(Coord::new(0, 1)), false);
        assert_eq!(grid.is_infected(Coord::new(1, 1)), true);
    }
}
