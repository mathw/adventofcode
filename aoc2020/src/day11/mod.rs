mod common;
mod part1;
mod part2;
mod seating;
mod traits;

use crate::dayerror::DayError;
use std::{io, str::FromStr, time::Duration};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

use self::{part1::*, part2::*, seating::*, traits::*};

pub fn part1<B: Backend>(terminal: &mut Terminal<B>, visualise: bool) -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let seating = SeatingWithPart1Rules(Seating::from_str(input)?);

    part_n(terminal, visualise, seating)
}

pub fn part2<B: Backend>(terminal: &mut Terminal<B>, visualise: bool) -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let seating = SeatingWithPart2Rules(Seating::from_str(input)?);

    part_n(terminal, visualise, seating)
}

fn part_n<B: Backend>(
    terminal: &mut Terminal<B>,
    visualise: bool,
    solutionfinder: impl SolutionFinder + SeatIterable,
) -> Result<String, DayError> {
    if visualise {
        terminal.clear()?;

        terminal.draw(|f| render(f, solutionfinder.seating(), 0))?;
    }

    let (fin, iterations) = if visualise {
        solutionfinder.find_solution_visually(
            &mut |s, i| {
                terminal
                    .draw(|f| render(f, s.seating(), i))
                    .expect("draw call failed")
            },
            Some(Duration::from_millis(100)),
        )
    } else {
        solutionfinder.find_solution()
    };

    tidy_cursor(terminal)?;

    Ok(format!(
        "Stable after {} iterations with {} seats filled",
        iterations,
        fin.seating().count_occupied_seats(),
    ))
}

fn tidy_cursor<B: Backend>(terminal: &mut Terminal<B>) -> Result<(), io::Error> {
    let size = terminal.size()?;
    terminal.set_cursor(0, size.height - 1)?;
    terminal.show_cursor()
}

fn render<B: Backend>(frame: &mut Frame<B>, seating: &Seating, iteration: usize) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Length(seating.width as u16),
                Constraint::Length(1),
                Constraint::Max(15),
                Constraint::Max(15),
            ]
            .as_ref(),
        )
        .split(frame.size());
    let iteration = Paragraph::new(vec![Spans::from(vec![Span::styled(
        format!("{}", iteration),
        Style::default().fg(Color::Green),
    )])])
    .block(Block::default().borders(Borders::NONE).title("Iteration"));
    frame.render_widget(iteration, chunks[2]);
    let count = Paragraph::new(Text::from(seating.count_occupied_seats().to_string())).block(
        Block::default()
            .borders(Borders::NONE)
            .title("Occupied Seats"),
    );
    frame.render_widget(count, chunks[3]);
    let mut seating_spans = vec![];
    for y in 0..seating.height {
        for x in 0..seating.width {
            match seating.seat_at(x, y) {
                Some(Seat::Empty) => {
                    seating_spans.push(Span::styled("L", Style::default().fg(Color::Green)))
                }
                Some(Seat::Full) => {
                    seating_spans.push(Span::styled("#", Style::default().fg(Color::Blue)))
                }
                Some(Seat::Floor) => {
                    seating_spans.push(Span::styled(".", Style::default().fg(Color::DarkGray)))
                }
                _ => seating_spans.push(Span::styled("!", Style::default().fg(Color::Red))),
            }
            seating_spans.push(Span::raw("\n"))
        }
    }
    let seating = Paragraph::new(Spans::from(seating_spans)).wrap(Wrap { trim: false });
    frame.render_widget(seating, chunks[0]);
}
