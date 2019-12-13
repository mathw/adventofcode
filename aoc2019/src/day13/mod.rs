use crate::day::Day;
use crate::intcode::{Program, State};
use std::collections::HashMap;
use std::str::FromStr;

type Screen = HashMap<(i64, i64), u8>;

pub struct Day13 {
    program: Program<i64>,
}

impl Day13 {
    pub fn new() -> Result<Day13, String> {
        Ok(Day13 {
            program: Program::<i64>::from_str(include_str!("input.intcode"))
                .map_err(|e| e.to_string())?,
        })
    }
}

impl Day for Day13 {
    fn part1(&mut self) -> Result<String, String> {
        let screen = run_game(&self.program);
        let block_tiles = screen.values().filter(|t| **t == 2).count();
        Ok(format!(
            "There are {} block tiles on the screen",
            block_tiles
        ))
    }
    fn part2(&mut self) -> Result<String, String> {
        Err("not implemented".into())
    }
}

fn run_game(program: &Program<i64>) -> Screen {
    let mut screen = HashMap::new();

    enum OutputState {
        WantsX,
        WantsY,
        WantsId,
    }

    let mut outputstate = OutputState::WantsX;

    let mut state = program.run_until_needs_interaction();

    let mut x = 0;
    let mut y = 0;

    loop {
        match state.state {
            State::Completed => break,
            State::NeedsInput => panic!("Did not expect to have to provide input"),
            State::ProvidedOutput(o) => {
                match outputstate {
                    OutputState::WantsX => {
                        x = o;
                        outputstate = OutputState::WantsY;
                    }
                    OutputState::WantsY => {
                        y = o;
                        outputstate = OutputState::WantsId;
                    }
                    OutputState::WantsId => {
                        draw_tile(&mut screen, x, y, o);
                        outputstate = OutputState::WantsX;
                    }
                };
                state = state.resume();
            }
        }
    }

    screen
}

fn draw_tile(screen: &mut Screen, x: i64, y: i64, id: i64) {
    let id = id as u8;
    if id == 0 {
        // empty tile
        screen.remove(&(x, y));
    } else {
        *(screen.entry((x, y)).or_default()) = id;
    }
}
