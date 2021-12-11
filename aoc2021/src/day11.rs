use crate::common::grid::Grid;
use crate::day::{DayResult, PartResult};
use std::collections::VecDeque;
use std::error::Error;

pub fn run() -> Result<DayResult, Box<dyn Error>> {
    let part1_flashes = part1(include_str!("inputs/day11.txt"))?;
    let part2_step = part2(include_str!("inputs/day11.txt"))?;
    Ok(DayResult::new(
        PartResult::Success(format!("There were {} flashes", part1_flashes)),
        PartResult::Success(format!(
            "The octopodes will flash simultaneously at step {}",
            part2_step
        )),
    ))
}

fn part1(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut input = parse_input(input)?;
    let mut flashes = 0;
    for _ in 0..100 {
        flashes += step(&mut input);
    }
    Ok(flashes)
}

fn part2(input: &str) -> Result<u64, Box<dyn Error>> {
    let mut input = parse_input(input)?;
    let num_octopodes = (input.width() * input.height()) as u64;
    for step_num in 1.. {
        let flashes = step(&mut input);
        if flashes == num_octopodes {
            return Ok(step_num);
        }
    }
    Err(format!("The octopodes did not simultaneously flash").into())
}

fn step(octopodes: &mut Grid<u8>) -> u64 {
    let mut flashed: Grid<bool> = Grid::new(octopodes.width(), octopodes.height());

    // first, increase each octopus by 1
    octopodes.mutate_all(|o| increase_octopus(*o));

    // now find all octopodes which need to flash
    let mut to_flash = all_to_flash(&octopodes);
    while to_flash.len() > 0 {
        flash_octopus(to_flash[0], &mut flashed, octopodes, &mut to_flash);
        to_flash.pop_front();
    }

    let mut flashes = 0;
    for (x, y) in flashed.coords_where(|flashed| *flashed) {
        octopodes.set(x, y, 0);
        flashes += 1;
    }
    flashes
}

fn should_flash(energy: u8) -> bool {
    energy > 9
}

fn all_to_flash(octopodes: &Grid<u8>) -> VecDeque<(usize, usize)> {
    octopodes.coords_where(|o| should_flash(*o)).collect()
}

fn flash_octopus(
    (x, y): (usize, usize),
    flashed: &mut Grid<bool>,
    octopodes: &mut Grid<u8>,
    to_flash: &mut VecDeque<(usize, usize)>,
) {
    if let Some(has_flashed) = flashed.get(x, y) {
        if *has_flashed {
            return;
        }
    }
    flashed.set(x, y, true);
    octopodes.set(x, y, 0);
    let surrounding = octopodes.surrounding_coords(x, y).collect::<Vec<_>>();
    for &(sx, sy) in surrounding.iter() {
        octopodes.mutate(sx, sy, |o| increase_octopus(*o));
        if let Some(o) = octopodes.get(sx, sy) {
            if should_flash(*o) {
                if let Some(f) = flashed.get(sx, sy) {
                    if !f {
                        to_flash.push_back((sx, sy))
                    }
                }
            }
        }
    }
}

fn increase_octopus(energy: u8) -> u8 {
    if energy > 9 {
        10
    } else {
        energy + 1
    }
}

fn parse_input(input: &str) -> Result<Grid<u8>, Box<dyn Error>> {
    let rows = input.lines().collect::<Vec<_>>();
    let mut grid = Grid::new(rows[0].len(), rows.len());
    for (y, row) in input.lines().enumerate() {
        for (x, c) in row.chars().enumerate() {
            let energy = match c {
                '0' => 0,
                '1' => 1,
                '2' => 2,
                '3' => 3,
                '4' => 4,
                '5' => 5,
                '6' => 6,
                '7' => 7,
                '8' => 8,
                '9' => 9,
                _ => return Err(format!("Invalid input character {}", c).into()),
            };
            grid.set(x, y, energy);
        }
    }
    Ok(grid)
}

#[test]
fn test_part1_sample_step1() {
    let mut squid = parse_input(include_str!("inputs/samples/day11.txt")).unwrap();
    step(&mut squid);

    let should_be = parse_input(
        "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637",
    )
    .unwrap();

    assert_eq!(squid, should_be);
}

#[test]
fn test_part1_sample_step2() {
    let mut squid = parse_input(
        "6594254334
3856965822
6375667284
7252447257
7468496589
5278635756
3287952832
7993992245
5957959665
6394862637",
    )
    .unwrap();

    step(&mut squid);

    let should_be = parse_input(
        "8807476555
5089087054
8597889608
8485769600
8700908800
6600088989
6800005943
0000007456
9000000876
8700006848",
    )
    .unwrap();

    assert_eq!(squid, should_be);
}

#[test]
fn test_part1_small_sample_step1() {
    let mut squid = parse_input(
        "11111
19991
19191
19991
11111",
    )
    .unwrap();

    step(&mut squid);

    let should_be = parse_input(
        "34543
40004
50005
40004
34543",
    )
    .unwrap();

    assert_eq!(squid, should_be);
}

#[test]
fn test_part2_sample() {
    let step = part2(include_str!("inputs/samples/day11.txt")).unwrap();
    assert_eq!(step, 195);
}
