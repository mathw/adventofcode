#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn instructions_from_string(input: &str) -> Vec<Move> {
    input.to_string()
        .to_uppercase()
        .chars()
        .filter(|c| match c {
            &'U' => true,
            &'D' => true,
            &'L' => true,
            &'R' => true,
            _ => false,
        })
        .map(|c| match c {
            'U' => Move::Up,
            'D' => Move::Down,
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Filter was wrong"),
        })
        .collect()
}

#[test]
fn test_letters_from_string() {
    let string = "hello world";

    assert_eq!(instructions_from_string(string),
               [Move::Left, Move::Left, Move::Right, Move::Left, Move::Down]);
}
