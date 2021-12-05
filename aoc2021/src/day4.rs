use crate::bingo::Board;
use crate::day::{DayResult, PartResult};

use itertools::Itertools;
use rayon::prelude::*;
use std::error::Error;
use std::str::FromStr;

pub fn run() -> Result<DayResult, Box<dyn Error + 'static>> {
    let input = include_str!("inputs/day4.txt");
    let part1 = run_part1(input)?;
    let part2 = run_part2(input)?;
    Ok(DayResult::new(
        PartResult::Success(format!("Score of winning board is {}", part1)),
        PartResult::Success(format!("Score of winning board is {}", part2)),
    ))
}

fn run_part1(input: &str) -> Result<u32, Box<dyn Error + 'static>> {
    let (draw, boards) = parse_input(input)?;
    let result = play(&boards, &draw);
    match result {
        None => Err(format!("No winning board found").into()),
        Some((winner, lastcalled)) => Ok(board_score(&winner) * lastcalled as u32),
    }
}

fn run_part2(input: &str) -> Result<u32, Box<dyn Error + 'static>> {
    let (draw, boards) = parse_input(input)?;
    let result = play_to_win_last(&boards, &draw);
    match result {
        None => Err(format!("No winning board found").into()),
        Some((winner, lastcalled)) => Ok(board_score(&winner) * lastcalled as u32),
    }
}

fn parse_input(input: &str) -> Result<(Vec<u8>, Vec<Board>), Box<dyn Error + 'static>> {
    // first line is the numbers to draw
    let lines: Vec<_> = input.lines().collect();

    let first_line = lines.get(0).ok_or("I expected a first line! Come on!")?;
    let drawn_numbers = first_line
        .split(',')
        .map(|x| u8::from_str(x))
        .collect::<Result<Vec<u8>, _>>()?;
    // skip blank line

    // now every five lines then a blank is a board
    let mut boards = Vec::new();
    let chunks = &lines[2..].into_iter().chunks(6);
    for mut board_lines in chunks {
        let s = board_lines.join("\n");
        let board = Board::from_str(&s)?;
        boards.push(board);
    }
    Ok((drawn_numbers, boards))
}

fn play(boards: &Vec<Board>, order: &Vec<u8>) -> Option<(Board, u8)> {
    boards
        .par_iter()
        .map(|b| play_board(b.clone(), order.iter().cloned()))
        .min_by(|(_, _, moves1), (_, _, moves2)| moves1.cmp(moves2))
        .map(|(board, _, moves)| (board, order[moves - 1]))
}

fn play_to_win_last(boards: &Vec<Board>, order: &Vec<u8>) -> Option<(Board, u8)> {
    boards
        .par_iter()
        .map(|b| play_board(b.clone(), order.iter().cloned()))
        .max_by(|(_, _, moves1), (_, _, moves2)| moves1.cmp(moves2))
        .map(|(board, _, moves)| (board, order[moves - 1]))
}

fn play_board(board: Board, order: impl Iterator<Item = u8>) -> (Board, bool, usize) {
    let mut board = board;
    let mut moves = 0;

    for draw in order {
        board.mark_number(draw);
        moves += 1;
        if board.is_win() {
            return (board, true, moves);
        }
    }

    (board, false, moves)
}

fn board_score(board: &Board) -> u32 {
    board
        .all_unmarked_numbers()
        .into_iter()
        .map(|n| n as u32)
        .sum()
}

#[cfg(test)]
use crate::bingo::BoardState;

#[test]
fn test_parse_input() {
    let input = "2,3,4

2  4  8  2  1
2  98 2  1  1
2  2  2  2  2
0  0  0  0  0
1  1  1  1  1

7 8 9 10 11
13 14 15 16 17
18 19 20 21 22
23 24 25 26 27
28 29 30 31 32
";
    let (drawn, boards) =
        parse_input(input).expect("Really should not parse fail here in this test");

    assert_eq!(drawn, vec![2, 3, 4]);
    assert_eq!(boards.len(), 2);
    assert_eq!(boards[0].number_at(0, 0), Some(2));
    assert_eq!(boards[0].number_at(1, 1), Some(98));
    assert_eq!(boards[1].number_at(4, 2), Some(30));
}

#[test]
fn test_part1_sample_single_board() {
    let board = Board::from_str(
        "14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
2  0 12  3  7",
    )
    .unwrap();

    let (_played_board, won, moves) = play_board(
        board,
        vec![7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24].into_iter(),
    );
    assert_eq!(won, true);
    assert_eq!(moves, 12);
}

#[test]
fn test_part1_sample() {
    let input = include_str!("inputs/day4-sample.txt");
    let (drawn, boards) = parse_input(input).unwrap();

    let (result, last_called) = play(&boards, &drawn).expect("Expected a winning board");
    assert_eq!(result.check_state(), BoardState::WinRow(0));
    assert_eq!(last_called, 24);
}

#[test]
fn test_part2_sample() {
    let input = include_str!("inputs/day4-sample.txt");
    let (drawn, boards) = parse_input(input).unwrap();

    let (result, last_called) =
        play_to_win_last(&boards, &drawn).expect("Expected a winning board");
    assert_eq!(result.check_state(), BoardState::WinColumn(2));
    assert_eq!(last_called, 13);
}
