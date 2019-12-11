use crate::day::Day;
use crate::intcode::{Program, State};
use std::collections::HashMap;
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
        let mut current = (0, 0);
        let mut facing = Facing::Up;

        let mut state = self.program.run_until_needs_interaction();
        let mut colour_to_paint = None;

        loop {
            match state.state {
                State::NeedsInput => {
                    state = state.resume_with_input(match grid.colour_at(current) {
                        Colour::Black => 0,
                        Colour::White => 1,
                    });
                }
                State::ProvidedOutput(o) => {
                    if colour_to_paint.is_none() {
                        colour_to_paint = Some(o);
                        grid.set_colour_at(
                            current,
                            match o {
                                0 => Colour::Black,
                                1 => Colour::White,
                                _ => return Err(format!("Unknown colour {}", o)),
                            },
                        );
                    } else {
                        let direction = o;
                        let turn = match direction {
                            0 => Grid::turn_left_from(current, facing),
                            1 => Grid::turn_right_from(current, facing),
                            _ => return Err(format!("Unknown direction {}", direction)),
                        };
                        current = turn.0;
                        facing = turn.1;
                        colour_to_paint = None;
                    }
                }
                State::Completed => {
                    break;
                }
            }
        }

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
