use super::instruction::Instruction;
use std::str::FromStr;
use pest::Parser;
use pest;

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("grammar.pest");

#[derive(Parser)]
#[grammar = "day7/grammar.pest"]
struct InstructionParser;

pub fn parse_line(line: &str) -> Option<Instruction> {
    fn next_as_string<R: pest::RuleType, I: pest::inputs::Input>(line: &mut pest::iterators::Pairs<R, I>) -> String {
        line.next().unwrap().as_str().trim().to_owned()
    }

    fn next_as_u16<R: pest::RuleType, I: pest::inputs::Input>(line: &mut pest::iterators::Pairs<R, I>) -> u16 {
        u16::from_str(line.next().unwrap().as_str().trim()).expect("Value should be a u16")
    }

    let pairs = InstructionParser::parse_str(Rule::instruction, line)
        .unwrap_or_else(|e| panic!("{}", e));

    for pair in pairs {
        for inner_pair in pair.into_inner() {
            match inner_pair.as_rule() {
                Rule::signal => {
                    let mut inner = inner_pair.into_inner();

                    let value = next_as_u16(&mut inner);
                    let target = next_as_string(&mut inner);

                    let instruction = Instruction::Signal { value, target };
                    return Some(instruction)
                }
                Rule::and => {
                    let mut inner = inner_pair.into_inner();

                    let a = next_as_string(&mut inner);
                    let b = next_as_string(&mut inner);
                    let target = next_as_string(&mut inner);

                    let instruction = Instruction::And { a, b, target };
                    return Some(instruction)
                }
                Rule::or =>  {
                    let mut inner = inner_pair.into_inner();

                    let a = next_as_string(&mut inner);
                    let b = next_as_string(&mut inner);
                    let target = next_as_string(&mut inner);

                    let instruction = Instruction::Or { a, b, target };
                    return Some(instruction)
                }
                Rule::lshift =>  {
                    let mut inner = inner_pair.into_inner();

                    let input = next_as_string(&mut inner);
                    let distance = next_as_u16(&mut inner);
                    let target = next_as_string(&mut inner);

                    let distance = (distance as i32) * -1;

                    let instruction = Instruction::Shift { input, distance, target };
                    return Some(instruction)
                }
                Rule::rshift => {
                    let mut inner = inner_pair.into_inner();

                    let input = next_as_string(&mut inner);
                    let distance = next_as_u16(&mut inner);
                    let target = next_as_string(&mut inner);

                    let distance = distance as i32;

                    let instruction = Instruction::Shift { input, distance, target };
                    return Some(instruction)
                }
                Rule::not =>  {
                    let mut inner = inner_pair.into_inner();

                    let input = next_as_string(&mut inner);
                    let target = next_as_string(&mut inner);

                    let instruction = Instruction::Not { input, target };
                    return Some(instruction)
                }
                _ => return None
            };
        }
    }

    None
}

#[test]
fn test_parse() {
    let input = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    let mut instructions = vec![];

    for line in input.lines() {
        instructions.push(parse_line(line));
    }

    let expected = vec![
        Some(Instruction::Signal { value: 123u16, target: "x".to_owned() }),
        Some(Instruction::Signal { value: 456u16, target: "y".to_owned() }),
        Some(Instruction::And { a: "x".to_owned(), b: "y".to_owned(), target: "d".to_owned() }),
        Some(Instruction::Or { a: "x".to_owned(), b: "y".to_owned(), target: "e".to_owned() }),
        Some(Instruction::Shift { input: "x".to_owned(), distance: -2, target: "f".to_owned() }),
        Some(Instruction::Shift { input: "y".to_owned(), distance: 2, target: "g".to_owned() }),
        Some(Instruction::Not { input: "x".to_owned(), target: "h".to_owned() }),
        Some(Instruction::Not { input: "y".to_owned(), target: "i".to_owned() })
    ];

    assert_eq!(instructions, expected);
}