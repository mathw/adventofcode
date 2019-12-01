use std::str::FromStr;

pub enum Tile {
    Wall,
    Open,
}

pub enum Unit {
    Elf { health: usize, index: usize },
    Goblin { health: usize, index: usize },
}

impl Unit {
    fn index(&self) -> usize {
        match self {
            Unit::Elf { index, .. } => *index,
            Unit::Goblin { index, .. } => *index,
        }
    }

    fn is_goblin(&self) -> bool {
        match self {
            Unit::Elf { .. } => false,
            Unit::Goblin { .. } => true,
        }
    }

    fn is_elf(&self) -> bool {
        !self.is_goblin()
    }

    fn identify_targets<'a>(&self, map: &'a Map) -> Vec<&'a Unit> {
        match self {
            Unit::Elf { .. } => map.units.iter().filter(|u| u.is_goblin()).collect(),
            Unit::Goblin { .. } => map.units.iter().filter(|u| u.is_elf()).collect(),
        }
    }
}

pub struct Map {
    tiles: Vec<Tile>,
    width: usize,
    height: usize,
    units: Vec<Unit>,
}

impl Map {
    fn sort_units(&mut self) {
        self.units.sort_by_key(|u| u.index());
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut result = Map {
            tiles: vec![],
            width: 0,
            height: 0,
            units: vec![],
        };

        for line in input.trim().lines() {
            let line = line.trim();
            if result.width == 0 {
                result.width = line.len();
            } else if result.width != line.len() {
                return Err(format!(
                    "Line length {} does not match previous line length {}",
                    line.len(),
                    result.width
                ));
            }
            result.height += 1;

            for c in line.chars() {
                match c {
                    '#' => result.tiles.push(Tile::Wall),
                    '.' => result.tiles.push(Tile::Open),
                    'G' => {
                        result.tiles.push(Tile::Open);
                        result.units.push(Unit::Goblin {
                            health: 300,
                            index: result.tiles.len() - 1,
                        })
                    }
                    'E' => {
                        result.tiles.push(Tile::Open);
                        result.units.push(Unit::Elf {
                            health: 300,
                            index: result.tiles.len() - 1,
                        })
                    }
                    _ => return Err(format!("Unexpected map character {}", c)),
                }
            }
        }

        Ok(result)
    }
}
