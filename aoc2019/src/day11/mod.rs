use crate::day::Day;
use crate::intcode::{Program, State};
use std::collections::HashMap;
use std::fmt::Display;
use std::str::FromStr;

pub struct Day11 {
    program: Program<i64>,
}

impl Day11 {
    pub fn new() -> Result<Day11, String> {
        Ok(Day11 {
            program: Program::<i64>::from_str(include_str!("input.txt"))
                .map_err(|e| e.to_string())?,
        })
    }
}

impl Day for Day11 {
    fn part1(&mut self) -> Result<String, String> {
        let mut grid = Grid::new();
        let mut robot = HullPaintingRobot::new(self.program.clone());

        robot.paint_hull(&mut grid);

        let painted_locations = grid.painted.len();
        Ok(format!("Painted {} different locations", painted_locations))
    }

    fn part2(&mut self) -> Result<String, String> {
        Err("Not implemented".into())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Colour {
    Black,
    White,
}
impl Default for Colour {
    fn default() -> Self {
        Colour::Black
    }
}
impl Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Colour::Black => write!(f, "black"),
            Colour::White => write!(f, "white"),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Facing {
    Up,
    Down,
    Left,
    Right,
}

impl Facing {
    fn left(&self) -> Facing {
        match self {
            Facing::Up => Facing::Left,
            Facing::Left => Facing::Down,
            Facing::Down => Facing::Right,
            Facing::Right => Facing::Up,
        }
    }
    fn right(&self) -> Facing {
        match self {
            Facing::Up => Facing::Right,
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
        }
    }
}

struct Grid {
    painted: HashMap<(i32, i32), Colour>,
}

impl Grid {
    fn new() -> Grid {
        Grid {
            painted: HashMap::new(),
        }
    }

    fn colour_at(&self, pos: (i32, i32)) -> Colour {
        self.painted.get(&pos).map(|x| *x).unwrap_or_default()
    }

    fn set_colour_at(&mut self, pos: (i32, i32), colour: Colour) {
        *(self.painted.entry(pos).or_default()) = colour
    }

    fn turn_left_from((x, y): (i32, i32), facing: Facing) -> ((i32, i32), Facing) {
        match facing {
            Facing::Up => ((x - 1, y), facing.left()),
            Facing::Left => ((x, y + 1), facing.left()),
            Facing::Down => ((x + 1, y), facing.left()),
            Facing::Right => ((x, y - 1), facing.left()),
        }
    }

    fn turn_right_from((x, y): (i32, i32), facing: Facing) -> ((i32, i32), Facing) {
        match facing {
            Facing::Up => ((x + 1, y), facing.right()),
            Facing::Left => ((x, y - 1), facing.right()),
            Facing::Down => ((x - 1, y), facing.right()),
            Facing::Right => ((x, y + 1), facing.right()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum OutputState {
    WantPaint,
    WantTurn,
}

struct HullPaintingRobot {
    program: Program<i64>,
}

impl HullPaintingRobot {
    fn new(program: Program<i64>) -> HullPaintingRobot {
        HullPaintingRobot { program }
    }

    fn paint_hull(&mut self, hull: &mut Grid) {
        let mut x = 0;
        let mut y = 0;
        let mut facing = Facing::Up;

        let mut state = self.program.run_until_needs_interaction();
        let mut output_mode = OutputState::WantPaint;
        loop {
            match state.state {
                State::Completed => break,
                State::ProvidedOutput(o) => match output_mode {
                    OutputState::WantPaint => {
                        let colour = match o {
                            0 => Colour::Black,
                            1 => Colour::White,
                            _ => panic!("unknown output colour {}", o),
                        };
                        hull.set_colour_at((x, y), colour);
                        output_mode = OutputState::WantTurn;
                        state = state.resume();
                    }
                    OutputState::WantTurn => {
                        let ((nx, ny), nf) = match o {
                            0 => Grid::turn_left_from((x, y), facing),
                            1 => Grid::turn_right_from((x, y), facing),
                            _ => panic!("Unknown turn instruction {}", o),
                        };
                        x = nx;
                        y = ny;
                        facing = nf;
                        output_mode = OutputState::WantPaint;
                        state = state.resume();
                    }
                },
                State::NeedsInput => {
                    let colour = hull.colour_at((x, y));
                    state = state.resume_with_input(match colour {
                        Colour::Black => 0,
                        Colour::White => 1,
                    });
                }
            }
        }
    }
}

#[test]
fn test_turn_left() {
    let position = (0, 0);
    let (new_position, facing) = Grid::turn_left_from(position, Facing::Up);

    assert_eq!(facing, Facing::Left);
    assert_eq!(new_position, (-1, 0));

    let (new_position, facing) = Grid::turn_left_from(new_position, facing);
    assert_eq!(facing, Facing::Down);
    assert_eq!(new_position, (-1, 1));
}

#[test]
fn test_turn_right() {
    let position = (0, 0);
    let (new_position, facing) = Grid::turn_right_from(position, Facing::Up);
    assert_eq!(facing, Facing::Right);
    assert_eq!(new_position, (1, 0));
}

#[cfg(test)]
mod propertytests {
    use super::*;
    use quickcheck::{Arbitrary, Gen};
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Colour {
        fn arbitrary<G: Gen>(g: &mut G) -> Colour {
            match bool::arbitrary(g) {
                true => Colour::White,
                false => Colour::Black,
            }
        }
    }

    #[quickcheck]
    fn location_painted_is_retrieved_as_that_colour(x: i32, y: i32, colour: Colour) -> bool {
        let mut grid = Grid::new();
        grid.set_colour_at((x, y), colour);
        let grid_colour = grid.colour_at((x, y));
        colour == grid_colour
    }
}
