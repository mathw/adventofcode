use crate::dayerror::DayError;
use std::str::FromStr;

pub fn part1() -> Result<String, DayError> {
    let seat_ids = get_all_seat_ids(include_str!("input.txt"))?;

    let max_seat_id = seat_ids
        .iter()
        .max()
        .ok_or(DayError::NoSolutionFoundError)?;

    Ok(format!("Max seat ID is {}", max_seat_id))
}

pub fn part2() -> Result<String, DayError> {
    let mut seat_ids = get_all_seat_ids(include_str!("input.txt"))?;

    seat_ids.sort();

    let missing_seat_id = find_gap_in(seat_ids).ok_or(DayError::NoSolutionFoundError)?;

    Ok(format!("The missing seat ID is {}", missing_seat_id))
}

fn find_gap_in(sorted_sequence: impl IntoIterator<Item = usize>) -> Option<usize> {
    sorted_sequence
        .into_iter()
        .fold((None, 0), |(current_guess, last), i| {
            if i > last + 1 {
                (Some(i - 1), i)
            } else {
                (current_guess, i)
            }
        })
        .0
}

fn get_all_seat_ids(input: &str) -> Result<Vec<usize>, DayError> {
    let passes = input
        .lines()
        .map(|l| l.trim())
        .filter(|l| l.len() > 0)
        .map(|l| BoardingPass::from_str(l))
        .collect::<Result<Vec<BoardingPass>, DayError>>()?;

    passes
        .iter()
        .map(|p| p.seat_id())
        .collect::<Option<Vec<usize>>>()
        .ok_or(DayError::NoSolutionFoundError)
}

#[derive(Debug)]
struct BoardingPass {
    row_directions: Vec<RowDirection>,
    column_directions: Vec<ColumnDirection>,
}

impl FromStr for BoardingPass {
    type Err = DayError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(DayError::InputParseError(format!(
                "Boarding pass is the wrong length: {}",
                s
            )));
        }
        let row = s
            .chars()
            .take(7)
            .map(|c| RowDirection::from_char(c))
            .collect::<Result<Vec<_>, _>>()?;
        let col = s
            .chars()
            .skip(7)
            .take(3)
            .map(|c| ColumnDirection::from_char(c))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(BoardingPass {
            row_directions: row,
            column_directions: col,
        })
    }
}

impl BoardingPass {
    fn row_number(&self) -> Option<usize> {
        let mut range = (0..=127).collect::<Vec<usize>>();
        for dir in &self.row_directions {
            match dir {
                RowDirection::Front => range = front_half(range),
                RowDirection::Back => range = back_half(range),
            };
            if range.len() == 1 {
                return Some(range[0]);
            }
        }
        None
    }

    fn col_number(&self) -> Option<usize> {
        let mut range = (0..=7).collect::<Vec<usize>>();
        for dir in &self.column_directions {
            match dir {
                ColumnDirection::Left => range = front_half(range),
                ColumnDirection::Right => range = back_half(range),
            };
            if range.len() == 1 {
                return Some(range[0]);
            }
        }
        None
    }

    fn seat_id(&self) -> Option<usize> {
        Some((self.row_number()? * 8) + self.col_number()?)
    }
}

fn front_half<T>(range: Vec<T>) -> Vec<T> {
    let segment = range.len() / 2;
    range.into_iter().take(segment).collect()
}

fn back_half<T>(range: Vec<T>) -> Vec<T> {
    let segment = range.len() / 2;
    range.into_iter().skip(segment).take(segment).collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum RowDirection {
    Front,
    Back,
}

impl RowDirection {
    fn from_char(c: char) -> Result<Self, DayError> {
        match c {
            'F' => Ok(RowDirection::Front),
            'B' => Ok(RowDirection::Back),
            _ => Err(DayError::InputParseError(format!(
                "Bad row direction {}",
                c
            ))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Copy)]
enum ColumnDirection {
    Left,
    Right,
}

impl ColumnDirection {
    fn from_char(c: char) -> Result<Self, DayError> {
        match c {
            'L' => Ok(ColumnDirection::Left),
            'R' => Ok(ColumnDirection::Right),
            _ => Err(DayError::InputParseError(format!(
                "Bad column direction {}",
                c
            ))),
        }
    }
}

#[test]
fn test_parse_pass() {
    let result = BoardingPass::from_str("L");
    assert!(result.is_err());

    let result = BoardingPass::from_str("FBFBFBFLRL");
    assert!(result.is_ok());

    let result = result.unwrap();
    assert_eq!(
        result.column_directions,
        vec![
            ColumnDirection::Left,
            ColumnDirection::Right,
            ColumnDirection::Left
        ]
    );
    assert_eq!(
        result.row_directions,
        vec![
            RowDirection::Front,
            RowDirection::Back,
            RowDirection::Front,
            RowDirection::Back,
            RowDirection::Front,
            RowDirection::Back,
            RowDirection::Front
        ]
    );
}

#[test]
fn test_row_number() {
    let pass = BoardingPass::from_str("FBFBBFFRLR").expect("Test data should parse");
    assert_eq!(pass.row_number(), Some(44));
    let pass = BoardingPass::from_str("BFFFBBFRRR").expect("Test data should parse");
    assert_eq!(pass.row_number(), Some(70));
    let pass = BoardingPass::from_str("FFFBBBFRRR").expect("Test data should parse");
    assert_eq!(pass.row_number(), Some(14));
    let pass = BoardingPass::from_str("BBFFBBFRLL").expect("Test data should parse");
    assert_eq!(pass.row_number(), Some(102));
}

#[test]
fn test_col_number() {
    let pass = BoardingPass::from_str("FBFBBFFRLR").expect("Test data should parse");
    assert_eq!(pass.col_number(), Some(5));
    let pass = BoardingPass::from_str("BFFFBBFRRR").expect("Test data should parse");
    assert_eq!(pass.col_number(), Some(7));
    let pass = BoardingPass::from_str("FFFBBBFRRR").expect("Test data should parse");
    assert_eq!(pass.col_number(), Some(7));
    let pass = BoardingPass::from_str("BBFFBBFRLL").expect("Test data should parse");
    assert_eq!(pass.col_number(), Some(4));
}

#[test]
fn test_seat_id() {
    let pass = BoardingPass::from_str("FBFBBFFRLR").expect("Test data should parse");
    assert_eq!(pass.seat_id(), Some(357));
    let pass = BoardingPass::from_str("BFFFBBFRRR").expect("Test data should parse");
    assert_eq!(pass.seat_id(), Some(567));
    let pass = BoardingPass::from_str("FFFBBBFRRR").expect("Test data should parse");
    assert_eq!(pass.seat_id(), Some(119));
    let pass = BoardingPass::from_str("BBFFBBFRLL").expect("Test data should parse");
    assert_eq!(pass.seat_id(), Some(820));
}

#[test]
fn test_find_gap_no_gap() {
    let gap = find_gap_in(vec![1, 2, 3, 4]);
    assert!(gap.is_none());
}

#[test]
fn test_find_gap() {
    let gap = find_gap_in(vec![1, 2, 4, 5]);
    assert_eq!(gap, Some(3));
}
