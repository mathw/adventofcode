use std::collections::HashMap;

pub fn run() {
    let mut state = State::new();

    let directions = read_input();

    for dir in directions.clone() {
        state.apply_move(dir);
    }

    let more_than_one = state.grid.values().filter(|&v| v > &0).count();

    println!("{} houses have more than one present", more_than_one);

    // part two

    let mut santa_state = State::new();
    let mut robo_state = State::new();

    let pairs = directions.chunks(2).collect::<Vec<_>>();

    let santa_directions = pairs.iter().map(|x| x[0].clone()).collect::<Vec<_>>();
    let robo_directions = pairs.iter().map(|x| x[1].clone()).collect::<Vec<_>>();

    for dir in santa_directions {
        santa_state.apply_move(dir);
    }

    for dir in robo_directions {
        robo_state.apply_move(dir);
    }

    let combined = combine_grids(&santa_state.grid, &robo_state.grid);

    let more_than_one = combined.values().filter(|&v| v > &0).count();

    println!("{} houses have more than one present", more_than_one);
}

fn combine_grids(grid1: &HashMap<Pos, u32>, grid2: &HashMap<Pos, u32>) -> HashMap<Pos, u32> {
    let mut target = HashMap::new();

    fn update_target_from(target: &mut HashMap<Pos, u32>, grid: &HashMap<Pos, u32>) {
        for kv in grid.into_iter() {
            let handle = target.entry(*kv.0).or_insert(0);
            *handle += *kv.1;
        }
    }

    update_target_from(&mut target, grid1);
    update_target_from(&mut target, grid2);

    target
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new() -> Pos {
        Pos { x: 0, y: 0 }
    }

    fn apply_move(&self, dir: Direction) -> Pos {
        match dir {
            Direction::Up => Pos { y: self.y + 1, ..*self },
            Direction::Down => Pos { y: self.y - 1, ..*self },
            Direction::Right => Pos { x: self.x + 1, ..*self },
            Direction::Left => Pos { x: self.x - 1, ..*self },
        }
    }
}

#[derive(Clone)]
struct State {
    pos: Pos,
    grid: HashMap<Pos, u32>,
}

impl State {
    fn new() -> State {
        let pos = Pos::new();

        let mut state = State {
            pos: pos,
            grid: HashMap::new(),
        };

        state.add_present(pos);

        state
    }

    fn add_present(&mut self, add_pos: Pos) {
        let handle = self.grid.entry(add_pos).or_insert(0);
        *handle += 1;
    }

    fn apply_move(&mut self, dir: Direction) {
        let new_pos = self.pos.apply_move(dir);
        self.pos = new_pos;
        self.add_present(new_pos);
    }
}

fn read_input() -> Vec<Direction> {
    let input = include_str!("input.txt");

    parse_input(input)
}

fn parse_input(input: &str) -> Vec<Direction> {
    fn parse_direction(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Up),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '<' => Some(Direction::Left),
            _ => None,
        }
    }

    input.chars().map(parse_direction).filter(|x| x.is_some()).map(|x| x.unwrap()).collect()
}


#[test]
fn parse_input_empty_string_is_empty() {
    let input = "";
    let result = parse_input(input);
    assert_eq!(result, vec![]);
}

#[test]
fn parse_input_gets_all_chars() {
    let input = "^^<>v>";
    let result = parse_input(input);
    assert_eq!(result,
               vec![Direction::Up,
                    Direction::Up,
                    Direction::Left,
                    Direction::Right,
                    Direction::Down,
                    Direction::Right]);
}
