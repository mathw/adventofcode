use crate::day::Day;
use crate::util::Trace;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;
use std::sync::mpsc::Sender;

pub struct Day13;

impl Default for Day13 {
    fn default() -> Day13 {
        Day13 {}
    }
}

impl Day for Day13 {
    fn part1(&mut self, sender: &Sender<String>) {
        let input = include_str!("input.txt");

        let track = ParsedRailway::from_str(input);
        match track {
            Err(msg) => sender.send(msg).unwrap(),
            Ok(pr) => {
                let mut railway = Railway::from(pr);
                let collision = railway.first_collision();
                sender
                    .send(format!(
                        "First collision at {},{}",
                        collision.0, collision.1
                    ))
                    .unwrap();
            }
        }
    }

    fn part2(&mut self, sender: &Sender<String>) {}
}

// a not as good track representation for all we can have from the first phase parser
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum ParsedTrack {
    Vertical,
    Horizontal,
    CornerNESW,
    CornerNWSE,
    Intersection,
}

impl ParsedTrack {
    fn from_char(c: char) -> Option<ParsedTrack> {
        match c {
            '-' => Some(ParsedTrack::Horizontal),
            '|' => Some(ParsedTrack::Vertical),
            '\\' => Some(ParsedTrack::CornerNESW),
            '/' => Some(ParsedTrack::CornerNWSE),
            '+' => Some(ParsedTrack::Intersection),
            _ => None,
        }
    }
}

// proper track representation after post-processing
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Track {
    Vertical,
    Horizontal,
    /// Corner which has rails going north and east of it
    CornerNE,
    /// Corner which has rails going north and west of it
    CornerNW,
    /// Corner which has rails going south and east of it
    CornerSE,
    /// Corner which has rails going south and west of it
    CornerSW,
    Intersection,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn(&self, turn: &IntersectionState) -> Direction {
        match (turn, *self) {
            (IntersectionState::Left, Direction::North) => Direction::West,
            (IntersectionState::Left, Direction::West) => Direction::South,
            (IntersectionState::Left, Direction::South) => Direction::East,
            (IntersectionState::Left, Direction::East) => Direction::North,
            (IntersectionState::Right, Direction::North) => Direction::East,
            (IntersectionState::Right, Direction::West) => Direction::North,
            (IntersectionState::Right, Direction::South) => Direction::West,
            (IntersectionState::Right, Direction::East) => Direction::South,
            (IntersectionState::Forward, dir) => dir,
        }
    }
}

struct ParsedRailway {
    track: HashMap<(usize, usize), ParsedTrack>,
    carts: HashMap<(usize, usize), Direction>,
}

impl ParsedRailway {
    fn track_at(&self, pos: (usize, usize)) -> Option<ParsedTrack> {
        self.track.get(&pos).map(|x| x.clone())
    }

    fn cart_at(&self, pos: (usize, usize)) -> Option<Direction> {
        self.carts.get(&pos).map(|x| x.clone())
    }
}

impl FromStr for ParsedRailway {
    type Err = String;

    fn from_str(input: &str) -> Result<ParsedRailway, Self::Err> {
        let mut track = HashMap::new();
        let mut carts = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if let Some(t) = ParsedTrack::from_char(cell) {
                    track.insert((x, y), t);
                } else if cell == '^' {
                    carts.insert((x, y), Direction::North);
                    track.insert((x, y), ParsedTrack::Vertical);
                } else if cell == '>' {
                    carts.insert((x, y), Direction::East);
                    track.insert((x, y), ParsedTrack::Horizontal);
                } else if cell == '<' {
                    carts.insert((x, y), Direction::West);
                    track.insert((x, y), ParsedTrack::Horizontal);
                } else if cell == 'v' {
                    carts.insert((x, y), Direction::South);
                    track.insert((x, y), ParsedTrack::Vertical);
                } else if cell != ' ' {
                    return Err(format!("Unexpected character '{}' at ({}, {})", cell, x, y));
                }
            }
        }

        Ok(ParsedRailway { track, carts })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum IntersectionState {
    Forward,
    Left,
    Right,
}

