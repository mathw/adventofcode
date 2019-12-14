use crate::day::Day;
use crate::intcode::{Program, State};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

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
        let mut program = self.program.clone();
        program[0] = 2;

        let score = run_game_interactive(&program).map_err(|e| e.to_string())?;
        Ok(format!("Final score is {}", score))
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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum JoystickPosition {
    Neutral,
    Left,
    Right,
}

impl From<JoystickPosition> for i64 {
    fn from(p: JoystickPosition) -> Self {
        match p {
            JoystickPosition::Neutral => 0,
            JoystickPosition::Left => -1,
            JoystickPosition::Right => 1,
        }
    }
}

fn render_screen(screen: &Screen) -> String {
    let max_x = *screen.keys().map(|(x, _)| x).max().unwrap_or(&0);
    let max_y = *screen.keys().map(|(_, y)| y).max().unwrap_or(&0);

    let mut lines: Vec<String> = Vec::new();

    for y in 0..=max_y {
        let mut line = Vec::new();
        for x in 0..=max_x {
            line.push(match screen.get(&(x, y)) {
                None | Some(0) => ' ',
                Some(1) => '█',
                Some(2) => '▒',
                Some(3) => '▁',
                Some(4) => '●',
                _ => '?',
            });
        }
        lines.push(line.iter().collect());
    }

    lines.join("\n")
}

fn draw_screen<B: Backend>(
    terminal: &mut Terminal<B>,
    screen: &Screen,
    score: i64,
    score_provided: bool,
) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Min(30),
                    Constraint::Length(1),
                ]
                .as_ref(),
            )
            .direction(Direction::Vertical)
            .split(f.size());

        Paragraph::new(
            [Text::styled(
                format!("Score: {}", score),
                Style::default().fg(if score_provided {
                    Color::LightGreen
                } else {
                    Color::LightRed
                }),
            )]
            .iter(),
        )
        .render(&mut f, chunks[0]);

        let game_board = render_screen(screen);
        let text = [Text::raw(game_board)];
        Paragraph::new(text.iter()).render(&mut f, chunks[1]);

        Paragraph::new(
            [Text::styled(
                "⬅️, ➡️ or space",
                Style::default().fg(Color::Gray),
            )]
            .iter(),
        )
        .render(&mut f, chunks[2]);
    })
}

fn run_game_interactive(program: &Program<i64>) -> Result<i64, io::Error> {
    let mut screen = HashMap::new();

    enum OutputState {
        WantsX,
        WantsY,
        WantsId,
        WantsScore,
    }

    let mut outputstate = OutputState::WantsX;

    let mut state = program.run_until_needs_interaction();

    let mut x = 0;
    let mut y = 0;
    let mut score = 0;
    let mut score_ever_provided = false;

    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let mut keys = stdin.keys();

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        match state.state {
            State::Completed => break,
            State::NeedsInput => {
                draw_screen(&mut terminal, &screen, score, score_ever_provided)?;
                loop {
                    match keys.next() {
                        Some(Ok(Key::Left)) => {
                            state = state.resume_with_input(JoystickPosition::Left.into());
                            break;
                        }
                        Some(Ok(Key::Right)) => {
                            state = state.resume_with_input(JoystickPosition::Right.into());
                            break;
                        }
                        Some(Ok(Key::Char(' '))) => {
                            state = state.resume_with_input(JoystickPosition::Neutral.into());
                            break;
                        }
                        _ => {}
                    }
                }
            }
            State::ProvidedOutput(o) => {
                match outputstate {
                    OutputState::WantsX => {
                        x = o;
                        outputstate = OutputState::WantsY;
                    }
                    OutputState::WantsY => {
                        y = o;
                        outputstate = if x == -1 && y == 0 {
                            OutputState::WantsScore
                        } else {
                            OutputState::WantsId
                        };
                    }
                    OutputState::WantsId => {
                        draw_tile(&mut screen, x, y, o);
                        outputstate = OutputState::WantsX;
                    }
                    OutputState::WantsScore => {
                        score = o;
                        score_ever_provided = true;
                        outputstate = OutputState::WantsX;
                    }
                };
                state = state.resume();
            }
        }
    }

    terminal.clear()?;
    terminal.show_cursor()?;

    Ok(score)
}
