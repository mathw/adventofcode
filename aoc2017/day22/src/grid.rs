use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Coord {
    x: isize,
    y: isize,
}

impl Coord {
    fn new(x: isize, y: isize) -> Coord {
        Coord { x: x, y: y }
    }
}

pub struct Grid {
    squares: HashMap<Coord, bool>,
}


impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Grid, Self::Err> {
        let mut map = HashMap::new();
        for (y, line) in s.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                match cell {
                    '#' => map.insert(Coord::new(x as isize, y as isize), true),
                    '.' => map.insert(Coord::new(x as isize, y as isize), false),
                    _ => return Err(()),
                };
            }
        }
        Ok(Grid { squares: map })
    }
}
