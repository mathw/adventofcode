use super::grid::{Coord, Grid, NodeState};

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn right(&self) -> Direction {
        match self {
            &Direction::North => Direction::East,
            &Direction::East => Direction::South,
            &Direction::South => Direction::West,
            &Direction::West => Direction::North,
        }
    }

    fn left(&self) -> Direction {
        match self {
            &Direction::North => Direction::West,
            &Direction::West => Direction::South,
            &Direction::South => Direction::East,
            &Direction::East => Direction::North,
        }
    }

    fn reverse(&self) -> Direction {
        match self {
            &Direction::North => Direction::South,
            &Direction::South => Direction::North,
            &Direction::East => Direction::West,
            &Direction::West => Direction::East,
        }
    }
}

pub struct Agent {
    grid: Grid,
    current_location: Coord,
    facing: Direction,
}

impl Agent {
    pub fn new(grid: Grid) -> Agent {
        if let Some(centre) = grid.find_centre() {
            Agent {
                grid: grid,
                current_location: centre,
                facing: Direction::North,
            }
        } else {
            Agent {
                grid: grid,
                current_location: Coord::new(0, 0),
                facing: Direction::North,
            }
        }
    }

    pub fn step(&mut self) -> bool {
        let mut did_infect = false;

        if self.grid.is_infected(self.current_location) {
            self.facing = self.facing.right();
            self.grid.clean(self.current_location);
        } else {
            self.facing = self.facing.left();
            self.grid.infect(self.current_location);
            did_infect = true;
        }

        self.move_forward();

        did_infect
    }

    fn move_forward(&mut self) {
        self.current_location = match self.facing {
            Direction::North => self.current_location.north(),
            Direction::East => self.current_location.east(),
            Direction::South => self.current_location.south(),
            Direction::West => self.current_location.west(),
        };
    }

    pub fn step_part_two(&mut self) -> bool {
        let mut did_infect = false;

        match self.grid.state_at(self.current_location) {
            NodeState::Clean => {
                self.facing = self.facing.left();
                self.grid.weaken(self.current_location);
            }
            NodeState::Weakened => {
                self.grid.infect(self.current_location);
                did_infect = true;
            }
            NodeState::Infected => {
                self.facing = self.facing.right();
                self.grid.flag(self.current_location);
            }
            NodeState::Flagged => {
                self.facing = self.facing.reverse();
                self.grid.clean(self.current_location);
            }
        };

        self.move_forward();

        did_infect
    }
}
