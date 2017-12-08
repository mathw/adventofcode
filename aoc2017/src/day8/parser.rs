use super::ast;
use regex;
use std::collections::HashSet;
use std::str::FromStr;

pub fn parse_program<'a>(src: &'a str) -> Result<ast::Program<'a>, &'static str> {
    let parsed_instructions = src.lines().map(|line| parse_instruction(line));

    let mut registers = HashSet::new();
    let mut instructions = Vec::new();

    for x in parsed_instructions {
        match x {
            Ok((i, r)) => {
                for register in r {
                    registers.insert(register);
                }

                instructions.push(i);
            }
            Err(e) => return Err(e),
        }
    }

    Ok(ast::Program::new(instructions, registers))
}

fn parse_instruction<'a>(src: &'a str)
                         -> Result<(ast::Instruction<'a>, HashSet<&'a str>), &'static str> {
    lazy_static! {
        static ref RE: regex::Regex =
            regex::Regex::new(r"(\w+) (\w+) (-?\d+) if (\w+) (\S+) (-?\d+)").unwrap();
    }

    let m = RE.captures(src);

    match m {
        Some(m) => make_instruction(m),
        None => Err("Unable to match regex"),
    }
}

fn make_instruction<'a>(m: regex::Captures<'a>)
                        -> Result<(ast::Instruction<'a>, HashSet<&'a str>), &'static str> {
    let action_register = m.get(1).unwrap().as_str();
    let action_action_str = &m.get(2).unwrap().as_str();
    let action_value_str = &m.get(3).unwrap().as_str();
    let cond_register = m.get(4).unwrap().as_str();
    let cond_op_str = &m.get(5).unwrap().as_str();
    let cond_value_str = &m.get(6).unwrap().as_str();

    let mut registers = HashSet::new();
    registers.insert(action_register);
    registers.insert(cond_register);

    let action_op = match *action_action_str {
        "inc" => ast::ActionOp::Inc,
        "dec" => ast::ActionOp::Dec,
        _ => return Err("Unknown action op"),
    };
    let action_value = i32::from_str(action_value_str).map_err(|_| "Unable to parse action value")?;
    let action = ast::Action::new(action_register, action_op, action_value);

    let cond_op = match *cond_op_str {
        "==" => ast::Operator::EqualTo,
        "!=" => ast::Operator::NotEqualTo,
        ">=" => ast::Operator::GreaterThanOrEqualTo,
        "<=" => ast::Operator::LessThanOrEqualTo,
        ">" => ast::Operator::GreaterThan,
        "<" => ast::Operator::LessThan,
        _ => return Err("Unknown condition op"),
    };
    let cond_value = i32::from_str(cond_value_str).map_err(|_| "Unable to parse condition value")?;
    let condition = ast::Condition::new(cond_register, cond_op, cond_value);

    Ok((ast::Instruction::new(condition, action), registers))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction_1() {
        match parse_instruction("ben inc 360 if sp >= -4777") {
            Ok((instruction, registers)) => {
                assert_eq!(instruction,
                           ast::Instruction::new(ast::Condition::new("sp",
                                          ast::Operator::GreaterThanOrEqualTo,
                                                                     -4777),
                                                 ast::Action::new("ben", ast::ActionOp::Inc, 360)));
                assert_eq!(registers.len(), 2);
                assert!(registers.contains("ben"));
                assert!(registers.contains("sp"));
            }
            _ => assert!(false, "Valid instruction must parse"),
        }
    }

    #[test]
    fn parse_instruction_2() {
        match parse_instruction("jzm dec 594 if bwj != 1443") {
            Ok((instruction, registers)) => {
                assert_eq!(instruction,
                           ast::Instruction::new(ast::Condition::new("bwj",
                                                                     ast::Operator::NotEqualTo,
                                                                     1443),
                                                 ast::Action::new("jzm", ast::ActionOp::Dec, 594)));
                assert_eq!(registers.len(), 2);
                assert!(registers.contains("jzm"));
                assert!(registers.contains("bwj"));
            }
            _ => assert!(false, "Valid instruction must parse"),
        }
    }

    #[test]
    fn parse_program_1() {
        let mut registers = HashSet::new();
        registers.insert("jzm");
        registers.insert("bwj");
        registers.insert("sp");
        match parse_program("jzm dec 594 if bwj != 1443\nbwj inc 360 if sp >= -4777") {
            Ok(program) => {
                assert_eq!(program,
                // so rustfmt just gives up here...
            ast::Program::new(vec![ast::Instruction::new(ast::Condition::new("bwj",
                                                                     ast::Operator::NotEqualTo,
                                                                     1443),
                                                 ast::Action::new("jzm", ast::ActionOp::Dec, 594)),
                                                 ast::Instruction::new(ast::Condition::new("sp",
                                          ast::Operator::GreaterThanOrEqualTo,
                                                                     -4777),
                                                 ast::Action::new("bwj", ast::ActionOp::Inc, 360))],
                                                 registers))
            }
            _ => assert!(false, "Valid program must parse"),
        }
    }
}
