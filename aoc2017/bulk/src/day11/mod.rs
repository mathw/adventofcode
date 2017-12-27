mod hexgrid;

use self::hexgrid::Direction;
use self::hexgrid::HexCoordinate;
use std::str::FromStr;
use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let directions = parse_input(input);

    let ((distance, max_distance), time) = timed(|| run(directions.clone()));

    println!("[{}ms] Distance is {} and the maximum distance ever was {}",
             time,
             distance,
             max_distance);
}

fn parse_input(input: &str) -> Vec<Direction> {
    input.trim().split(',').filter_map(|d| Direction::from_str(d.trim()).ok()).collect()
}

fn run<I>(input: I) -> (usize, usize)
    where I: IntoIterator<Item = Direction>
{
    let (endpoint, max_distance) = HexCoordinate::new(0, 0)
        .follow_directions_tracking_distance(input);
    (endpoint.distance_from_origin(), max_distance)
}