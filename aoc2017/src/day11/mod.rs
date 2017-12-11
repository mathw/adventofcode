mod hexgrid;

use self::hexgrid::Direction;
use self::hexgrid::HexCoordinate;
use std::str::FromStr;
use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let directions = parse_input(input);

    let (result, time) = timed(|| part1(directions.clone()));

    println!("[{}ms] Distance is {}", time, result);
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.trim().split(',').filter_map(|d| Direction::from_str(d.trim()).ok()).collect()
}

fn part1<I>(input: I) -> usize
    where I: IntoIterator<Item = Direction>
{
    HexCoordinate::new(0, 0).follow_directions(input).distance_from_origin()
}