impl IntersectionState {
    fn next(&self) -> IntersectionState {
        match self {
            IntersectionState::Forward => IntersectionState::Right,
            IntersectionState::Left => IntersectionState::Forward,
            IntersectionState::Right => IntersectionState::Left,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Cart {
    facing: Direction,
    state: IntersectionState,
}

struct Railway {
    track: HashMap<(usize, usize), Track>,
    carts: HashMap<(usize, usize), Cart>,
}

impl Railway {
    fn track_at(&self, pos: (usize, usize)) -> Option<Track> {
        self.track.get(&pos).map(|x| x.clone())
    }

    fn cart_at(&self, pos: (usize, usize)) -> Option<Cart> {
        self.carts.get(&pos).map(|x| x.clone())
    }

    /// All carts move one tick
    /// Returns first collision location if there is one
    fn step(&mut self) -> Option<(usize, usize)> {
        #[cfg(test)]
        println!("Starting new step");

        let carts = self.carts_in_order();

        for cart_pos in carts {
            let cart_record = self
                .cart_at(cart_pos)
                .expect("There really should be a cart here!");

            let (new_pos, new_direction, new_state) =
                self.advance_position(cart_pos, cart_record.facing, cart_record.state);

            self.carts.remove(&cart_pos);

            if self
                .carts
                .insert(
                    new_pos,
                    Cart {
                        facing: new_direction,
                        state: new_state,
                    },
                )
                .is_some()
            {
                // COLLISION!!!!
                return Some(new_pos);
            }
        }

        None
    }

    fn carts_in_order(&self) -> Vec<(usize, usize)> {
        let mut carts_in_order: Vec<(usize, usize)> = self.carts.keys().cloned().collect();
        carts_in_order.sort_by(|(xa, ya), (xb, yb)| match ya.cmp(yb) {
            Ordering::Equal => xa.cmp(xb),
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
        });

        carts_in_order
    }

    fn advance_position(
        &self,
        pos: (usize, usize),
        dir: Direction,
        state: IntersectionState,
    ) -> ((usize, usize), Direction, IntersectionState) {
        // #[cfg(test)]
        println!(
            "Advance cart at ({}, {}) facing {:?} state {:?}",
            pos.0, pos.1, dir, state
        );

        match (dir, self.track_at(pos).expect("There has to be track here")) {
            (Direction::North, Track::Vertical)
            | (Direction::North, Track::CornerNE)
            | (Direction::North, Track::CornerNW)
            | (Direction::North, Track::Intersection) => {
                let new_pos = (pos.0, pos.1 - 1);
                let (new_direction, new_state) = match self
                    .track_at(new_pos)
                    .expect("There has to be track here too")
                    .trace()
                {
                    Track::CornerSE => (Direction::East, state),
                    Track::CornerSW => (Direction::West, state),
                    Track::Intersection => (dir.turn(&state), state.next()),
                    _ => (Direction::North, state),
                };
                (new_pos, new_direction, new_state)
            }
            (Direction::South, Track::Vertical)
            | (Direction::South, Track::CornerSE)
            | (Direction::South, Track::CornerSW)
            | (Direction::South, Track::Intersection) => {
                let new_pos = (pos.0, pos.1 + 1);
                let (new_direction, new_state) = match self
                    .track_at(new_pos)
                    .expect("There has to be track here too")
                {
                    Track::CornerNE => (Direction::East, state),
                    Track::CornerNW => (Direction::West, state),
                    Track::Intersection => (dir.turn(&state), state.next()),
                    _ => (Direction::South, state),
                };
                (new_pos, new_direction, new_state)
            }
            (Direction::East, Track::Horizontal)
            | (Direction::East, Track::CornerNE)
            | (Direction::East, Track::CornerSE)
            | (Direction::East, Track::Intersection) => {
                let new_pos = (pos.0 + 1, pos.1);
                let (new_direction, new_state) = match self
                    .track_at(new_pos)
                    .expect("There has to be track here too")
                {
                    Track::CornerSW => (Direction::South, state),
                    Track::CornerNW => (Direction::North, state),
                    Track::Intersection => (dir.turn(&state), state.next()),
                    _ => (Direction::East, state),
                };
                (new_pos, new_direction, new_state)
            }
            (Direction::West, Track::Horizontal)
            | (Direction::West, Track::CornerNW)
            | (Direction::West, Track::CornerSW)
            | (Direction::West, Track::Intersection) => {
                let new_pos = (pos.0 - 1, pos.1);
                let (new_direction, new_state) = match self
                    .track_at(new_pos)
                    .expect("There has to be track here too")
                {
                    Track::CornerSE => (Direction::South, state),
                    Track::CornerNE => (Direction::North, state),
                    Track::Intersection => (dir.turn(&state), state.next()),
                    _ => (Direction::West, state),
                };
                (new_pos, new_direction, new_state)
            }
            (dir, track) => panic!(format!(
                "Impossible track/position combination {:?} {:?}",
                dir, track
            )),
        }
    }

    fn first_collision(&mut self) -> (usize, usize) {
        let mut step = 0;
        loop {
            step += 1;
            println!("Starting step {}", step);
            match self.step() {
                Some(pos) => return pos,
                None => (),
            };
        }
    }
}

impl From<ParsedRailway> for Railway {
    fn from(parsed: ParsedRailway) -> Railway {
        let mut track = HashMap::new();

        for ((x, y), cell) in parsed.track.iter() {
            match cell {
                ParsedTrack::Horizontal => {
                    track.insert((*x, *y), Track::Horizontal);
                }
                ParsedTrack::Vertical => {
                    track.insert((*x, *y), Track::Vertical);
                }
                ParsedTrack::Intersection => {
                    track.insert((*x, *y), Track::Intersection);
                }
                ParsedTrack::CornerNESW => {
                    // so, is it NE or SW?
                    track.insert(
                        (*x, *y),
                        match parsed.track_at((*x, y + 1)) {
                            Some(ParsedTrack::Vertical)
                            | Some(ParsedTrack::Intersection)
                            | Some(ParsedTrack::CornerNWSE) => Track::CornerSW,
                            _ => Track::CornerNE,
                        },
                    );
                }
                ParsedTrack::CornerNWSE => {
                    // so, is it NW or SE?
                    track.insert(
                        (*x, *y),
                        match if *x > 0 {
                            parsed.track_at((x - 1, *y))
                        } else {
                            None
                        } {
                            Some(ParsedTrack::Horizontal)
                            | Some(ParsedTrack::Intersection)
                            | Some(ParsedTrack::CornerNESW) => Track::CornerNW,
                            _ => Track::CornerSE,
                        },
                    );
                }
            }
        }

        Railway {
            track,
            carts: parsed
                .carts
                .into_iter()
                .map(|(p, d)| {
                    (
                        p,
                        Cart {
                            facing: d,
                            state: IntersectionState::Left,
                        },
                    )
                })
                .collect(),
        }
    }
}

#[test]
fn parse_simple_loop() {
    let track = r"/----\
|    |
|    |
|    |
|    |
\----/";

    let track = ParsedRailway::from_str(track).expect("Track should parse");
    assert_eq!(track.track_at((0, 0)), Some(ParsedTrack::CornerNWSE));
    assert_eq!(track.track_at((1, 0)), Some(ParsedTrack::Horizontal));
    assert_eq!(track.track_at((1, 1)), None);
    assert_eq!(track.track_at((0, 1)), Some(ParsedTrack::Vertical));
}

#[test]
fn postconvert_simple_loop() {
    let track = r"/----\
|    |
v    |
|    |
|    |
\----/";

    let track = ParsedRailway::from_str(track).expect("Track should parse");
    let track = Railway::from(track);

    assert_eq!(track.track_at((0, 0)), Some(Track::CornerSE));
    assert_eq!(track.track_at((1, 0)), Some(Track::Horizontal));
    assert_eq!(track.track_at((5, 0)), Some(Track::CornerSW));
    assert_eq!(track.track_at((5, 5)), Some(Track::CornerNW));
    assert_eq!(track.track_at((0, 5)), Some(Track::CornerNE));
    assert_eq!(track.track_at((0, 2)), Some(Track::Vertical));
    assert_eq!(
        track.cart_at((0, 2)),
        Some(Cart {
            facing: Direction::South,
            state: IntersectionState::Left
        })
    );
}

#[test]
fn step_simple_loop() {
    let track = r"/----\
|    |
v    |
|    |
|    |
\----/";

    let track = ParsedRailway::from_str(track).expect("Track should parse");
    let mut track = Railway::from(track);

    track.step();
    assert_eq!(track.cart_at((0, 2)), None);
    assert_eq!(
        track.cart_at((0, 3)),
        Some(Cart {
            facing: Direction::South,
            state: IntersectionState::Left
        })
    );

    track.step();
    assert_eq!(
        track.cart_at((0, 4)),
        Some(Cart {
            facing: Direction::South,
            state: IntersectionState::Left
        })
    );

    track.step();
    assert_eq!(
        track.cart_at((0, 5)),
        Some(Cart {
            facing: Direction::East,
            state: IntersectionState::Left
        })
    );

    track.step();
    assert_eq!(
        track.cart_at((1, 5)),
        Some(Cart {
            facing: Direction::East,
            state: IntersectionState::Left
        })
    );
}

#[test]
fn simple_intersection() {
    let track = r"     |
----+--
    ^  
    |";
    let mut railway = Railway::from(ParsedRailway::from_str(track).expect("Track should parse"));

    railway.step();
    railway.step();
    assert_eq!(
        railway.cart_at((3, 1)),
        Some(Cart {
            facing: Direction::West,
            state: IntersectionState::Forward
        })
    );
}

#[test]
fn turn_cart() {
    let direction = Direction::North;
    let new_direction = direction.turn(&IntersectionState::Left);
    assert_eq!(new_direction, Direction::West);
}

#[test]
fn simple_collision() {
    let track = r"->--<-";
    let mut railway = Railway::from(ParsedRailway::from_str(track).expect("Track should parse"));

    let step = railway.step();
    assert_eq!(step, None);

    let step = railway.step();
    assert_eq!(step, Some((3, 0)));
}

#[test]
fn simple_collision_2() {
    let track = r"->--<-";
    let mut railway = Railway::from(ParsedRailway::from_str(track).expect("Track should parse"));

    let collision = railway.first_collision();
    assert_eq!(collision, (3, 0));
}

#[test]
fn test_first_failure() {
    let track = r"---\
->-+";

    let mut railway = Railway::from(ParsedRailway::from_str(track).expect("Track should parse"));

    let _step = railway.step();
    assert_eq!(
        railway.cart_at((2, 1)),
        Some(Cart {
            facing: Direction::East,
            state: IntersectionState::Left
        })
    );
    let _step = railway.step();
    assert_eq!(
        railway.cart_at((3, 1)),
        Some(Cart {
            facing: Direction::North,
            state: IntersectionState::Forward
        })
    );
    let _step = railway.step();
    assert_eq!(
        railway.cart_at((3, 0)),
        Some(Cart {
            facing: Direction::West,
            state: IntersectionState::Forward
        })
    );
}
