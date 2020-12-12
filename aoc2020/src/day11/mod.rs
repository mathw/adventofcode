use crate::dayerror::DayError;
use std::{
    fmt::{self, Display},
    str::FromStr,
    thread::sleep,
    time::Duration,
};
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame, Terminal,
};

pub fn part1<B: Backend>(terminal: &mut Terminal<B>, visualise: bool) -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let seating = Seating::from_str(input)?;

    if visualise {
        terminal.clear()?;

        terminal.draw(|f| render(f, &seating, 0))?;
    }

    let (fin, iterations) = if visualise {
        seating.iterate_until_stable(
            &mut |s, i| {
                terminal
                    .draw(|f| render(f, s, i))
                    .expect("draw call failed")
            },
            Some(Duration::from_millis(100)),
        )
    } else {
        seating.iterate_until_stable(&mut |_, _| {}, None)
    };

    let size = terminal.size()?;
    terminal.set_cursor(0, size.height - 1)?;
    terminal.show_cursor()?;

    Ok(format!(
        "Stable after {} iterations with {} seats filled",
        iterations,
        fin.count_occupied_seats(),
    ))
}

pub fn part2<B: Backend>(terminal: &mut Terminal<B>, visualise: bool) -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let seating = Seating::from_str(input)?;

    if visualise {
        terminal.clear()?;

        terminal.draw(|f| render(f, &seating, 0))?;
    }

    let (fin, iterations) = if visualise {
        seating.iterate_until_stable_2(
            &mut |s, i| {
                terminal
                    .draw(|f| render(f, s, i))
                    .expect("draw call failed")
            },
            Some(Duration::from_millis(100)),
        )
    } else {
        seating.iterate_until_stable_2(&mut |_, _| {}, None)
    };

    let size = terminal.size()?;
    terminal.set_cursor(0, size.height - 1)?;
    terminal.show_cursor()?;

    Ok(format!(
        "Stable after {} iterations with {} seats filled",
        iterations,
        fin.count_occupied_seats(),
    ))
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Empty,
    Full,
    Floor,
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Seating {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl Seating {
    fn new(width: usize, height: usize, seats: Vec<Seat>) -> Seating {
        Seating {
            width,
            height,
            seats,
        }
    }

    fn seat_at(&self, x: usize, y: usize) -> Option<Seat> {
        self.to_index(x, y).map(|i| self.seats[i])
    }

    fn to_index(&self, x: usize, y: usize) -> Option<usize> {
        let index = self.to_index_unchecked(x, y);
        if index >= self.seats.len() {
            None
        } else {
            Some(index)
        }
    }

    fn to_index_unchecked(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn iterate(&self) -> Seating {
        let mut new = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.to_index_unchecked(x, y);
                new.seats[index] = self.iterate_seat(x, y, index);
            }
        }
        new
    }

    fn iterate_2(&self) -> Seating {
        let mut new = self.clone();

        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.to_index_unchecked(x, y);
                new.seats[index] = self.iterate_seat_2(x, y, index);
            }
        }
        new
    }

    fn iterate_until_stable<F>(&self, drawfunc: &mut F, delay: Option<Duration>) -> (Seating, usize)
    where
        F: (FnMut(&Seating, usize) -> ()),
    {
        self.iterate_until_stable_with(drawfunc, delay, |s| s.iterate())
    }

    fn iterate_until_stable_2<F>(
        &self,
        drawfunc: &mut F,
        delay: Option<Duration>,
    ) -> (Seating, usize)
    where
        F: (FnMut(&Seating, usize) -> ()),
    {
        self.iterate_until_stable_with(drawfunc, delay, |s| s.iterate_2())
    }

    fn iterate_until_stable_with<F, I>(
        &self,
        drawfunc: &mut F,
        delay: Option<Duration>,
        iterfunc: I,
    ) -> (Seating, usize)
    where
        F: (FnMut(&Seating, usize) -> ()),
        I: Fn(&Seating) -> Seating,
    {
        let mut previous = self.clone();
        let mut next = self.iterate();
        let mut iterations = 1;

        while previous != next {
            drawfunc(&next, iterations);
            previous = next;
            next = iterfunc(&previous);
            iterations += 1;
            if let Some(d) = delay {
                sleep(d);
            }
        }

        (next, iterations)
    }

    fn iterate_seat(&self, x: usize, y: usize, index: usize) -> Seat {
        let seat = self.seats[index];
        match seat {
            Seat::Empty if self.count_adjacent_occupied(x, y) == 0 => Seat::Full,
            Seat::Full if self.count_adjacent_occupied(x, y) >= 4 => Seat::Empty,
            x => x,
        }
    }

    fn iterate_seat_2(&self, x: usize, y: usize, index: usize) -> Seat {
        let seat = self.seats[index];
        if seat == Seat::Floor {
            return Seat::Floor;
        }

        let visible = self.visible_occupied_seats_from(x, y);
        match seat {
            Seat::Empty if visible == 0 => Seat::Full,
            Seat::Full if visible >= 5 => Seat::Empty,
            x => x,
        }
    }

    fn count_adjacent_occupied(&self, x: usize, y: usize) -> usize {
        let xsub = x.checked_sub(1);
        let ysub = y.checked_sub(1);
        let xplus = if x + 1 < self.width {
            Some(x + 1)
        } else {
            None
        };
        let yplus = if y + 1 < self.height {
            Some(y + 1)
        } else {
            None
        };

        let locations = match (xsub, ysub, xplus, yplus) {
            (None, None, None, None) => vec![],
            (Some(xs), None, None, None) => vec![(xs, y)],
            (None, Some(ys), None, None) => vec![(x, ys)],
            (None, None, Some(xp), None) => vec![(xp, y)],
            (None, None, None, Some(yp)) => vec![(x, yp)],
            (Some(xs), Some(ys), None, None) => vec![(xs, ys), (xs, y), (x, ys)],
            (None, Some(ys), Some(xp), None) => vec![(x, ys), (xp, ys), (xp, y)],
            (None, None, Some(xp), Some(yp)) => vec![(xp, y), (xp, yp), (x, yp)],
            (None, Some(ys), None, Some(yp)) => vec![(x, ys), (x, yp)],
            (Some(xs), None, None, Some(yp)) => vec![(x, yp), (xs, yp), (xs, y)],
            (Some(xs), None, Some(xp), None) => vec![(xs, y), (xp, y)],
            (Some(xs), Some(ys), Some(xp), None) => {
                vec![(xp, y), (xp, ys), (xs, ys), (xs, y), (x, ys)]
            }
            (None, Some(ys), Some(xp), Some(yp)) => {
                vec![(x, ys), (xp, ys), (xp, y), (x, yp), (xp, yp)]
            }
            (Some(xs), None, Some(xp), Some(yp)) => {
                vec![(x, yp), (xs, yp), (xp, yp), (xs, y), (xp, y)]
            }
            (Some(xs), Some(ys), None, Some(yp)) => {
                vec![(xs, y), (xs, ys), (xs, yp), (x, ys), (x, yp)]
            }
            (Some(xs), Some(ys), Some(xp), Some(yp)) => vec![
                (xs, ys),
                (xs, y),
                (xs, yp),
                (x, ys),
                (x, yp),
                (xp, ys),
                (xp, y),
                (xp, yp),
            ],
        };

        locations
            .iter()
            .filter_map(|&(x, y)| self.seats.get(self.to_index(x, y)?))
            .filter(|s| **s == Seat::Full)
            .count()
    }

    fn count_occupied_seats(&self) -> usize {
        self.seats.iter().filter(|s| **s == Seat::Full).count()
    }

    fn can_see_occupied_north_of(&self, x: usize, y: usize) -> bool {
        self.seats
            .chunks_exact(self.width)
            .take(y)
            .map(|chunk| chunk[x])
            .rev()
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    fn can_see_occupied_south_of(&self, x: usize, y: usize) -> bool {
        self.seats
            .chunks_exact(self.width)
            .skip(y + 1)
            .map(|chunk| chunk[x])
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    fn can_see_occupied_east_of(&self, x: usize, y: usize) -> bool {
        if x + 1 == self.width {
            return false;
        }
        let start_index = self.to_index_unchecked(x + 1, y);
        let end_index = self.to_index_unchecked(self.width - 1, y);
        self.seats[start_index..=end_index]
            .iter()
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    fn can_see_occupied_west_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.to_index_unchecked(0, y);
        let end_index = self.to_index_unchecked(x, y);
        (start_index..end_index)
            .rev()
            .map(|i| self.seats[i])
            .filter_map(|s| match s {
                Seat::Full => Some(true),
                Seat::Empty => Some(false),
                Seat::Floor => None,
            })
            .next()
            .unwrap_or(false)
    }

    fn can_see_occupied_southeast_of(&self, x: usize, y: usize) -> bool {
        if x + 1 == self.width {
            return false;
        }
        let start_index = self.to_index_unchecked(x, y);
        let mut current_index = start_index + self.width + 1;
        while current_index < self.seats.len() {
            if current_index % self.width == 0 {
                // we've wrapped into column 0! won't find anything now
                return false;
            }
            match self.seats[current_index] {
                Seat::Full => return true,
                Seat::Empty => return false,
                Seat::Floor => {}
            }
            current_index += self.width + 1;
        }
        return false;
    }

    fn can_see_occupied_southwest_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            current_index += self.width - 1;

            if current_index >= self.seats.len() {
                // off the bottom
                return false;
            }
            match self.seats[current_index] {
                Seat::Full => return true,
                Seat::Empty => return false,
                Seat::Floor => {}
            }
            if current_index % self.width == 0 {
                // start of line, not going to find anything now
                return false;
            }
        }
    }

    fn can_see_occupied_northeast_of(&self, x: usize, y: usize) -> bool {
        let start_index = self.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            match current_index.checked_sub(self.width - 1) {
                Some(i) => {
                    if i % self.width == 0 {
                        // ran over into next line!
                        return false;
                    }

                    current_index = i;
                    match self.seats[current_index] {
                        Seat::Full => return true,
                        Seat::Empty => return false,
                        Seat::Floor => {}
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    fn can_see_occupied_northwest_of(&self, x: usize, y: usize) -> bool {
        if x == 0 {
            return false;
        }
        let start_index = self.to_index_unchecked(x, y);
        let mut current_index = start_index;
        loop {
            match current_index.checked_sub(self.width + 1) {
                Some(i) => {
                    current_index = i;
                    match self.seats[current_index] {
                        Seat::Full => return true,
                        Seat::Empty => return false,
                        Seat::Floor => {}
                    };
                    if i % self.width == 0 {
                        // start of line, not going to find anything now
                        return false;
                    }
                }
                None => {
                    return false;
                }
            }
        }
    }

    fn visible_occupied_seats_from(&self, x: usize, y: usize) -> usize {
        let north = self.can_see_occupied_north_of(x, y);
        let south = self.can_see_occupied_south_of(x, y);
        let east = self.can_see_occupied_east_of(x, y);
        let west = self.can_see_occupied_west_of(x, y);
        let southeast = self.can_see_occupied_southeast_of(x, y);
        let southwest = self.can_see_occupied_southwest_of(x, y);
        let northeast = self.can_see_occupied_northeast_of(x, y);
        let northwest = self.can_see_occupied_northwest_of(x, y);

        [
            north, south, east, west, southeast, southwest, northeast, northwest,
        ]
        .iter()
        .filter(|x| **x)
        .count()
    }

    #[cfg(test)]
    fn count_empty_seats(&self) -> usize {
        self.seats.iter().filter(|s| **s == Seat::Empty).count()
    }
}

impl FromStr for Seating {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = 0;
        let mut height = 0;
        let mut seats = Vec::new();

        for line in s.lines().filter(|l| l.len() > 0) {
            width = line.len();
            height += 1;

            for c in line.chars() {
                match c {
                    'L' => seats.push(Seat::Empty),
                    '.' => seats.push(Seat::Floor),
                    '#' => seats.push(Seat::Full),
                    _ => {
                        return Err(DayError::InputParseError(format!(
                            "Unrecognised seat character '{}'",
                            c
                        )))
                    }
                };
            }
        }

        Ok(Seating::new(width, height, seats))
    }
}

impl Display for Seating {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        for y in 0..self.height {
            for x in 0..self.width {
                write!(
                    f,
                    "{}",
                    match self.seat_at(x, y) {
                        Some(Seat::Full) => '#',
                        Some(Seat::Empty) => 'L',
                        Some(Seat::Floor) => '.',
                        None => '!',
                    }
                )?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_parse() {
    let result = Seating::from_str(
        "LL
..
L.
L#
",
    );
    let seating = result.expect("Should parse");
    assert_eq!(seating.width, 2);
    assert_eq!(seating.height, 4);
    assert_eq!(
        seating.seats,
        vec![
            Seat::Empty,
            Seat::Empty,
            Seat::Floor,
            Seat::Floor,
            Seat::Empty,
            Seat::Floor,
            Seat::Empty,
            Seat::Full
        ]
    );
}

#[test]
fn test_surrounds() {
    let seating = Seating::from_str(
        "L.LL.LL.LL
LLLL#LL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    )
    .unwrap();

    assert_eq!(seating.count_adjacent_occupied(0, 0), 0);
    assert_eq!(seating.count_adjacent_occupied(3, 1), 1);
    assert_eq!(seating.count_adjacent_occupied(0, 1), 0);
}

#[test]
fn test_iterate() {
    let seating = Seating::from_str(
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    )
    .unwrap();

    let seating = seating.iterate();
    let target = Seating::from_str(
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##",
    )
    .unwrap();
    assert_eq!(seating.seats, target.seats);

    let seating = seating.iterate();
    let target = Seating::from_str(
        "#.LL.L#.##
#LLLLLL.L#
L.L.L..L..
#LLL.LL.L#
#.LL.LL.LL
#.LLLL#.##
..L.L.....
#LLLLLLLL#
#.LLLLLL.L
#.#LLLL.##",
    )
    .unwrap();

    assert_eq!(
        seating.seats, target.seats,
        "Desired:\n{}, Obtained:\n{}",
        target, seating
    );

    let seating = seating.iterate();
    let target = Seating::from_str(
        "#.##.L#.##
#L###LL.L#
L.#.#..#..
#L##.##.L#
#.##.LL.LL
#.###L#.##
..#.#.....
#L######L#
#.LL###L.L
#.#L###.##",
    )
    .unwrap();

    assert_eq!(seating.seats, target.seats);

    let seating = seating.iterate();
    let target = Seating::from_str(
        "#.#L.L#.##
#LLL#LL.L#
L.L.L..#..
#LLL.##.L#
#.LL.LL.LL
#.LL#L#.##
..L.L.....
#L#LLLL#L#
#.LLLLLL.L
#.#L#L#.##",
    )
    .unwrap();

    assert_eq!(seating.seats, target.seats);
    let seating = seating.iterate();
    let target = Seating::from_str(
        "#.#L.L#.##
#LLL#LL.L#
L.#.L..#..
#L##.##.L#
#.#L.LL.LL
#.#L#L#.##
..L.L.....
#L#L##L#L#
#.LLLLLL.L
#.#L#L#.##",
    )
    .unwrap();

    assert_eq!(seating.seats, target.seats);
}

#[test]
fn test_iterate_until_stable() {
    let seating = Seating::from_str(
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    )
    .unwrap();
    let (fin, iterations) = seating.iterate_until_stable(&mut |_, _| {}, None);
    assert_eq!(fin.count_occupied_seats(), 37);
    assert_eq!(iterations, 6);
}

#[test]
fn test_many_surround_counts() {
    let scenarios = vec![1, 0, 0, 1, 0, 1, 0, 1, 0]
        .into_iter()
        .permutations(9)
        .map(|seats| (seats.clone(), if seats[4] == 1 { 3 } else { 4 }, (1, 1)));

    fn scenario_to_seating(template: &Vec<u8>) -> Seating {
        Seating::new(
            3,
            3,
            template
                .iter()
                .map(|x| match x {
                    0 => Seat::Empty,
                    1 => Seat::Full,
                    _ => Seat::Floor,
                })
                .collect(),
        )
    }

    for (scenario, expected_count, (x, y)) in scenarios {
        let seating = scenario_to_seating(&scenario);
        assert_eq!(
            seating.count_adjacent_occupied(x, y),
            expected_count,
            "Failed \n{} expecting {} around {},{}",
            seating,
            expected_count,
            x,
            y
        );
    }
}

#[test]
fn test_real_input_first_line() {
    let line = "LLLLLLLLLLLLLLL.L..LLLLLL.LLLLLLLLL.LLLLLL.LLLLLLL.LLLLLLL.LLLLLL.LLLLLLLLL.LLLL...LLLLLLLLLLL\n";
    let seating = Seating::from_str(line).unwrap();
    assert_eq!(seating.width, 94);
    assert_eq!(seating.height, 1);
    assert_eq!(seating.seats.len(), 94);
    assert_eq!(
        seating.seats,
        line.trim()
            .chars()
            .map(|c| match c {
                'L' => Seat::Empty,
                '.' => Seat::Floor,
                _ => Seat::Full,
            })
            .collect::<Vec<Seat>>()
    );
    assert_eq!(seating.to_string(), line);
}

#[test]
fn test_hayward_input() {
    let input = include_str!("hayward_input.txt");
    let seating = Seating::from_str(input).unwrap();
    let (fin, _) = seating.iterate_until_stable(&mut |_, _| {}, None);
    assert_eq!(fin.count_occupied_seats(), 2354);
}

#[test]
fn test_iterate_hayward_1() {
    let input = include_str!("hayward_input.txt");
    let seating = Seating::from_str(input).unwrap();
    let new = seating.iterate();
    assert_eq!(
        seating.count_empty_seats(),
        new.count_occupied_seats(),
        "First iteration: all empty seats should always become occupied"
    );
}

#[test]
fn test_index() {
    let seating = Seating {
        height: 97,
        width: 99,
        seats: Vec::new(),
    };
    let index = seating.to_index_unchecked(0, 1);
    assert_eq!(index, 99);
}

#[test]
fn test_visible_occupied_seats_from() {
    let seat = Seating::from_str(
        ".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....",
    )
    .unwrap();
    assert_eq!(seat.visible_occupied_seats_from(3, 4), 8);

    let seat = Seating::from_str(
        ".............
.L.L.#.#.#.#.
.............",
    )
    .unwrap();
    assert_eq!(seat.visible_occupied_seats_from(1, 1), 0);

    let seat = Seating::from_str(
        ".##.##.
#.#.#.#
##...##
...L...
##...##
#.#.#.#
.##.##.",
    )
    .unwrap();
    assert_eq!(seat.visible_occupied_seats_from(3, 3), 0);
}

#[test]
fn test_iterate_2() {
    let seating = Seating::from_str(
        "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL",
    )
    .unwrap();
    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##
"
    );

    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.LL.LL.L#
#LLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLLL.L
#.LLLLL.L#
"
    );

    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.L#.##.L#
#L#####.LL
L.#.#..#..
##L#.##.##
#.##.#L.##
#.#####.#L
..#.#.....
LLL####LL#
#.L#####.L
#.L####.L#
"
    );

    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##LL.LL.L#
L.LL.LL.L#
#.LLLLL.LL
..L.L.....
LLLLLLLLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );

    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.#L.L#
#.L####.LL
..#.#.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );

    let seating = seating.iterate_2();
    assert_eq!(
        seating.to_string(),
        "#.L#.L#.L#
#LLLLLL.LL
L.L.L..#..
##L#.#L.L#
L.L#.LL.L#
#.LLLL#.LL
..#.L.....
LLL###LLL#
#.LLLLL#.L
#.L#LL#.L#
"
    );
}
