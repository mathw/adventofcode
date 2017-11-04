#[derive(Debug, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
}

fn direction_from_char(c: char) -> Option<Direction> {
    match c {
        '(' => Some(Direction::Up),
        ')' => Some(Direction::Down),
        _ => None,
    }
}

fn parse_directions(input: &str) -> Vec<Direction> {
    input.to_string().chars().filter_map(|c| direction_from_char(c)).collect()
}

fn direction_to_floor_change(d: &Direction) -> i32 {
    match d {
        &Direction::Up => 1,
        &Direction::Down => -1,
    }
}

fn follow_directions<I>(starting_floor: i32, directions: I) -> i32
    where I: IntoIterator<Item = Direction>
{
    directions.into_iter()
        .map(|d| direction_to_floor_change(&d))
        .fold(starting_floor, |c, x| c + x)
}

fn find_first_in_basement<I>(starting_floor: i32, directions: I) -> Option<usize>
    where I: IntoIterator<Item = Direction>
{
    directions.into_iter()
        .map(|d| direction_to_floor_change(&d))
        .scan(starting_floor, |state: &mut i32, x| {
            *state = *state + x;
            Some(*state)
        })
        .position(|x| x < 0)
        .map(|x| x + 1)
}

pub fn do_dayone() {
    let input = include_str!("input.txt");

    let directions = parse_directions(input);

    let floor = follow_directions(0, directions.clone());
    let first_basement = find_first_in_basement(0, directions);

    println!("The floor Santa needs is {} and he first enters the basement at {}",
             floor,
             match first_basement {
                 Some(x) => x,
                 None => 0,
             });
}

#[test]
fn test_parse() {
    assert_eq!(parse_directions("))(("),
               [Direction::Down, Direction::Down, Direction::Up, Direction::Up]);
    assert_eq!(parse_directions(")a)( ("),
               [Direction::Down, Direction::Down, Direction::Up, Direction::Up]);
}

#[test]
fn test_follow_directions() {
    assert_eq!(follow_directions(0, vec![Direction::Down]), -1);
    assert_eq!(follow_directions(0, vec![Direction::Up]), 1);
    assert_eq!(follow_directions(0,
                                 vec![Direction::Up,
                                      Direction::Up,
                                      Direction::Down,
                                      Direction::Up]),
               2);
}
