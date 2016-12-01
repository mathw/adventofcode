use instructions::{Turn, Step};

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub struct Offset {
    northings: i64,
    eastings: i64,
}

impl Offset {
    pub fn new() -> Offset {
        Offset {
            northings: 0,
            eastings: 0,
        }
    }

    pub fn move_blocks(&self, heading: &Heading, blocks: &u32) -> Offset {
        match heading {
            &Heading::North => {
                Offset {
                    northings: self.northings + *blocks as i64,
                    eastings: self.eastings,
                }
            }
            &Heading::South => {
                Offset {
                    northings: self.northings - *blocks as i64,
                    eastings: self.eastings,
                }
            }
            &Heading::East => {
                Offset {
                    northings: self.northings,
                    eastings: self.eastings + *blocks as i64,
                }
            }
            &Heading::West => {
                Offset {
                    northings: self.northings,
                    eastings: self.eastings - *blocks as i64,
                }
            }
        }
    }

    pub fn distance(&self) -> i64 {
        self.northings.abs() + self.eastings.abs()
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Heading {
    North,
    South,
    East,
    West,
}

pub fn explode_step(starting_heading: &Heading, step: &Step) -> Vec<Heading> {
    let new_heading = turn_heading(starting_heading, &step.turn);
    (0..step.blocks).map(|_| new_heading.clone()).collect()
}

#[derive(PartialEq, Eq, Debug, Hash, Clone)]
pub struct State {
    heading: Heading,
    offset: Offset,
}

impl State {
    pub fn new() -> State {
        State {
            heading: Heading::North,
            offset: Offset::new(),
        }
    }

    pub fn apply_turn(&self, turn: &Turn) -> State {
        State {
            heading: turn_heading(&self.heading, &turn),
            offset: self.offset.clone(),
        }
    }

    pub fn move_blocks(&self, blocks: &u32) -> State {
        State {
            heading: self.heading.clone(),
            offset: self.offset.move_blocks(&self.heading, blocks),
        }
    }

    pub fn distance(&self) -> i64 {
        self.offset.distance()
    }
}

fn turn_heading(heading: &Heading, turn: &Turn) -> Heading {
    match (heading, turn) {
        (&Heading::North, &Turn::Left) => Heading::West,
        (&Heading::North, &Turn::Right) => Heading::East,
        (&Heading::South, &Turn::Left) => Heading::East,
        (&Heading::South, &Turn::Right) => Heading::West,
        (&Heading::East, &Turn::Left) => Heading::North,
        (&Heading::East, &Turn::Right) => Heading::South,
        (&Heading::West, &Turn::Left) => Heading::South,
        (&Heading::West, &Turn::Right) => Heading::North,
    }
}

#[test]
fn test_turn_state() {
    let starting_state = State {
        heading: Heading::East,
        offset: Offset {
            northings: 6,
            eastings: 8,
        },
    };

    let turn = Turn::Right;

    let new_state = starting_state.apply_turn(&turn);

    assert_eq!(new_state.offset, starting_state.offset);
    assert_eq!(new_state.heading, Heading::South);
}

#[test]
fn test_explode_step() {
    let step = Step {
        turn: Turn::Left,
        blocks: 4,
    };

    let hs = explode_step(&Heading::North, &step);

    assert_eq!(hs.len(), 4);
    assert_eq!(hs,
               [Heading::West, Heading::West, Heading::West, Heading::West]);
}
