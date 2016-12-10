use super::movet::Move;

pub fn instructions_from_string(input: &str) -> Vec<Move> {
    input.to_string()
        .to_uppercase()
        .chars()
        .filter_map(|c| match c {
            'U' => Some(Move::Up),
            'D' => Some(Move::Down),
            'L' => Some(Move::Left),
            'R' => Some(Move::Right),
            _ => None,
        })
        .collect()
}

#[test]
fn test_instructions_from_string() {
    let string = "hello\n world";

    assert_eq!(instructions_from_string(string),
               [Move::Left, Move::Left, Move::Right, Move::Left, Move::Down]);
}
