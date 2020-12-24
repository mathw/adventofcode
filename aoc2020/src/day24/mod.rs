use crate::dayerror::DayError;
use hex2d::{Coordinate, Direction};
use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Colour {
    Black,
    White,
}

impl Default for Colour {
    fn default() -> Self {
        Colour::White
    }
}

impl Default for &Colour {
    fn default() -> Self {
        &Colour::White
    }
}

impl Colour {
    fn flip(&self) -> Colour {
        match self {
            Colour::Black => Colour::White,
            Colour::White => Colour::Black,
        }
    }
}

pub fn part1() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let answer = run_part1(input)?;
    Ok(format!("There are {} black tiles", answer))
}

pub fn part2() -> Result<String, DayError> {
    let input = include_str!("input.txt");
    let answer = run_part2(input)?;
    Ok(format!("There are {} black tiles", answer))
}

fn parse_directions(line: &str) -> Result<Vec<Direction>, DayError> {
    let mut rest = line;
    let mut directions = Vec::new();
    while rest.len() > 0 {
        if rest.starts_with("ne") {
            rest = &rest[2..];
            directions.push(Direction::XZ);
        } else if rest.starts_with("nw") {
            rest = &rest[2..];
            directions.push(Direction::YZ);
        } else if rest.starts_with("se") {
            rest = &rest[2..];
            directions.push(Direction::ZY);
        } else if rest.starts_with("sw") {
            rest = &rest[2..];
            directions.push(Direction::ZX);
        } else if rest.starts_with("e") {
            rest = &rest[1..];
            directions.push(Direction::XY);
        } else if rest.starts_with("w") {
            rest = &rest[1..];
            directions.push(Direction::YX);
        } else {
            return Err(DayError::InputParseError(format!(
                "Unable to parse input string at {}",
                rest
            )));
        }
    }
    Ok(directions)
}

type Floor = HashMap<(i32, i32), Colour>;

fn get_part1_tiles(input: &str) -> Result<Floor, DayError> {
    let directions = input
        .lines()
        .map(|l| parse_directions(l))
        .collect::<Result<Vec<_>, _>>()?;
    // false = white, true = black
    let mut tiles = HashMap::new();
    for d in &directions {
        run_directions_from_origin(&mut tiles, d);
    }
    Ok(tiles)
}

fn run_part1(input: &str) -> Result<usize, DayError> {
    Ok(get_part1_tiles(input)?
        .values()
        .filter(|v| **v == Colour::Black)
        .count())
}

fn apply_life_rule(tiles: &Floor, coordinate: (i32, i32)) -> Colour {
    let coord = Coordinate::from(coordinate);
    let this_tile_value = *tiles.get(&coordinate).unwrap_or_default();
    let surrounding_black_tiles = coord
        .neighbors()
        .iter()
        .map(|c| tiles.get(&(c.x, c.y)).unwrap_or_default())
        .filter(|t| **t == Colour::Black)
        .count();
    if this_tile_value == Colour::Black {
        // it's black - should be white if there are zero or >2 black tiles
        if surrounding_black_tiles == 0 || surrounding_black_tiles > 2 {
            Colour::White
        } else {
            Colour::Black
        }
    } else {
        // it's white - should be black if there are exactly two surrounding black tiles
        if surrounding_black_tiles == 2 {
            Colour::Black
        } else {
            Colour::White
        }
    }
}

fn apply_life_step(floor: &Floor) -> Floor {
    let mut new_floor = HashMap::new();

    let (min_x, min_y, max_x, max_y) = bounds(floor);

    for x in (min_x - 1)..=(max_x + 1) {
        for y in (min_y - 1)..=(max_y + 1) {
            let new_colour = apply_life_rule(floor, (x, y));
            new_floor.insert((x, y), new_colour);
        }
    }

    new_floor
}

fn run_part2(input: &str) -> Result<usize, DayError> {
    let mut floor = get_part1_tiles(input)?;
    for _ in 0..100 {
        floor = apply_life_step(&floor);
    }
    Ok(floor.values().filter(|v| **v == Colour::Black).count())
}

fn bounds(floor: &Floor) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = i32::MIN;
    let mut max_y = i32::MIN;

    for (x, y) in floor.keys() {
        min_x = i32::min(min_x, *x);
        max_x = i32::max(max_x, *x);
        min_y = i32::min(min_y, *y);
        max_y = i32::max(max_y, *y);
    }
    (min_x, min_y, max_x, max_y)
}

fn run_directions_from_origin(tiles: &mut Floor, directions: &Vec<Direction>) {
    let mut current_location = Coordinate::from((0, 0));
    for dir in directions {
        current_location = current_location + *dir;
    }
    let tile = tiles
        .entry((current_location.x, current_location.y))
        .or_default();
    (*tile) = tile.flip()
}

#[test]
fn test_parse() {
    let directions = parse_directions("nwwswee").unwrap();
    assert_eq!(
        directions,
        vec![
            Direction::YZ,
            Direction::YX,
            Direction::ZX,
            Direction::XY,
            Direction::XY
        ]
    );
}

#[test]
fn test_part1_sample() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let result = run_part1(input).unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_part2_sample() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let result = run_part2(input).unwrap();
    assert_eq!(result, 2208);
}

#[test]
fn test_part2_sample_step1() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    let floor = get_part1_tiles(input).unwrap();
    let day1 = apply_life_step(&floor);
    assert_eq!(day1.values().filter(|v| **v == Colour::Black).count(), 15);
}
