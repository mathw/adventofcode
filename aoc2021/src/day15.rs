use crate::{
    common::grid::Grid,
    day::{DayResult, PartResult},
};
use pathfinding::directed::dijkstra::dijkstra;
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let part1 = part1(include_str!("inputs/day15.txt"))?;
    let part2 = part2(include_str!("inputs/day15.txt"))?;
    Ok(DayResult::new(
        PartResult::Success(format!("{} is the lowest risk path", part1)),
        PartResult::Success(format!(
            "{} is the lowest risk path in the monster grid",
            part2
        )),
    ))
}

fn part1(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid = parse_input_grid(input)?;
    Ok(solve_part1(&grid)?)
}

fn part2(input: &str) -> Result<usize, Box<dyn Error>> {
    let grid = parse_input_grid(input)?;
    Ok(solve_part2(&grid)?)
}

fn parse_input_grid(input: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines[0].len();

    let mut grid = Grid::new(width, lines.len());

    for (y, row) in lines.into_iter().enumerate() {
        for (x, c) in row.chars().enumerate() {
            grid.set(x, y, format!("{}", c).parse::<u8>()?);
        }
    }

    Ok(grid)
}

fn solve_part1(caves_grid: &Grid<u8>) -> Result<usize, String> {
    let successors = |node: &(usize, usize)| {
        caves_grid
            .surrounding_coords_no_diagonals(node.0, node.1)
            .map(|(nx, ny)| ((nx, ny), *caves_grid.get(nx, ny).unwrap() as usize))
    };
    let success =
        |node: &(usize, usize)| *node == (caves_grid.width() - 1, caves_grid.height() - 1);

    if let Some((_, cost)) = dijkstra(&(0, 0), successors, success) {
        Ok(cost)
    } else {
        Err(format!("Couldn't find a path :("))
    }
}

fn solve_part2(caves_grid: &Grid<u8>) -> Result<usize, String> {
    let mega_grid = make_full_map(&caves_grid);
    solve_part1(&mega_grid)
}

fn make_full_map(source_grid: &Grid<u8>) -> Grid<u8> {
    let mut target_grid = Grid::new(source_grid.width() * 5, source_grid.height() * 5);
    for gx in 0..5u8 {
        for gy in 0..5u8 {
            let add = gx + gy;
            let x_offset = source_grid.width() * gx as usize;
            let y_offset = source_grid.height() * gy as usize;
            for (x, y) in source_grid.all_coords() {
                let target_x = x + x_offset;
                let target_y = y + y_offset;
                let target_value = add_risk(*source_grid.get(x, y).unwrap(), add);
                target_grid.set(target_x, target_y, target_value);
            }
        }
    }
    target_grid
}

fn add_risk(risk: u8, add: u8) -> u8 {
    let add = add - (add / 9) * 9;
    let r = risk + add;
    if r > 9 {
        r - 9
    } else {
        r
    }
}

#[test]
fn test_part1_sample() {
    let caves = parse_input_grid(include_str!("inputs/samples/day15.txt")).unwrap();
    let total_risk = solve_part1(&caves).unwrap();
    assert_eq!(total_risk, 40);
}

#[test]
fn test_part2_sample() {
    let caves = parse_input_grid(include_str!("inputs/samples/day15.txt")).unwrap();
    let total_risk = solve_part2(&caves).unwrap();
    assert_eq!(total_risk, 315);
}
