use crate::{
    common::grid::Grid,
    day::{DayResult, PartResult},
};
use regex::Regex;
use std::{error::Error, fmt::Display};

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let input_dots = include_str!("inputs/day13/dots.txt");
    let input_instructions = include_str!("inputs/day13/instructions.txt");
    let grid = parse_dots(input_dots)?;
    let instructions = parse_instructions(input_instructions)?;

    let part1 = part1(&grid, &instructions).ok_or(format!("Failed part 1"))?;
    let part2 = part2(&grid, &instructions).ok_or(format!("Failed part 2"))?;

    Ok(DayResult::new(
        PartResult::Success(format!("{} dots", part1)),
        PartResult::Success(format!("\n{}", part2)),
    ))
}

fn part1(grid: &Grid<bool>, instructions: &Vec<Instruction>) -> Option<usize> {
    let new_grid = execute_instruction(*instructions.get(0)?, grid)?;
    Some(count_grid_dots(&new_grid))
}

fn part2(grid: &Grid<bool>, instructions: &Vec<Instruction>) -> Option<String> {
    let new_grid = execute_instructions(instructions.iter().cloned(), grid)?;
    Some(new_grid.to_string())
}

fn parse_dots(input: &str) -> Result<Grid<bool>, Box<dyn Error>> {
    let points = input
        .lines()
        .map(|l| l.trim().split(',').collect::<Vec<_>>())
        .map(|parts| {
            if parts.len() != 2 {
                Err("Line found with no ,".into())
            } else {
                Ok((parts[0].parse::<usize>()?, parts[1].parse::<usize>()?))
            }
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;
    let width = points.iter().map(|(x, _)| x).max().ok_or("no_points")? + 1;
    let height = points.iter().map(|(_, y)| y).max().ok_or("no points")? + 1;
    let mut grid = Grid::new(width, height);

    for (x, y) in points {
        grid.set(x, y, true).ok_or(format!(
            "Point {},{} is outside the grid that I just sized to fit these points",
            x, y
        ))?;
    }

    Ok(grid)
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Instruction {
    FoldY(usize),
    FoldX(usize),
}

fn parse_instructions(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"fold along (?P<axis>.)=(?P<value>\d+)").unwrap();
    }

    input
        .lines()
        .map::<Result<Instruction, Box<dyn Error>>, _>(|l| {
            let captures = RE
                .captures(l.trim())
                .ok_or_else(|| format!("line \"{}\" doesn't match", l))?;
            Ok(match &captures["axis"] {
                "x" => Ok(Instruction::FoldX(captures["value"].parse::<usize>()?)),
                "y" => Ok(Instruction::FoldY(captures["value"].parse::<usize>()?)),
                _ => Err(format!("Invalid axis found")),
            }?)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn fold_grid_horizontal(grid: &Grid<bool>, y: usize) -> Option<Grid<bool>> {
    let mut new_grid = Grid::new(grid.width(), y);
    for target_y in 0..y {
        let partner_y = grid.height() - target_y - 1;
        for x in 0..grid.width() {
            new_grid.set(
                x,
                target_y,
                *grid.get(x, target_y)? || *grid.get(x, partner_y)?,
            );
        }
    }
    Some(new_grid)
}

fn fold_grid_vertical(grid: &Grid<bool>, x: usize) -> Option<Grid<bool>> {
    let mut new_grid = Grid::new(x, grid.height());
    for target_x in 0..x {
        let partner_x = grid.width() - target_x - 1;
        for y in 0..grid.height() {
            new_grid.set(
                target_x,
                y,
                *grid.get(target_x, y)? || *grid.get(partner_x, y)?,
            );
        }
    }
    Some(new_grid)
}

fn count_grid_dots(grid: &Grid<bool>) -> usize {
    grid.all_values().filter(|v| **v).count()
}

fn execute_instruction(instruction: Instruction, grid: &Grid<bool>) -> Option<Grid<bool>> {
    match instruction {
        Instruction::FoldY(y) => fold_grid_horizontal(grid, y),
        Instruction::FoldX(x) => fold_grid_vertical(grid, x),
    }
}

fn execute_instructions(
    instructions: impl Iterator<Item = Instruction>,
    grid: &Grid<bool>,
) -> Option<Grid<bool>> {
    let mut new_grid = grid.clone();
    for instruction in instructions {
        new_grid = execute_instruction(instruction, &new_grid)?;
    }
    Some(new_grid)
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for y in 0..self.height() {
            for x in 0..self.width() {
                write!(f, "{}", if *self.get(x, y).unwrap() { '#' } else { ' ' })?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[test]
fn test_parse_dots() {
    let grid =
        parse_dots(include_str!("inputs/samples/day13/dots.txt")).expect("This should parse");
    assert_eq!(grid.get(6, 10), Some(&true));
    assert_eq!(grid.get(2, 14), Some(&true));
    assert_eq!(grid.get(2, 13), Some(&false));
}

#[test]
fn test_parse_instructions() {
    let instructions = parse_instructions(include_str!("inputs/samples/day13/instructions.txt"))
        .expect("This should parse");
    assert_eq!(instructions[0], Instruction::FoldY(7));
    assert_eq!(instructions[1], Instruction::FoldX(5));
}

#[test]
fn test_part1_sample() {
    let dots =
        parse_dots(include_str!("inputs/samples/day13/dots.txt")).expect("this should parse");
    let instructions = parse_instructions(include_str!("inputs/samples/day13/instructions.txt"))
        .expect("this should also parse");
    let result = execute_instruction(instructions[0], &dots).expect("Expecting a grid");
    let dots_count = count_grid_dots(&result);
    assert_eq!(dots_count, 17);
    let result = execute_instruction(instructions[1], &result).expect("expecting another grid");
    let dots_count = count_grid_dots(&result);
    assert_eq!(dots_count, 16);
}

#[test]
fn test_part2_sample() {
    let dots =
        parse_dots(include_str!("inputs/samples/day13/dots.txt")).expect("this should parse");
    let instructions = parse_instructions(include_str!("inputs/samples/day13/instructions.txt"))
        .expect("this should also parse");
    let result = part2(&dots, &instructions).unwrap();

    assert_eq!(
        result,
        "#####
#...#
#...#
#...#
#####
.....
....."
    );
}
