use crate::{
    day::Day,
    intcode::{Program, State},
};
use std::{
    collections::HashMap,
    io::{self, Read},
    str::FromStr,
    sync::mpsc::channel,
    thread,
    time::Duration,
};
use termion::{event::Key, input::TermRead, raw::IntoRawMode};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Paragraph, Text, Widget},
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

        let (score, stopped_by_user, blocks_left) =
            run_game_interactive(&program).map_err(|e| e.to_string())?;
        if stopped_by_user {
            Err(format!(
                "Stopped by user with score of {} and {} blocks left",
                score, blocks_left
            ))
        } else {
            if blocks_left > 0 {
                Err(format!(
                    "Stopped by game engine with {} blocks left and a score of {}",
                    blocks_left, score
                ))
            } else {
                Ok(format!(
                    "Game engine stopped with score of {} and {} blocks left",
                    score, blocks_left
                ))
            }
        }
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
    ai_intent: String,
) -> Result<(), io::Error> {
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .constraints(
                [
                    Constraint::Length(1),
                    Constraint::Length(30),
                    Constraint::Length(1),
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

        Paragraph::new([Text::styled(ai_intent, Style::default().fg(Color::Gray))].iter())
            .render(&mut f, chunks[2]);
        Paragraph::new(
            [Text::styled(
                "Hold q to exit",
                Style::default().fg(Color::LightGreen),
            )]
            .iter(),
        )
        .render(&mut f, chunks[3]);
    })
}

fn run_game_interactive(program: &Program<i64>) -> Result<(i64, bool, usize), io::Error> {
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
    let mut paddle_location = (-1, -1);
    let mut ball_location = (-1, -1);
    let mut stopped_by_user = false;
    let mut last_input = 0;

    let stdout = io::stdout().into_raw_mode()?;
    let mut stdin = termion::async_stdin();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal.clear()?;
    terminal.hide_cursor()?;

    loop {
        let mut buf = [0];
        if stdin.read(&mut buf).is_ok() {
            last_input = buf[0];
            if last_input == 113 || last_input == 3 {
                stopped_by_user = true;
                break;
            }
        }
        match state.state {
            State::Completed => break,
            State::NeedsInput => {
                let horizontal_gap = ball_location.0 - paddle_location.0;
                let direction = if horizontal_gap < 0 {
                    -1
                } else if horizontal_gap == 0 {
                    0
                } else {
                    1
                };
                let ai_intent = format!(
                    "Ball @ {:?}  Paddle @ {:?}  X-gap {}  Direction {}",
                    ball_location, paddle_location, horizontal_gap, direction
                );
                draw_screen(
                    &mut terminal,
                    &screen,
                    score,
                    score_ever_provided,
                    ai_intent,
                )?;
                thread::sleep(Duration::from_millis(5));
                state = state.resume_with_input(direction);
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
                        if o == 3 {
                            paddle_location = (x, y);
                        }
                        if o == 4 {
                            ball_location = (x, y);
                        }
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

    Ok((
        score,
        stopped_by_user,
        screen.values().filter(|&t| *t == 2).count(),
    ))
}
