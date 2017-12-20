use std::collections::HashMap;
use util::timed;

pub fn go() {
    let input = include_str!("input.txt");

    let ((letters, steps), time) = timed(|| run_maze(input));
    println!("[{}ms] {} in {} steps", time, letters, steps);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize
}

impl Pos {
    fn new(x: usize, y: usize) -> Pos {
        Pos {
            x: x, y: y
        }
    }

    /// Determine the coordinate in the specified direction
    /// Returns None if that would go out of usize range
    fn next_in_direction(&self, d: Direction) -> Option<Pos> {
        match d {
            Direction::Up => if self.y == 0 {
                None
            }
            else {
                Some(Pos::new(self.x, self.y - 1))
            },
            Direction::Down => Some(Pos::new(self.x, self.y + 1)),
            Direction::Left => if self.x == 0 {
                None
            }
            else {
                Some(Pos::new(self.x - 1, self.y))
            },
            Direction::Right => Some(Pos::new(self.x + 1, self.y))
        }
    }
}

fn find_start_point(input: &str) -> usize {
    let first_line = input.lines().next().unwrap();
    first_line.find('|').expect("Couldn't find start point on first line")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    fn left(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Left,
            &Direction::Left => Direction::Down,
            &Direction::Down => Direction::Right,
            &Direction::Right => Direction::Up
        }
    }

    fn right(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Right,
            &Direction::Right => Direction::Down,
            &Direction::Down => Direction::Left,
            &Direction::Left => Direction::Up,
        }
    }
}

fn build_maze(src: &str) -> HashMap<Pos, char> {
    let mut m = HashMap::new();

    for (y, line) in src.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            m.insert(Pos::new(x, y), c);
        }
    }

    m
}

#[derive(Clone)]
struct PathFinder {
    maze: HashMap<Pos, char>,
    current_pos: Pos,
    direction: Direction,
}

impl PathFinder {
    fn new(maze: &str) -> PathFinder {
        let start_pos = Pos::new(find_start_point(maze), 0);
        PathFinder {
            maze: build_maze(maze),
            current_pos: start_pos,
            direction: Direction::Down
        }
    }

    fn char_at_pos(&self, p: Pos) -> Option<char> {
        let c = self.maze.get(&p).map(|c| *c);

        #[cfg(test)]
        println!("Char at {:?} is {:?}", p, c);

        c
    }

    fn find_next_pos(&self) -> Option<Pos> {
        self.current_pos.next_in_direction(self.direction)
    }

    fn is_pos_valid(&self, p: Pos) -> bool {
        #[cfg(test)]
        println!("Checking if {:?} is valid", p);

        match self.char_at_pos(p) {
            None => false,
            Some(' ') => false,
            _ => true
        }
    }

    fn move_to_next_pos(&mut self) {
        #[cfg(test)]
        println!("Finding next position. Current position is {:?} with char {:?}", self.current_pos, self.char_at_pos(self.current_pos));

        match self.char_at_pos(self.current_pos) {
            // if we are currently on a +, we need to turn
            Some('+') => self.turn_and_move(),
            _ => {
        #[cfg(test)]
                println!("We do not need to turn here. Determining next position.");
                if let Some(next_pos) = self.find_next_pos() {
        #[cfg(test)]
                    println!("Next position is {:?}", next_pos);
                    if self.is_pos_valid(next_pos) {
        #[cfg(test)]
                        println!("Position is valid. Updating current position.");
                        self.current_pos = next_pos;
                    }
                }
            }
        }
    }

    fn turn_and_move(&mut self) {
        // so we're sitting on a + and we need to figure out which way to go
        // it's going to be either left or right
        let dir_left = self.direction.left();
        let dir_right = self.direction.right();

        let pos_left = self.current_pos.next_in_direction(dir_left);
        if pos_left.is_some() && self.is_pos_valid(pos_left.unwrap()) {
            self.direction = dir_left;
            self.current_pos = pos_left.unwrap();
            return;
        }

        let pos_right = self.current_pos.next_in_direction(dir_right);
        if pos_right.is_some() && self.is_pos_valid(pos_right.unwrap()) {
            self.direction = dir_right;
            self.current_pos = pos_right.unwrap();
            return;
        }

        panic!("Unable to turn and move as neither left nor right positions were valid");
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PathStep {
    Letter(char),
    Step
}

impl Iterator for PathFinder {
    type Item = PathStep;

    fn next(&mut self) -> Option<PathStep> {
        let old_pos = self.current_pos;
        self.move_to_next_pos();

        if old_pos == self.current_pos {
            // end of the path!
            None
        }
        else {
            match self.char_at_pos(self.current_pos) {
                Some('|') | Some('-') | Some('+') => Some(PathStep::Step),
                Some(c) => Some(PathStep::Letter(c)),
                x => panic!("Got unexpected current character {:?} which should not have been seen by the iterator impl", x)
            }
        }
    }
}

fn run_maze(maze: &str) -> (String, usize) {
    let i = PathFinder::new(maze);
    let steps = i.collect::<Vec<_>>();

    let letters = steps.iter().filter_map(|s| match s { &PathStep::Step => None, &PathStep::Letter(c) => Some(c) }).collect::<String>();
    let steps = steps.len();

    (letters, steps + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    static SAMPLE_INPUT: &'static str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+ 
";

    #[test]
    fn can_find_start_point_in_sample() {
        assert_eq!(find_start_point(SAMPLE_INPUT), 5);
    }

    #[test]
    fn can_start_moving_in_sample() {
        let mut i = PathFinder::new(SAMPLE_INPUT);
        assert_eq!(i.next(), Some(PathStep::Step));
        assert_eq!(i.next(), Some(PathStep::Letter('A')));
    }

    #[test]
    fn can_get_character_at_position() {
        let i = PathFinder::new(r"abcdefg|
1234567|");
        assert_eq!(i.char_at_pos(Pos::new(0,0)), Some('a'), "character at 0,0");
        assert_eq!(i.char_at_pos(Pos::new(5,0)), Some('f'), "character at 5,0");
        assert_eq!(i.char_at_pos(Pos::new(5,1)), Some('6'), "character at 5,1");
    }

    #[test]
    fn run_sample() {
        let (letters, steps) = run_maze(SAMPLE_INPUT);

        assert_eq!(letters, "ABCDEF");
        assert_eq!(steps, 38);
    }
}