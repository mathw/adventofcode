mod instruction;
mod parser;
mod evaluate;

use self::evaluate::evaluate;
use self::parser::parse_line;

pub fn run() {
    let input = include_str!("input.txt");

    let instructions = input.lines().map(|line| parse_line(line).unwrap()).collect::<Vec<_>>();

    let final_states = evaluate(&instructions);

    println!("Wire a has value {}", final_states.get("a").unwrap());
}


#[test]
fn test_evaluate() {
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    let instructions = input.lines().map(|line| parse_line(line).unwrap()).collect::<Vec<_>>();

    let states = evaluate(&instructions);

    assert_eq!(states.get("d"), Some(&72));
    assert_eq!(states.get("e"), Some(&507));
    assert_eq!(states.get("f"), Some(&492));
    assert_eq!(states.get("g"), Some(&114));
    assert_eq!(states.get("h"), Some(&65412));
    assert_eq!(states.get("i"), Some(&65079));
    assert_eq!(states.get("x"), Some(&123));
    assert_eq!(states.get("y"), Some(&456));
}